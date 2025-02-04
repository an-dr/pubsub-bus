mod commands;
mod input;
mod player;
use commands::Commands;
use input::Input;
use player::Player;
use pubsub_bus::*;
use std::sync::{Arc, Mutex};

fn main() {
    // Create a bus
    let bus: Arc<EventBus<Commands>> = Arc::new(EventBus::new());

    // Create players and subscribe them to the bus
    let player1 = Arc::new(Mutex::new(Player { id: 1 }));
    let player2 = Arc::new(Mutex::new(Player { id: 2 }));
    let mut  input = Input::new();

    bus.add_subscriber(player1);
    bus.add_subscriber(player2);
    bus.add_publisher(&mut input);

    // Create an input and connect it to the bus

    // Send some events
    input.send_move(1, 1.0, 2.0);
    input.send_atack(2);
}
