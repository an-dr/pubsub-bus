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
pub trait Subscriber<ContentType, TopicId>: Send + Sync {
    
    #[allow(unused_variables)] // This is a default implementation
    fn is_subscribed_to(&self, topic_id: &TopicId) -> bool {
        true
    }

    /// Handles an incoming event.
    ///
    /// **Note**: Implementations of this method should aim to be efficient and
    /// non-blocking. Slow or blocking operations within `on_event` can delay
    /// event propagation to other subscribers, as the event bus processes
    /// subscribers sequentially for each event.
    fn on_event(&mut self, event: &BusEvent<ContentType, TopicId>);
}
