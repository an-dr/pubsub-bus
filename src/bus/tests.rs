use crate::shared::IntoShared;

use super::{Event, EventBus, Subscriber};

struct TestEvent {
    destination: u64,
    value: i32,
}

struct TestSubscriber {
    id: u64,
    events: Vec<TestEvent>,
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
    let mut bus = EventBus::new();

    let subscriber1 = TestSubscriber {
        id: 1,
        events: Vec::new(),
    }
    .into_shared();
    let subscriber2 = TestSubscriber {
        id: 2,
        events: Vec::new(),
    }
    .into_shared();

    // bus.subscribe(Box::new(subscriber1)); TODO consider
    bus.subscribe(subscriber1);
    bus.subscribe(subscriber2);

    // Create and publish events
    let event42 = Event::new(TestEvent {
        destination: 1,
        value: 42,
    });
    let event24 = Event::new(TestEvent {
        destination: 2,
        value: 24,
    });

    bus.publish(&event42);
    bus.publish(&event24);
}
