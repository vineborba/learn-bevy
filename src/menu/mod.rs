use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu);
    }
}

pub fn main_menu() {
    println!("Main Menu");
}
