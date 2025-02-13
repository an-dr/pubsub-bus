use crate::event::Event;
use crate::subscriber::Subscriber;
use crate::{EventBus, EventEmitter, Publisher};
use std::sync::{Arc, Mutex};

struct TestSubscriber {
    attribute: i32,
}

impl Subscriber<i32> for TestSubscriber {
    fn on_event(&mut self, event: &Event<i32>) {
        let id = event.get_id();
        println!(
            "Received event with id: {} and content: {}",
            id,
            event.get_content()
        );

        self.attribute = *event.get_content();
    }
}

struct TestPublisher {
    pub publisher: EventEmitter<i32>,
}

impl TestPublisher {
    pub fn publish(&mut self, val: i32) {
        self.publisher.publish(val, None);
    }
}

impl Publisher<i32> for TestPublisher {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<i32> {
        &mut self.publisher
    }
}

#[test]
fn test_subscriber() {
    let bus: Arc<EventBus<i32>> = Arc::new(EventBus::new());

    let mut publisher = TestPublisher {
        publisher: EventEmitter::new(),
    };
    let subscriber = Arc::new(Mutex::new(TestSubscriber { attribute: 0 }));

    bus.add_subscriber(subscriber.clone());
    bus.add_publisher(&mut publisher);

    publisher.publish(42);
    assert_eq!(subscriber.lock().unwrap().attribute, 42);
    publisher.publish(24);
    assert_eq!(subscriber.lock().unwrap().attribute, 24);
}
