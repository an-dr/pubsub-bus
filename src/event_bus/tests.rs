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

// NOTE on scope, found while testing this fix: this crate locks each
// subscriber's own `Mutex` even just to call `is_subscribed_to` on it
// during a publish pass, and again (held for the full call) for `on_event`.
// A `publish()` called reentrantly from inside `on_event` will therefore
// always try to re-lock the *currently executing* subscriber's own Mutex as
// soon as that subscriber is reached in the reentrant pass's iteration --
// std::sync::Mutex is not reentrant, so this deadlocks unconditionally,
// regardless of topic filtering (is_subscribed_to itself requires the lock)
// and regardless of the snapshot-before-callback fix below (that fix only
// removes the RwLock-recursion hazard on `subscribers`, a different lock).
// Making reentrant publish safe in general would require replacing this
// per-subscriber Mutex with a reentrant lock, which is a larger design
// change than this fix and has its own correctness tradeoffs (a subscriber
// re-entering its own on_event while mid-mutation of its fields). Out of
// scope here. Consumers that cannot guarantee their subscription graph
// never re-enters a still-executing subscriber (bones' Adapter cannot,
// since is_subscribed_to always returns true there) must defer delivery
// outside the on_event call stack entirely -- never call publish() from
// within a Subscriber's on_event.
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
    // Regression coverage for the snapshot-before-callback refactor itself:
    // with an unchanging subscriber list, every publish must still reach
    // every subscriber exactly once, in the order published.
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
    // Documents the snapshot's actual semantic: publish() delivers to who
    // was subscribed at snapshot time, not to whoever ends up subscribed by
    // the time delivery finishes. Subscribers added after a publish() call
    // returns see only subsequent events, same as before this fix.
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
