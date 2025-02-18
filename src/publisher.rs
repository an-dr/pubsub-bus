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
use crate::{event_bus_internal::EventBusInternal, EventBus};
use std::sync::Arc;

#[cfg(test)]
mod tests;

/// EventEmitter is a struct that can be used to publish events to the event bus.
/// It supposed to be used by the publisher.
pub struct EventEmitter<ContentType, TopicId: std::cmp::PartialEq> {
    event_bus: Option<Arc<EventBusInternal<ContentType, TopicId>>>,
    source_id: u64,
}

impl<ContentType, TopicId: std::cmp::PartialEq> EventEmitter<ContentType, TopicId> {
    pub fn with_bus(bus: &EventBus<ContentType, TopicId>) -> Self {
        Self {
            event_bus: Some(bus.get_internal()),
            source_id: 0,
        }
    }

    pub fn new() -> Self {
        Self {
            event_bus: None,
            source_id: 0,
        }
    }

    /// Set the event bus for the emitter.
    /// If source_id is None, the publisher will be assigned a unique id.
    pub fn set_bus(
        &mut self,
        bus: &EventBus<ContentType, TopicId>,
        source_id: Option<u64>,
    ) -> Result<(), &'static str> {
        let internal_bus = bus.get_internal();
        let id = internal_bus.register_publisher(source_id)?;

        self.source_id = id;
        self.event_bus = Some(bus.get_internal());
        Ok(())
    }

    /// Publish an event to the event bus.
    /// If topic_id is None, the event will be sent to all subscribers.
    pub fn publish(&mut self, content: ContentType, topic_id: Option<TopicId>) {
        match &mut self.event_bus {
            None => {
                panic!("Publisher has no bus");
            }
            Some(bus) => {
                bus.publish(content, topic_id, self.source_id);
            }
        }
    }
}

/// Publisher is a trait that defines a publisher to the event bus.
/// Publisher is expected to care an EventEmitter.
pub trait Publisher<ContentType, TopicId: std::cmp::PartialEq> {
    /// Get the emitter of the publisher. Has to be implemented by the publisher.
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<ContentType, TopicId>;

    /// Default implementation to Publish an event to the event bus.
    /// If topic_id is None, the event will be sent to all subscribers.
    fn publish(&mut self, content: ContentType, topic_id: Option<TopicId>) {
        self.get_mut_emitter().publish(content, topic_id);
    }
}
