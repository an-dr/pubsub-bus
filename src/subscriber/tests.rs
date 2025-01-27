use crate::event::Event;
use crate::subscriber::Subscriber;

struct TestSubscriber {
    attribute: i32,
}

impl Subscriber<i32> for TestSubscriber {
    fn on_event(&mut self, event: &Event<i32>) {
        self.attribute = *event.get_content();
    }
}

#[test]
fn test_subscriber() {
    let mut subscriber = TestSubscriber { attribute: 0 };
    let event = Event::new(42);

    subscriber.on_event(&event);
    assert_eq!(subscriber.attribute, 42);
}
