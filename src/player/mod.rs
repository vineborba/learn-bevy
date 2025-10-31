use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, confine_player_movement)
            .add_systems(Update, enemy_hit_player)
            .add_systems(Update, player_hit_star);
    }
}
