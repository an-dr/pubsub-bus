use crate::{BusEvent, EventBus, EventEmitter, Publisher, Subscriber};
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
            println!(
                "Subscriber {} ignoring event with content: {}",
                self.id, content.value
            );
            return;
        }
        println!(
            "Subscriber {} received event with content: {}",
            self.id, content.value
        );
    }
}

struct TestPublisher {
    publisher_value: i32,
    pub emitter: EventEmitter<TestEvent, u32>,
}

impl TestPublisher {
    pub fn new(value: i32) -> Self {
        let publisher = EventEmitter::new();
        Self {
            publisher_value: value,
            emitter: publisher,
        }
    }

    pub fn publish_to(&mut self, destination: u64) {
        let event = TestEvent {
            destination,
            value: self.publisher_value,
        };

        self.emitter.publish(event, None);
    }
}

impl Publisher<TestEvent, u32> for TestPublisher {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<TestEvent, u32> {
        &mut self.emitter
    }
}

#[test]
fn test_bus() {
    // Create a bus and subscribers
    let bus = EventBus::new();

    let subscriber1 = Arc::new(Mutex::new(TestSubscriber { id: 1 }));
    let subscriber2 = Arc::new(Mutex::new(TestSubscriber { id: 2 }));

    let mut publisher1 = TestPublisher::new(42);
    let mut publisher2 = TestPublisher::new(24);

    bus.add_subscriber(subscriber1);
    bus.add_subscriber(subscriber2);
    bus.add_publisher(&mut publisher1);
    bus.add_publisher(&mut publisher2);

    publisher1.publish_to(0);
    publisher1.publish_to(1);
    publisher1.publish_to(2);
    publisher1.publish_to(3);

    publisher2.publish_to(0);
    publisher2.publish_to(1);
    publisher2.publish_to(2);
    publisher2.publish_to(3);
}
