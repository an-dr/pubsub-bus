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
}

impl Publisher<Commands, TopicIds> for Input {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<Commands, TopicIds> {
        &mut self.emitter
    }
}
