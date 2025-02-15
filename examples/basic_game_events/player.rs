use crate::{
    commands::Commands,
    topic_ids::{TOPIC_PLAYER_1, TOPIC_PLAYER_2},
};
use pubsub_bus::*;

pub struct Player {
    pub id: u32,
}

impl Subscriber<Commands, String> for Player {
    fn on_event(&mut self, event: &BusEvent<Commands, String>) {
        let event_id = event.get_id();
        let event_source_id = event.get_source_id();
        match event.get_content() {
            Commands::Move { player_id, x, y } => {
                println!(
                    "[Player {}] Received event {} from {}: Move({}, {}, {})",
                    self.id, event_id, event_source_id, player_id, x, y
                );
            }
            Commands::Atack { player_id } => {
                println!(
                    "[Player {}] Received event {} from {}: Atack({})",
                    self.id, event_id, event_source_id, player_id
                );
            }
        }
    }

    fn get_subscribed_topics(&self) -> Option<Vec<String>> {
        if self.id == 1 {
            return Some(vec![TOPIC_PLAYER_1.to_string()]);
        }
        if self.id == 2 {
            return Some(vec![TOPIC_PLAYER_2.to_string()]);
        }
        None
    }
}
