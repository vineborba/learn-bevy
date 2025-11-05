use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::{AppState, game::SimulationState};

pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::InGame), spawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .chain()
                    .run_if(in_state(AppState::InGame).and(in_state(SimulationState::Running))),
            )
            .add_systems(OnExit(AppState::InGame), despawn_stars);
    }
}
