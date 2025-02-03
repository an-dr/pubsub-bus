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
use super::{Event, Subscriber, event::IntoEvent};
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests;

pub struct EventBus<ContentType> {
    next_id: u64,
    subscribers: Vec<Arc<Mutex<dyn Subscriber<ContentType>>>>,
}

impl<ContentType> EventBus<ContentType> {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: Arc<Mutex<dyn Subscriber<ContentType>>>) {
        self.subscribers.push(subscriber);
    }
    
    pub fn get_next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn publish(&mut self, event: &mut Event<ContentType>) {
        let id = self.get_next_id();
        for s in self.subscribers.iter_mut() {
            event.set_id(id);
            s.lock().unwrap().on_event(&event);
        }
    }
}
