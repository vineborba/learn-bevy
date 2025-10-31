use bevy::prelude::*;

use crate::{
    enemy::EnemyPlugin, player::PlayerPlugin, root::RootPlugin, score::ScorePlugin,
    star::StarPlugin,
};

mod enemy;
mod player;
pub mod root;
mod score;
mod star;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RootPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(ScorePlugin)
        .run();
}
