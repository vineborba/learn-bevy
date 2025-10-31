use bevy::prelude::*;

#[derive(Message)]
pub struct GameOverMessage {
    pub score: u32,
}
