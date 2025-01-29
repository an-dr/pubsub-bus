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

pub struct Event<ContentType> {
    id: u64,
    source_id: u64,
    content: ContentType,
}

impl<ContentType> Event<ContentType> {
    pub fn new(content: ContentType) -> Self {
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

    pub fn get_content(&self) -> &ContentType {
        &self.content
    }

    pub fn get_mut_content(&mut self) -> &mut ContentType {
        &mut self.content
    }
}

pub trait IntoEvent<ContentType> {
    fn into_event(self) -> Event<ContentType>;
}

impl<ContentType> IntoEvent<ContentType> for ContentType {
    fn into_event(self) -> Event<ContentType> {
        Event::new(self)
    }
}
