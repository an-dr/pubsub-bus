struct TestEvent {
    a: i32,
}

#[test]
fn test_event() {
    use super::Event;

    let mut event = Event::new(TestEvent { a: 42 });

    assert_eq!(event.get_content().a, 42);

    let content = event.get_mut_content();
    content.a = 43;
    assert_eq!(event.get_content().a, 43);

    event.set_header(1, 2);
    assert_eq!(event.get_id(), 1);
    assert_eq!(event.get_source_id(), 2);
}
