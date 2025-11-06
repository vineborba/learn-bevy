use bevy::prelude::*;

mod game;
mod main_menu;
pub mod root;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use root::RootPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(RootPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Hash, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}
