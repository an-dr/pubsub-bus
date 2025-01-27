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
use std::collections::VecDeque;

#[cfg(test)]
mod tests;

pub struct EventQueue<T> {
    events: VecDeque<T>,
}

impl<T> EventQueue<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(max_size),
        }
    }

    // Push a new event into the queue
    pub fn push(&mut self, event: T) {
        self.events.push_back(event);
    }

    // Pop an event from the queue
    pub fn pop(&mut self) -> Option<T> {
        self.events.pop_front()
    }
}
