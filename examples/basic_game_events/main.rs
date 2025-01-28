use eventing_rs::*;
mod commands;
mod input;
mod player;
use input::Input;
use player::Player;

fn main() {
    // Create a bus
    let mut bus = EventBus::new().into_shared();

    // Create players and subscribe them to the bus
    let player1 = Player { id: 1 }.into_shared();
    let player2 = Player { id: 2 }.into_shared();
    bus.with(|b| {
        b.subscribe(player1);
        b.subscribe(player2)
    });

    // Create an input and connect it to the bus
    let input = Input::new(bus.clone());

    // Send some events
    input.send_move(1, 1.0, 2.0);
    input.send_atack(2);
}
