pub enum Commands {
    Atack { player_id: u32 },
    Move { player_id: u32, x: f32, y: f32 },
}
