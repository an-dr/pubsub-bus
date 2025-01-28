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
use crate::{Event, EventBus, Shared};

#[cfg(test)]
mod tests;

pub struct Publisher<ContentType> {
    event_bus: Option<Shared<EventBus<ContentType>>>,
}

impl<ContentType> Publisher<ContentType> {
    pub fn new() -> Self {
        Self { event_bus: None }
    }

    pub fn set_bus(&mut self, bus: Shared<EventBus<ContentType>>) {
        self.event_bus = Some(bus);
    }

    pub fn publish(&self, event: &Event<ContentType>) {
        if let Some(bus) = &self.event_bus {
            bus.lock().unwrap().publish(event);
        }
    }
}

trait WithPublisher<ContentType> {
    fn get_publisher(&self) -> &Publisher<ContentType>;

    fn publish(&self, event: Event<ContentType>) {
        self.get_publisher().publish(&event);
    }
}
