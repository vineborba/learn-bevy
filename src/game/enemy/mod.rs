use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::{AppState, game::SimulationState};

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::InGame), spawn_enemies)
            .add_systems(
                Update,
                (
                    (
                        enemy_movement,
                        confine_enemy_movement,
                        update_enemy_direction,
                    )
                        .chain(),
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::InGame).and(in_state(SimulationState::Running))),
            )
            .add_systems(OnExit(AppState::InGame), despawn_enemies);
    }
}
