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

/// A struct that represents an event that can be sent over the event bus.
/// The content is user-defined. Besudes the content, the event has an id,
/// a source id, and a topic id.
pub struct BusEvent<ContentType, TopicId> {
    id: usize,
    topic_id: Option<TopicId>,
    source_id: u64,
    content: ContentType,
}

impl<ContentType, TopicId> BusEvent<ContentType, TopicId> {
    pub fn new(id: usize, source_id: u64, topic_id: Option<TopicId>, content: ContentType) -> Self {
        BusEvent {
            id,
            topic_id,
            source_id,
            content,
        }
    }

    pub fn get_topic_id(&self) -> &Option<TopicId> {
        &self.topic_id
    }

    pub fn get_id(&self) -> usize {
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
