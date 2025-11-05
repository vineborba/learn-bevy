use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use crate::AppState;

use {enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin, star::StarPlugin, systems::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(ScorePlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::InGame)));
    }
}

#[derive(States, PartialEq, Eq, Debug, Hash, Clone, Copy, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}
