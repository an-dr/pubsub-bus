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
use crate::event::IntoEvent;
use crate::EventBus;
use std::sync::Arc;

#[cfg(test)]
mod tests;

pub struct EventEmitter<ContentType, TopicId: std::cmp::PartialEq> {
    event_bus: Option<Arc<EventBus<ContentType, TopicId>>>,
}

impl<ContentType, TopicId: std::cmp::PartialEq> EventEmitter<ContentType, TopicId> {
    pub fn with_bus(bus: Arc<EventBus<ContentType, TopicId>>) -> Self {
        Self {
            event_bus: Some(bus),
        }
    }

    pub fn new() -> Self {
        Self { event_bus: None }
    }

    pub fn set_bus(&mut self, bus: Arc<EventBus<ContentType, TopicId>>) {
        self.event_bus = Some(bus);
    }

    pub fn publish(&mut self, content: ContentType, topic_id: Option<TopicId>) {
        
        let mut event = content.into_event(topic_id);
        match &mut self.event_bus {
            None => {
                panic!("Publisher has no bus");
            }
            Some(bus) => {
                bus.publish(&mut event, topic_id);
            }
        }
    }
}

pub trait Publisher<ContentType, TopicId: std::cmp::PartialEq> {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<ContentType, TopicId>;
}
