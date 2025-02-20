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

use std::sync::{Arc, Mutex, RwLock};

use crate::{BusEvent, Subscriber};

pub struct EventBusInternal<ContentType, TopicId: std::cmp::PartialEq + Clone> {
    next_event_id: Arc<Mutex<usize>>,

    // RwLock as we do not expect many writes, but many reads
    subscribers: RwLock<Vec<Arc<Mutex<dyn Subscriber<ContentType, TopicId>>>>>,

    // Publisher IDs
    publishers: RwLock<Vec<u64>>,
}

impl<ContentType, TopicId: std::cmp::PartialEq + Clone> EventBusInternal<ContentType, TopicId> {
    pub fn new() -> Self {
        Self {
            next_event_id: Arc::new(Mutex::new(0)),
            subscribers: RwLock::new(Vec::new()),
            publishers: RwLock::new(Vec::new()),
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

    pub fn register_publisher(&self, source_id: Option<u64>) -> Result<u64, &'static str> {
        let mut publishers = self.publishers.write().unwrap();
        let id = match source_id {
            // If the source_id is provided, check if it already exists
            Some(id) => {
                if publishers.contains(&id) {
                    return Err("Publisher with the same id already exists");
                }
                id
            }
            // If the source_id is not provided, assign a new id in sequence
            None => {
                let mut id = 0;
                while publishers.contains(&id) {
                    id += 1;
                }
                id
            }
        };

        publishers.push(id);
        Ok(id)
    }

    pub fn get_next_id(&self) -> usize {
        let mut id = self.next_event_id.lock().unwrap();
        *id += 1;
        *id
    }

    pub fn publish(&self, event: ContentType, topic_id: Option<TopicId>, source_id: u64) {
        let id = self.get_next_id(); // reserve a new id for the event
        let event_internal = BusEvent::new(id, source_id, topic_id.clone(), event);

        // notify all subscribers
        for s in self.subscribers.read().unwrap().iter() {
            // If for a specific topic, check if the subscriber is interested in the topic
            if topic_id.is_some() {
                let topic_id = topic_id.as_ref().unwrap();
                if !s.lock().unwrap().is_subscribed_to(topic_id) {
                    continue;
                }

                {
                    // TODO: Remove this deprecated block in the next major release
                    let topics = s.lock().unwrap().get_subscribed_topics();
                    if let Some(topics) = topics {
                        // if the subscriber is not subscribed to the topic
                        if !topics.contains(event_internal.get_topic_id().as_ref().unwrap()) {
                            continue;
                        }
                    }
                }
            }

            s.lock().unwrap().on_event(&event_internal);
        }
    }
}
