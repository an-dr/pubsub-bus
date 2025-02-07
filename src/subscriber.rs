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
use super::Event;

#[cfg(test)]
mod tests;

pub trait Subscriber<ContentType>: Send + Sync {
    fn on_event(&mut self, event: &Event<ContentType>);
}
