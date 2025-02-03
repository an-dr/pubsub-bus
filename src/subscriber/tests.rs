use shared_type::{IntoShared, Shared};

use crate::event::{Event};
use crate::subscriber::Subscriber;
use crate::{EventBus, Publisher};

struct TestSubscriber {
    attribute: i32,
}

impl Subscriber<i32> for TestSubscriber {
    fn on_event(&mut self, event: &Event<i32>) {
        let id = event.get_id();
        println!("Received event with id: {} and content: {}", id, event.get_content());
        
        self.attribute = *event.get_content();
    }
}

struct TestPublisher {
    pub publisher: Publisher<i32>,
}

impl TestPublisher {

    pub fn publish(&self, val: i32) {
        self.publisher.publish(val);
    }
}

#[test]
fn test_subscriber() {
    let bus: Shared<EventBus<i32>> = EventBus::new().into_shared();
    
    let publisher = TestPublisher{ publisher: Publisher::new(bus.clone()) };
    let subscriber = TestSubscriber { attribute: 0 }.into_shared();
    
    bus.lock().unwrap().subscribe(subscriber.clone());

    publisher.publish(42);
    assert_eq!(subscriber.lock().unwrap().attribute, 42);
    publisher.publish(24);
    assert_eq!(subscriber.lock().unwrap().attribute, 24);
}
