use bevy::prelude::*;

mod game;
mod menu;
pub mod root;

use game::GamePlugin;
use menu::MenuPlugin;
use root::RootPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(RootPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(MenuPlugin)
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Hash, Clone, Copy, Default)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    GameOver,
}
