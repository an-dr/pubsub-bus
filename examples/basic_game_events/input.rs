use crate::commands::Commands;
use crate::topic_ids::TopicIds;
use pubsub_bus::*;

#[allow(dead_code)] // allow dead code for illustrative purposes
pub struct Input {
    device: String, // E.g. "keyboard", "mouse", "gamepad"
    emitter: EventEmitter<Commands, TopicIds>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            device: "keyboard".to_string(),
            emitter: EventEmitter::new(),
        }
    }

    pub fn send_move(&mut self, topic: TopicIds, x: f32, y: f32) {
        let player_id = match topic {
            TopicIds::Player1 => 1,
            TopicIds::Player2 => 2,
        };
        let event = Commands::Move { player_id, x, y };
        self.emitter.publish(event, Some(topic));
    }

    pub fn send_atack(&mut self, topic: TopicIds) {
        let player_id = match topic {
            TopicIds::Player1 => 1,
            TopicIds::Player2 => 2,
        };
        let event = Commands::Atack { player_id };
        self.emitter.publish(event, Some(topic));
    }
}

impl Publisher<Commands, TopicIds> for Input {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<Commands, TopicIds> {
        &mut self.emitter
    }
}
