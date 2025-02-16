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
use super::{BusEvent, Publisher, Subscriber};
use std::sync::{Arc, Mutex, RwLock};

#[cfg(test)]
mod tests;

pub struct EventBus<ContentType, TopicId: std::cmp::PartialEq> {
    internal: Arc<EventBusInternal<ContentType, TopicId>>,
}

impl<ContentType, TopicId: std::cmp::PartialEq> EventBus<ContentType, TopicId> {
    pub fn new() -> Self {
        Self {
            internal: Arc::new(EventBusInternal::new()),
        }
    }

    pub fn add_subscriber_shared(&self, subscriber: Arc<Mutex<dyn Subscriber<ContentType, TopicId>>>) {
        self.internal.add_subscriber_shared(subscriber);
    }

    pub fn add_subscriber<S>(&self, subscriber: S)
    where
        S: Subscriber<ContentType, TopicId> + 'static, // Ensures it can be converted to a trait object
    {
        self.internal.add_subscriber(subscriber);
    }

    pub fn add_publisher(&self, publisher: &mut dyn Publisher<ContentType, TopicId>) {
        publisher.get_mut_emitter().set_bus(self);
    }

    pub fn publish(&self, event: ContentType, topic_id: Option<TopicId>) {
        self.internal.publish(event, topic_id);
    }

    pub fn get_internal(&self) -> Arc<EventBusInternal<ContentType, TopicId>> {
        self.internal.clone()
    }
}

pub struct EventBusInternal<ContentType, TopicId: std::cmp::PartialEq> {
    next_event_id: Arc<Mutex<usize>>,
    
    // RwLock as we do not expect many writes, but many reads
    subscribers: RwLock<Vec<Arc<Mutex<dyn Subscriber<ContentType, TopicId>>>>>,
}

impl<ContentType, TopicId: std::cmp::PartialEq> EventBusInternal<ContentType, TopicId> {
    pub fn new() -> Self {
        Self {
            next_event_id: Arc::new(Mutex::new(0)),
            subscribers: RwLock::new(Vec::new()),
        }
    }

    pub fn add_subscriber_shared(
        &self,
        subscriber: Arc<Mutex<dyn Subscriber<ContentType, TopicId>>>,
    ) {
        self.subscribers.write().unwrap().push(subscriber);
    }

    // Accepts any object implementing Subscriber and wraps it in Arc + Mutex
    pub fn add_subscriber<S>(&self, subscriber: S)
    where
        S: Subscriber<ContentType, TopicId> + 'static, // Ensures it can be converted to a trait object
    {
        let subscriber = Arc::new(Mutex::new(subscriber));

        self.subscribers.write().unwrap().push(subscriber);
    }

    pub fn get_next_id(&self) -> usize {
        let mut id = self.next_event_id.lock().unwrap();
        *id += 1;
        *id
    }

    pub fn publish(&self, event: ContentType, topic_id: Option<TopicId>) {
        // reserve a new id for the event
        let id = self.get_next_id();

        let mut event_internal = BusEvent::new(event, topic_id);
        event_internal.set_id(id);

        // notify all subscribers
        for s in self.subscribers.read().unwrap().iter() {
            // if there are topics
            let topics = s.lock().unwrap().get_subscribed_topics();
            if let Some(topics) = topics {
                // if the subscriber is not subscribed to the topic
                if !topics.contains(event_internal.get_topic_id().as_ref().unwrap()) {
                    continue;
                }
            }

            s.lock().unwrap().on_event(&event_internal);
        }
    }
}
