// *************************************************************************
//
// Copyright (c) 2025 Andrei Gramakov. All rights reserved.
//
// This file is licensed under the terms of the MIT license.
// For a copy, see: https://opensource.org/licenses/MIT
//
// site:    https://agramakov.me
// e-mail:  mail@agramakov.me
//
// *************************************************************************
use crate::{Event, EventBus, EventEmitter, Publisher, Subscriber};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

// global counter
const ITERATIONS: usize = 1000000;
static COUNTER: AtomicUsize = AtomicUsize::new(0);

struct TestEvent {
    source: u64,
    destination: u64,
    value: i32,
}

struct TestSubscriber {
    id: u64,
}

impl Subscriber<TestEvent> for TestSubscriber {
    fn on_event(&mut self, event: &Event<TestEvent>) {
        let content = event.get_content();
        if content.destination != self.id {
            return;
        }
        COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        // println!("Received event with content: {}", content.value);
    }
}

struct TestPublisher {
    publisher_value: i32,
    pub emitter: EventEmitter<TestEvent>,
    self_id: u64,
}

impl TestPublisher {
    pub fn new(value: i32) -> Self {
        let publisher = EventEmitter::new();
        Self {
            publisher_value: value,
            emitter: publisher,
            self_id: 0,
        }
    }

    pub fn publish_to(&mut self, destination: u64) {
        let event = TestEvent {
            destination,
            value: self.publisher_value,
            source: self.self_id,
        };

        self.emitter.publish(event);
    }
}

impl Publisher<TestEvent> for TestPublisher {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<TestEvent> {
        &mut self.emitter
    }
}

#[test]
fn test_multithreading() {
    let bus = Arc::new(EventBus::new());

    let subscriber1 = Arc::new(Mutex::new(TestSubscriber { id: 1 }));
    let subscriber2 = Arc::new(Mutex::new(TestSubscriber { id: 2 }));

    bus.add_subscriber(subscriber1.clone());
    bus.add_subscriber(subscriber2.clone());

    let mut publisher1 = TestPublisher::new(42);
    let mut publisher2 = TestPublisher::new(24);
    let mut publisher3 = TestPublisher::new(100);
    let mut publisher4 = TestPublisher::new(200);
    let mut publisher5 = TestPublisher::new(300);

    bus.add_publisher(&mut publisher1);
    bus.add_publisher(&mut publisher2);
    bus.add_publisher(&mut publisher3);
    bus.add_publisher(&mut publisher4);
    bus.add_publisher(&mut publisher5);

    let handle1 = thread::Builder::new()
        .name("publishing 1".to_string())
        .spawn(move || {
            for _ in 0..ITERATIONS {
                publisher1.publish_to(1);
            }
        })
        .unwrap();

    let handle2 = thread::Builder::new()
        .name("poublishing 2".to_string())
        .spawn(move || {
            for _ in 0..ITERATIONS {
                publisher2.publish_to(2);
            }
        })
        .unwrap();

    let handle3 = thread::Builder::new()
        .name("publishing 3".to_string())
        .spawn(move || {
            for _ in 0..ITERATIONS {
                publisher3.publish_to(2);
            }
        })
        .unwrap();

    let handle4 = thread::Builder::new()
        .name("publishing 4".to_string())
        .spawn(move || {
            for _ in 0..ITERATIONS {
                publisher4.publish_to(1);
            }
        })
        .unwrap();

    let handle5 = thread::Builder::new()
        .name("publishing 5".to_string())
        .spawn(move || {
            for _ in 0..ITERATIONS {
                publisher5.publish_to(1);
            }
        })
        .unwrap();

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();
    handle5.join().unwrap();

    println!("COUNTER: {}", COUNTER.load(Ordering::SeqCst));
    assert_eq!(COUNTER.load(Ordering::SeqCst), ITERATIONS * 5);
}
