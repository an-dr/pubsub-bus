use crate::commands::Commands;
use pubsub_bus::*;

pub struct Input {
    device: String,
    publisher: Publisher<Commands>,
}

impl Input {
    pub fn new(bus: Shared<EventBus<Commands>>) -> Self {
        let mut publisher = Publisher::new();
        publisher.set_bus(bus);
        Self {
            device: "keyboard".to_string(),
            publisher: publisher,
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
