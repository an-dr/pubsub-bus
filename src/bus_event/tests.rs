use crate::BusEvent;

struct TestEvent {
    a: i32,
}

#[test]
fn test_event() {
    let mut event1 = BusEvent::new(TestEvent { a: 42 }, Some(1));

    assert_eq!(event1.get_content().a, 42);

    let content = event1.get_mut_content();
    content.a = 43;
    assert_eq!(event1.get_content().a, 43);

    event1.set_header(1, 2);
    assert_eq!(event1.get_id(), 1);
    assert_eq!(event1.get_source_id(), 2);

    let mut event2: BusEvent<TestEvent, u32> = BusEvent::new(TestEvent { a: 24 }, None);
    assert_eq!(event2.get_content().a, 24);

    assert_eq!(*event2.get_topic_id(), None);
    assert_eq!(event2.get_content().a, 24);

    event2.set_header(2, 3);
    assert_eq!(event2.get_id(), 2);
    assert_eq!(event2.get_source_id(), 3);
}
