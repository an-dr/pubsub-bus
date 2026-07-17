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
use crate::event_bus_internal::{EventBusInternal, SharedSubscriber};
use crate::{Publisher, Subscriber};
use std::sync::Arc;

#[cfg(test)]
mod tests;

/// The Event Bus itself. Add subscribers and publishers to it.
///
/// Cheap to clone: every clone shares the same underlying bus (the
/// internal state is already `Arc`-backed).
pub struct EventBus<ContentType, TopicId: std::cmp::PartialEq + Clone> {
    internal: Arc<EventBusInternal<ContentType, TopicId>>,
}

// Written by hand rather than `#[derive(Clone)]`: derive would add a
// `ContentType: Clone` bound (it can't see that `Arc<T>` clones without
// `T: Clone`), forcing every event payload type to be `Clone` just to
// clone the bus handle.
impl<ContentType, TopicId: std::cmp::PartialEq + Clone> Clone for EventBus<ContentType, TopicId> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
        }
    }
}

impl<ContentType, TopicId: std::cmp::PartialEq + Clone> Default for EventBus<ContentType, TopicId> {
    fn default() -> Self {
        Self::new()
    }
}

impl<ContentType, TopicId: std::cmp::PartialEq + Clone> EventBus<ContentType, TopicId> {
    pub fn new() -> Self {
        Self {
            internal: Arc::new(EventBusInternal::new()),
        }
    }

    pub fn add_subscriber_shared(
        &self,
        subscriber: SharedSubscriber<ContentType, TopicId>,
    ) {
        self.internal.add_subscriber_shared(subscriber);
    }

    /// Removes a subscriber previously added via `add_subscriber_shared`,
    /// identified by pointer identity. No-op if it is not present.
    pub fn remove_subscriber_shared(
        &self,
        subscriber: &SharedSubscriber<ContentType, TopicId>,
    ) {
        self.internal.remove_subscriber_shared(subscriber);
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

    /// Queues an event for the next `dispatch()`. Unlike `publish()`, safe
    /// to call reentrantly from `on_event`.
    pub fn enqueue(&self, event: ContentType, topic_id: Option<TopicId>, source_id: u64) {
        self.internal.enqueue(event, topic_id, source_id);
    }

    /// Delivers everything queued by `enqueue()` since the last call, in
    /// order, via `publish()`.
    pub fn dispatch(&self) {
        self.internal.dispatch();
    }

    pub fn get_internal(&self) -> Arc<EventBusInternal<ContentType, TopicId>> {
        self.internal.clone()
    }
}
