use crate::queue::EventQueue;

#[test]
fn test_queue() {
    #[derive(PartialEq, Debug)]
    enum TestEvent {
        Event1 { a: i32 },
        Event2 { b: i32 },
    }

    let mut queue = EventQueue::new(10);
    queue.push(TestEvent::Event1 { a: 42 });
    queue.push(TestEvent::Event2 { b: 43 });

    match queue.pop() {
        Some(TestEvent::Event1 { a }) => assert_eq!(a, 42),
        _ => panic!("Expected Event1"),
    }

    match queue.pop() {
        Some(TestEvent::Event2 { b }) => assert_eq!(b, 43),
        _ => panic!("Expected Event2"),
    }

    assert_eq!(queue.pop(), None);
}
