use crate::commands::Commands;
use pubsub_bus::*;

#[allow(dead_code)] // allow dead code for illustrative purposes
pub struct Input {
    device: String, // E.g. "keyboard", "mouse", "gamepad"
    emitter: EventEmitter<Commands>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            device: "keyboard".to_string(),
            emitter: EventEmitter::new(),
        }
    }

    pub fn send_move(&mut self, player_id: u32, x: f32, y: f32) {
        let event = Commands::Move { player_id, x, y };

        self.emitter.publish(event);
    }

    pub fn send_atack(&mut self, player_id: u32) {
        let event = Commands::Atack { player_id };

        self.emitter.publish(event);
    }
}

impl Publisher<Commands> for Input {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<Commands> {
        &mut self.emitter
    }
}
