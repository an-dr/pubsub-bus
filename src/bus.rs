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
use super::{Event, Publisher, Subscriber};
use std::sync::{Arc, Mutex, RwLock};

#[cfg(test)]
mod tests;

pub struct EventBus<ContentType> {
    next_event_id: Arc<Mutex<usize>>,
    // RwLock as we do not expect many writes, but many reads
    subscribers: RwLock<Vec<Arc<Mutex<dyn Subscriber<ContentType>>>>>,
}

impl<ContentType> EventBus<ContentType> {
    pub fn new() -> Self {
        Self {
            next_event_id: Arc::new(Mutex::new(0)),
            subscribers: RwLock::new(Vec::new()),
        }
    }

    pub fn add_subscriber(&self, subscriber: Arc<Mutex<dyn Subscriber<ContentType>>>) {
        self.subscribers.write().unwrap().push(subscriber);
    }

    pub fn add_publisher(self: &Arc<Self>, publisher: &mut dyn Publisher<ContentType>) {
        publisher.get_mut_emitter().set_bus(self.clone());
    }

    pub fn get_next_id(&self) -> usize {
        let mut id = self.next_event_id.lock().unwrap();
        *id += 1;
        *id
    }

    pub fn publish(&self, event: &mut Event<ContentType>, topic_id: Option<u32>) {
        // reserve a new id for the event
        let id = self.get_next_id();
        event.set_id(id);

        // set the topic id
        if topic_id == Some(0) {
            println!("Topic id 0 is the same as no topic id. Use None instead.");
        }
        if let Some(topic_id) = topic_id {
            event.set_topic_id(topic_id);
        }

        // notify all subscribers
        for s in self.subscribers.read().unwrap().iter() {
            // if there are topics
            let topics = s.lock().unwrap().get_subscribed_topics();
            if let Some(topics) = topics {
                // if the subscriber is not subscribed to the topic
                if !topics.contains(&event.get_topic_id()) {
                    continue;
                }
            }

            s.lock().unwrap().on_event(&event);
        }
    }
}
