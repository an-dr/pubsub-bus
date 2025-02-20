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
// *************************************************************************\

use super::BusEvent;

#[cfg(test)]
mod tests;

/// A trait that defines a subscriber to the event bus.
/// 
/// Override `is_interested_in_topic` to specify the topics the subscriber is interested in.
/// The default implementation always returns true.
/// 
/// Override `on_event` to handle the event.
pub trait Subscriber<ContentType, TopicId>: Send + Sync {
    
    #[allow(unused_variables)] // This is a default implementation
    fn is_interested_in_topic(&self, topic_id: &TopicId) -> bool {
        true
    }

    #[deprecated(since="3.1.0", note="Please use `is_interested_in_topic` instead. Using of both methods is not recommended.")]
    fn get_subscribed_topics(&self) -> Option<Vec<TopicId>> {
        None
    }

    fn on_event(&mut self, event: &BusEvent<ContentType, TopicId>);
}
