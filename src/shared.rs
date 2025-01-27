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
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests;

pub type Shared<T> = Arc<Mutex<T>>;

/// Convenience trait to add `into_shared()` to any type
pub trait IntoShared<T> {
    fn into_shared(self) -> Shared<T>;
}

impl<T> IntoShared<T> for T {
    fn into_shared(self) -> Shared<T> {
        Arc::new(Mutex::new(self))
    }
}
