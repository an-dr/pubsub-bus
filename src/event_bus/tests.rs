use crate::{BusEvent, EventBus, Subscriber};
use std::sync::{Arc, Mutex};

struct TestEvent {
    destination: u64,
    value: i32,
}

struct TestSubscriber {
    id: u64,
}

impl Subscriber<TestEvent, u32> for TestSubscriber {
    fn on_event(&mut self, event: &BusEvent<TestEvent, u32>) {
        let content = event.get_content();
        if content.destination != self.id {
            return;
        }
        println!("Received event with content: {}", content.value);
    }
}

#[test]
fn test_bus() {
    // Create a bus and subscribers
    let bus = EventBus::new();

    let subscriber1 = Arc::new(Mutex::new(TestSubscriber { id: 1 }));
    let subscriber2 = Arc::new(Mutex::new(TestSubscriber { id: 2 }));
    let subscriber4 = TestSubscriber { id: 4 };

    bus.add_subscriber_shared(subscriber1);
    bus.add_subscriber_shared(subscriber2);
    bus.add_subscriber(subscriber4);
    

    // Create and publish events
    let event42 = TestEvent {
        destination: 1,
        value: 42,
    };
    let event24 = TestEvent {
        destination: 2,
        value: 24,
    };
    let event64 = TestEvent {
        destination: 3,
        value: 64,
    };
    
    let event84 = TestEvent {
        destination: 4,
        value: 84,
    };

    bus.publish(event42, None, 0);
    bus.publish(event24, None, 0);
    bus.publish(event64, None, 0);
    bus.publish(event84, None, 0);
}

struct CountingSubscriber {
    count: Arc<Mutex<u32>>,
}

impl Subscriber<TestEvent, u32> for CountingSubscriber {
    fn on_event(&mut self, _event: &BusEvent<TestEvent, u32>) {
        *self.count.lock().unwrap() += 1;
    }
}

#[test]
fn removed_subscriber_stops_receiving_events() {
    let bus = EventBus::new();
    let count = Arc::new(Mutex::new(0));
    let subscriber: Arc<Mutex<dyn Subscriber<TestEvent, u32>>> = Arc::new(Mutex::new(CountingSubscriber {
        count: count.clone(),
    }));
    bus.add_subscriber_shared(subscriber.clone());

    bus.publish(TestEvent { destination: 1, value: 1 }, None, 0);
    assert_eq!(*count.lock().unwrap(), 1);

    bus.remove_subscriber_shared(&subscriber);
    bus.publish(TestEvent { destination: 1, value: 2 }, None, 0);
    assert_eq!(*count.lock().unwrap(), 1, "removed subscriber must not be delivered to");
}

#[test]
fn remove_subscriber_not_present_is_a_no_op() {
    let bus = EventBus::new();
    let subscriber: Arc<Mutex<dyn Subscriber<TestEvent, u32>>> = Arc::new(Mutex::new(CountingSubscriber {
        count: Arc::new(Mutex::new(0)),
    }));
    // never added; must not panic
    bus.remove_subscriber_shared(&subscriber);
}

// Reentrant publish() still deadlocks unconditionally on the currently-
// executing subscriber's own Mutex (locked for is_subscribed_to too, not
// just on_event) -- topic filtering doesn't help. Use enqueue()/dispatch()
// for reactive publishing instead; see that regression test below.
struct CountingSubscriber2 {
    count: Arc<Mutex<u32>>,
}

impl Subscriber<TestEvent, u32> for CountingSubscriber2 {
    fn on_event(&mut self, _event: &BusEvent<TestEvent, u32>) {
        *self.count.lock().unwrap() += 1;
    }
}

#[test]
fn snapshotting_subscribers_does_not_change_delivery_to_a_stable_list() {
    let bus = EventBus::new();
    let count = Arc::new(Mutex::new(0));
    let subscriber = Arc::new(Mutex::new(CountingSubscriber2 {
        count: count.clone(),
    }));
    bus.add_subscriber_shared(subscriber);

    for i in 0..5 {
        bus.publish(TestEvent { destination: 1, value: i }, None, 0);
    }

    assert_eq!(*count.lock().unwrap(), 5);
}

#[test]
fn a_subscriber_added_during_the_snapshot_window_is_not_notified_of_the_in_flight_event() {
    let bus = EventBus::new();
    let count = Arc::new(Mutex::new(0));

    bus.publish(TestEvent { destination: 1, value: 1 }, None, 0);

    let subscriber = Arc::new(Mutex::new(CountingSubscriber2 {
        count: count.clone(),
    }));
    bus.add_subscriber_shared(subscriber);

    assert_eq!(*count.lock().unwrap(), 0);

    bus.publish(TestEvent { destination: 1, value: 2 }, None, 0);
    assert_eq!(*count.lock().unwrap(), 1);
}

struct RecordingValueSubscriber {
    values: Arc<Mutex<Vec<i32>>>,
}

impl Subscriber<TestEvent, u32> for RecordingValueSubscriber {
    fn on_event(&mut self, event: &BusEvent<TestEvent, u32>) {
        self.values.lock().unwrap().push(event.get_content().value);
    }
}

#[test]
fn enqueue_without_dispatch_does_not_deliver() {
    let bus = EventBus::new();
    let values = Arc::new(Mutex::new(Vec::new()));
    let subscriber = Arc::new(Mutex::new(RecordingValueSubscriber {
        values: values.clone(),
    }));
    bus.add_subscriber_shared(subscriber);

    bus.enqueue(TestEvent { destination: 1, value: 1 }, None, 0);

    assert!(values.lock().unwrap().is_empty());
}

#[test]
fn dispatch_delivers_queued_events_in_order() {
    let bus = EventBus::new();
    let values = Arc::new(Mutex::new(Vec::new()));
    let subscriber = Arc::new(Mutex::new(RecordingValueSubscriber {
        values: values.clone(),
    }));
    bus.add_subscriber_shared(subscriber);

    bus.enqueue(TestEvent { destination: 1, value: 1 }, None, 0);
    bus.enqueue(TestEvent { destination: 1, value: 2 }, None, 0);
    bus.enqueue(TestEvent { destination: 1, value: 3 }, None, 0);
    bus.dispatch();

    assert_eq!(*values.lock().unwrap(), vec![1, 2, 3]);
}

// enqueue() touches only the outbox, never a subscriber lock, so unlike
// publish() this is always safe to call reentrantly from on_event.
struct ReactiveEnqueueSubscriber {
    internal: Arc<crate::event_bus_internal::EventBusInternal<TestEvent, u32>>,
    already_fired: bool,
}

impl Subscriber<TestEvent, u32> for ReactiveEnqueueSubscriber {
    fn on_event(&mut self, _event: &BusEvent<TestEvent, u32>) {
        if !self.already_fired {
            self.already_fired = true;
            self.internal
                .enqueue(TestEvent { destination: 1, value: 99 }, None, 0);
        }
    }
}

#[test]
fn reactive_enqueue_from_on_event_does_not_deadlock_and_waits_for_next_dispatch() {
    let bus: EventBus<TestEvent, u32> = EventBus::new();
    let values = Arc::new(Mutex::new(Vec::new()));

    let reactive = Arc::new(Mutex::new(ReactiveEnqueueSubscriber {
        internal: bus.get_internal(),
        already_fired: false,
    }));
    let recorder = Arc::new(Mutex::new(RecordingValueSubscriber {
        values: values.clone(),
    }));
    bus.add_subscriber_shared(reactive);
    bus.add_subscriber_shared(recorder);

    bus.enqueue(TestEvent { destination: 1, value: 1 }, None, 0);
    bus.dispatch();
    assert_eq!(
        *values.lock().unwrap(),
        vec![1],
        "reactively-enqueued event must not appear within the same dispatch"
    );

    bus.dispatch();
    assert_eq!(*values.lock().unwrap(), vec![1, 99]);
}
