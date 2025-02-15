use core::panic;

use crate::commands::Commands;
use crate::topic_ids::*;
use pubsub_bus::*;

#[allow(dead_code)] // allow dead code for illustrative purposes
pub struct Input {
    device: String, // E.g. "keyboard", "mouse", "gamepad"
    emitter: EventEmitter<Commands, String>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            device: "keyboard".to_string(),
            emitter: EventEmitter::new(),
        }
    }

    pub fn send_move(&mut self, topic: &str, x: f32, y: f32) {
        let player_id = match topic {
            TOPIC_PLAYER_1 => 1,
            TOPIC_PLAYER_2 => 2,
            _ => panic!("Unknown topic"),
        };
        let event = Commands::Move { player_id, x, y };
        self.emitter.publish(event, Some(topic.to_string()));
    }

    pub fn send_atack(&mut self, topic: &str) {
        let player_id = match topic {
            TOPIC_PLAYER_1 => 1,
            TOPIC_PLAYER_2 => 2,
            _ => panic!("Unknown topic"),
        };
        let event = Commands::Atack { player_id };
        self.emitter.publish(event, Some(topic.to_string()));
    }
}

impl Publisher<Commands, String> for Input {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<Commands, String> {
        &mut self.emitter
    }
}
