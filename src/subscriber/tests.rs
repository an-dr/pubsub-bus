use crate::event::{Event, IntoEvent};
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
    let event = 42.into_event();

    subscriber.on_event(&event);
    assert_eq!(subscriber.attribute, 42);
}
