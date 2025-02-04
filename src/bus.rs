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
use super::{Event, Publisher, Subscriber};
use std::sync::{Arc, Mutex, RwLock};

#[cfg(test)]
mod tests;

pub struct EventBus<ContentType> {
    next_event_id: Arc<Mutex<u64>>,
    // RwLock is we do not expect many writes, but many reads
    subscribers: RwLock<Vec<Arc<Mutex<dyn Subscriber<ContentType>>>>>,
}

impl<ContentType> EventBus<ContentType> {
    pub fn new() -> Self {
        Self {
            next_event_id: Arc::new(Mutex::new(0)),
            subscribers: RwLock::new(Vec::new()),
        }
    }

    pub fn add_subscriber(&self, subscriber: Arc<Mutex<dyn Subscriber<ContentType>>>) {
        self.subscribers.write().unwrap().push(subscriber);
    }

    pub fn add_publisher(self: &Arc<Self>, publisher: &mut dyn Publisher<ContentType>) {
        publisher.get_mut_emitter().set_bus(self.clone());
    }

    pub fn get_next_id(&self) -> u64 {
        let mut id = self.next_event_id.lock().unwrap();
        *id += 1;
        *id
    }

    pub fn publish(&self, event: &mut Event<ContentType>) {
        // reserve a new id for the event
        let id = self.get_next_id();
        event.set_id(id);

        // notify all subscribers
        for s in self.subscribers.read().unwrap().iter() {
            s.lock().unwrap().on_event(&event);
        }
    }
}
