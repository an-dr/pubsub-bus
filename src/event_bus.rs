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
use crate::{event_bus_internal::EventBusInternal, Publisher, Subscriber};
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests;

/// The Event Bus itself. Add subscribers and publishers to it.
pub struct EventBus<ContentType, TopicId: std::cmp::PartialEq> {
    internal: Arc<EventBusInternal<ContentType, TopicId>>,
}

impl<ContentType, TopicId: std::cmp::PartialEq> EventBus<ContentType, TopicId> {
    pub fn new() -> Self {
        Self {
            internal: Arc::new(EventBusInternal::new()),
        }
    }

    pub fn add_subscriber_shared(
        &self,
        subscriber: Arc<Mutex<dyn Subscriber<ContentType, TopicId>>>,
    ) {
        self.internal.add_subscriber_shared(subscriber);
    }

    pub fn add_subscriber<S>(&self, subscriber: S)
    where
        S: Subscriber<ContentType, TopicId> + 'static, // Ensures it can be converted to a trait object
    {
        self.internal.add_subscriber(subscriber);
    }

    /// Add a publisher to the event bus.
    /// If source_id is None, the publisher will be assigned a unique id.
    pub fn add_publisher(
        &self,
        publisher: &mut dyn Publisher<ContentType, TopicId>,
        source_id: Option<u64>,
    ) -> Result<(), &'static str> {
        publisher.get_mut_emitter().set_bus(self, source_id)
    }

    pub fn publish(&self, event: ContentType, topic_id: Option<TopicId>, source_id: u64) {
        self.internal.publish(event, topic_id, source_id);
    }

    pub fn get_internal(&self) -> Arc<EventBusInternal<ContentType, TopicId>> {
        self.internal.clone()
    }
}
