use crate::BusEvent;

struct TestEvent {
    a: i32,
}

#[test]
fn test_event() {
    let mut event1 = BusEvent::new(1, 2, Some(1), TestEvent { a: 42 });

    assert_eq!(event1.get_content().a, 42);

    let content = event1.get_mut_content();
    content.a = 43;
    assert_eq!(event1.get_content().a, 43);

    assert_eq!(event1.get_id(), 1);
    assert_eq!(event1.get_source_id(), 2);

    let event2: BusEvent<TestEvent, u32> = BusEvent::new(2, 3, None, TestEvent { a: 24 });
    assert_eq!(event2.get_content().a, 24);

    assert_eq!(*event2.get_topic_id(), None);
    assert_eq!(event2.get_content().a, 24);

    assert_eq!(event2.get_id(), 2);
    assert_eq!(event2.get_source_id(), 3);
}
