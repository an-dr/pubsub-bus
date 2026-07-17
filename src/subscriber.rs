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
/// Override `is_subscribed_to` to specify the topics the subscriber is interested in.
/// The default implementation always returns true.
///
/// Override `on_event` to handle the event.
///
/// Only `Send` is required, not `Sync`: subscribers are always reached
/// through an `Arc<Mutex<dyn Subscriber<..>>>`, and `Arc<Mutex<T>>` is
/// itself `Send + Sync` whenever `T: Send` -- the trait does not need to
/// promise interior thread-safety on its own.
pub trait Subscriber<ContentType, TopicId>: Send {
    
    #[allow(unused_variables)] // This is a default implementation
    fn is_subscribed_to(&self, topic_id: &TopicId) -> bool {
        true
    }

    #[deprecated(since="3.1.0", note="Please use `is_subscribed_to` instead. Using of both methods is not recommended.")]
    fn get_subscribed_topics(&self) -> Option<Vec<TopicId>> {
        None
    }

    fn on_event(&mut self, event: &BusEvent<ContentType, TopicId>);
}
