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
