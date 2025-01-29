use super::Publisher;
use crate::{event::IntoEvent, Event, EventBus, Subscriber};
use shared_type::{IntoShared, Shared};

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

struct TestPublisher {
    publisher_value: i32,
    publisher: Publisher<TestEvent>,
}

impl TestPublisher {
    pub fn new(bus: Shared<EventBus<TestEvent>>, value: i32) -> Self {
        let mut publisher = Publisher::new();
        publisher.set_bus(bus);
        Self {
            publisher_value: value,
            publisher,
        }
    }

    pub fn publish_to(&self, destination: u64) {
        let event = TestEvent {
            destination,
            value: self.publisher_value,
        }
        .into_event();

        self.publisher.publish(&event);
    }
}

#[test]
fn test_bus() {
    // Create a bus and subscribers
    let bus = EventBus::new().into_shared();

    let subscriber1 = TestSubscriber { id: 1 }.into_shared();
    let subscriber2 = TestSubscriber { id: 2 }.into_shared();

    bus.lock().unwrap().subscribe(subscriber1);
    bus.lock().unwrap().subscribe(subscriber2);

    let publisher1 = TestPublisher::new(bus.clone(), 42);
    let publisher2 = TestPublisher::new(bus.clone(), 24);

    publisher1.publish_to(0);
    publisher1.publish_to(1);
    publisher1.publish_to(2);
    publisher1.publish_to(3);

    publisher2.publish_to(0);
    publisher2.publish_to(1);
    publisher2.publish_to(2);
    publisher2.publish_to(3);
}
