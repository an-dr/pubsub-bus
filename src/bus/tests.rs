use crate::{Event, EventBus, IntoEvent, Subscriber};
use std::sync::{Arc, Mutex};

struct TestEvent {
    destination: u64,
    value: i32,
}

struct TestSubscriber {
    id: u64,
}

impl Subscriber<TestEvent> for TestSubscriber {
    fn on_event(&mut self, event: &Event<TestEvent>) {
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

    bus.add_subscriber(subscriber1);
    bus.add_subscriber(subscriber2);

    // Create and publish events
    let event42 = TestEvent {
        destination: 1,
        value: 42,
    };
    let event24 = TestEvent {
        destination: 2,
        value: 24,
    };

    bus.publish(&mut event42.into_event());
    bus.publish(&mut event24.into_event());
}
