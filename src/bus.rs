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
use super::{Event, Subscriber};
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests;

pub struct EventBus<ContentType> {
    subscribers: Vec<Arc<Mutex<dyn Subscriber<ContentType>>>>,
}

impl<ContentType> EventBus<ContentType> {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: Arc<Mutex<dyn Subscriber<ContentType>>>) {
        self.subscribers.push(subscriber);
    }

    pub fn publish(&mut self, event: ContentType) {
        for s in self.subscribers.iter_mut() {
            s.lock().unwrap().on_event(event.into_e);
        }
    }
}
