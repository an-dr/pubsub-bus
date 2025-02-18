use crate::subscriber::Subscriber;
use crate::{BusEvent, EventBus};
use std::sync::{Arc, Mutex};

struct TestSubscriber {
    attribute: i32,
}

impl Subscriber<i32, u32> for TestSubscriber {
    fn on_event(&mut self, event: &BusEvent<i32, u32>) {
        let id = event.get_id();
        println!(
            "Received event with id: {} and content: {}",
            id,
            event.get_content()
        );

        self.attribute = *event.get_content();
    }
    
    fn get_subscribed_topics(&self) -> Option<Vec<u32>> {
        Some(vec![42])
    }
}

#[test]
fn test_subscriber() {
    let bus: EventBus<i32, u32> = EventBus::new();

    let subscriber = Arc::new(Mutex::new(TestSubscriber { attribute: 0 }));
    bus.add_subscriber_shared(subscriber.clone());

    bus.publish(42, Some(42), 0);
    assert_eq!(subscriber.lock().unwrap().attribute, 42);
    
    // The subscriber is not subscribed to this topic
    bus.publish(24, Some(24), 1);
    assert_eq!(subscriber.lock().unwrap().attribute, 42);
}
