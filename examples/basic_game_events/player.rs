use crate::commands::Commands;
use eventing_rs::*;

pub struct Player {
    pub id: u64,
}

impl Subscriber<Commands> for Player {
    fn on_event(&mut self, event: &Event<Commands>) {
        match event.get_content() {
            Commands::Move { player_id, x, y } => {
                println!(
                    "Received event with content: Move({}, {}, {})",
                    player_id, x, y
                );
            }
            Commands::Atack { player_id } => {
                println!("Received event with content: Atack({})", player_id);
            }
        }
    }
}
