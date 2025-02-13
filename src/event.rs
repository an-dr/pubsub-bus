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

pub struct Event<ContentType, TopicId> {
    id: usize,
    topic_id: Option<TopicId>,
    source_id: u64,
    content: ContentType,
}

impl<ContentType, TopicId> Event<ContentType, TopicId> {
    pub fn new(content: ContentType, topic_id: Option<TopicId>) -> Self {
        Event {
            id: 0,
            topic_id,
            source_id: 0,
            content,
        }
    }

    pub fn set_header(&mut self, id: usize, source_id: u64) {
        self.id = id;
        self.source_id = source_id;
    }

    pub fn get_topic_id(&self) -> &Option<TopicId> {
        &self.topic_id
    }

    pub fn set_topic_id(&mut self, topic_id: Option<TopicId>) {
        self.topic_id = topic_id;
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

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

pub trait IntoEvent<ContentType, TopicId> {
    fn into_event(self, topic: Option<TopicId>) -> Event<ContentType, TopicId>;
}

impl<ContentType, TopicId> IntoEvent<ContentType, TopicId> for ContentType {
    fn into_event(self, topic: Option<TopicId>) -> Event<ContentType, TopicId> {
        Event::new(self, topic)
    }
}
