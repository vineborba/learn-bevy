use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

use crate::{AppState, game::SimulationState};

pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(
                Update,
                (
                    (player_movement, confine_player_movement).chain(),
                    enemy_hit_player,
                    player_hit_star,
                )
                    .run_if(in_state(AppState::InGame).and(in_state(SimulationState::Running))),
            )
            .add_systems(OnExit(AppState::InGame), despawn_player);
    }
}
