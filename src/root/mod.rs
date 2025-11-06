use bevy::prelude::*;

mod messages;
mod systems;

pub use messages::*;
use systems::*;

pub struct RootPlugin;

impl Plugin for RootPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<GameOverMessage>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, transition_to_game_state)
            .add_systems(Update, transition_to_main_menu_state)
            .add_systems(Update, exit_game)
            .add_systems(Update, handle_game_over);
    }
}
