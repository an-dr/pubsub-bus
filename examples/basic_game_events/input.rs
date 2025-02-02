#[allow(dead_code)] // allow dead code for illustrative purposes
use crate::commands::Commands;
use pubsub_bus::*;
use std::sync::{Arc, Mutex};

pub struct Input {
    device: String,
    publisher: Publisher<Commands>,
}

impl Input {
    pub fn new(bus: Arc<Mutex<EventBus<Commands>>>) -> Self {
        Self {
            device: "keyboard".to_string(),
            publisher: Publisher::new(bus),
        }
    }

    pub fn send_move(&self, player_id: u32, x: f32, y: f32) {
        let event = Commands::Move { player_id, x, y }.into_event();

        self.publisher.publish(&event);
    }

    pub fn send_atack(&self, player_id: u32) {
        let event = Commands::Atack { player_id }.into_event();

        self.publisher.publish(&event);
    }
}
