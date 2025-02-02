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
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests;

pub struct Publisher<ContentType> {
    event_bus: Arc<Mutex<EventBus<ContentType>>>,
}

impl<ContentType> Publisher<ContentType> {
    pub fn new(bus: Arc<Mutex<EventBus<ContentType>>>) -> Self {
        Self { event_bus: bus }
    }

    pub fn publish(&self, content: ContentType) {
        self.event_bus
            .lock()
            .unwrap()
            .publish(&content.into_event());
    }
}
