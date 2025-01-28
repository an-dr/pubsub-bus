use crate::commands::Commands;
use eventing_rs::*;

pub struct Player {
    pub id: u32,
}

impl Subscriber<Commands> for Player {
    fn on_event(&mut self, event: &Event<Commands>) {
        let event_id = event.get_id();
        let event_source_id = event.get_source_id();
        match event.get_content() {
            Commands::Move { player_id, x, y } => {
                if *player_id != self.id {
                    return;
                }
                println!(
                    "Received event {} from {}: Move({}, {}, {})",
                    event_id, event_source_id, player_id, x, y
                );
            }
            Commands::Atack { player_id } => {
                if *player_id != self.id {
                    return;
                }
                println!(
                    "Received event {} from {}: Atack({})",
                    event_id, event_source_id, player_id
                );
            }
        }
    }
}
