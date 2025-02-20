use crate::{commands::Commands, topic_ids::TopicIds};
use pubsub_bus::*;

pub struct Player {
    pub id: u32,
}

impl Subscriber<Commands, TopicIds> for Player {
    fn on_event(&mut self, event: &BusEvent<Commands, TopicIds>) {
        let event_id = event.get_id();
        let event_source_id = event.get_source_id();
        match event.get_content() {
            Commands::Move { dx, dy } => {
                println!(
                    "[Player {}] Received event {} from ID{}: Move({}, {})",
                    self.id, event_id, event_source_id, dx, dy
                );
            }
            Commands::Atack => {
                println!(
                    "[Player {}] Received event {} from ID{}: Atack",
                    self.id, event_id, event_source_id
                );
            }
        }
    }

    fn is_interested_in_topic(&self, topic_id: &TopicIds) -> bool {
        match topic_id {
            TopicIds::Player1 => self.id == 1,
            TopicIds::Player2 => self.id == 2,
        }
    }
}
