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
#[cfg(test)]
mod tests;

pub struct Event<T> {
    id: u64,
    source_id: u64,
    content: T,
}

impl<T> Event<T> {
    pub fn new(content: T) -> Self {
        Event {
            id: 0,
            source_id: 0,
            content,
        }
    }

    pub fn set_header(&mut self, id: u64, source_id: u64) {
        self.id = id;
        self.source_id = source_id;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_source_id(&self) -> u64 {
        self.source_id
    }

    pub fn get_content(&self) -> &T {
        &self.content
    }

    pub fn get_mut_content(&mut self) -> &mut T {
        &mut self.content
    }
}

impl<T> From<T> for Event<T> {
    fn from(content: T) -> Self {
        Event::new(content)
    }
}
