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

use crate::BusEvent;
use std::collections::VecDeque;

pub trait BusEventQueue<ContentType, TopicId> {
    fn push_event(&mut self, event: BusEvent<ContentType, TopicId>);
    fn pop_event(&mut self) -> Option<BusEvent<ContentType, TopicId>>;
}

pub struct ReceivingQueue<ContentType, TopicId> {
    queue: VecDeque<BusEvent<ContentType, TopicId>>,
}

impl<ContentType, TopicId> BusEventQueue<ContentType, TopicId>
    for ReceivingQueue<ContentType, TopicId>
{
    fn push_event(&mut self, event: BusEvent<ContentType, TopicId>) {
        self.queue.push_back(event);
    }

    fn pop_event(&mut self) -> Option<BusEvent<ContentType, TopicId>> {
        self.queue.pop_front()
    }
}
