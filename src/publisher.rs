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

pub struct EventEmitter<ContentType> {
    event_bus: Option<Arc<EventBus<ContentType>>>,
}

impl<ContentType> EventEmitter<ContentType> {
    pub fn with_bus(bus: Arc<EventBus<ContentType>>) -> Self {
        Self {
            event_bus: Some(bus),
        }
    }

    pub fn new() -> Self {
        Self { event_bus: None }
    }

    pub fn set_bus(&mut self, bus: Arc<EventBus<ContentType>>) {
        self.event_bus = Some(bus);
    }

    pub fn publish(&mut self, content: ContentType) {
        let mut event = content.into_event();
        match &mut self.event_bus {
            None => {
                log::error!("Publisher has no bus");
                return;
            }
            Some(bus) => {
                bus.publish(&mut event);
            }
        }
    }
}

pub trait Publisher<ContentType> {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<ContentType>;
}
