mod commands;
mod input;
mod player;
mod topic_ids;
use commands::Commands;
use input::Input;
use player::Player;
use pubsub_bus::*;
use std::sync::{Arc, Mutex};
use topic_ids::{TOPIC_PLAYER_1, TOPIC_PLAYER_2};

fn main() {
    // Create a bus
    let bus: Arc<EventBus<Commands, u32>> = Arc::new(EventBus::new());

    // Create players and subscribe them to the bus
    let player1 = Arc::new(Mutex::new(Player { id: 1 }));
    let player2 = Arc::new(Mutex::new(Player { id: 2 }));
    let mut input = Input::new();

    bus.add_subscriber(player1);
    bus.add_subscriber(player2);
    bus.add_publisher(&mut input);

    // Create an input and connect it to the bus

    // Send some events
    input.send_move(TOPIC_PLAYER_1, 1.0, 2.0);
    input.send_move(TOPIC_PLAYER_2, 1.0, 2.0);
    input.send_atack(TOPIC_PLAYER_2);
    input.send_atack(TOPIC_PLAYER_1);
}
