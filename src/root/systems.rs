use bevy::{app::AppExit, prelude::*, render::view::Hdr, window::PrimaryWindow};

use super::messages::GameOverMessage;
use crate::AppState;
use crate::game::SimulationState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d::default(),
        Hdr::default(),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn transition_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) && !matches!(app_state.get(), AppState::InGame) {
        next_state.set(AppState::InGame);
        println!("Transitioning to InGame state");
    }
}

pub fn transition_to_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) && !matches!(app_state.get(), AppState::Menu) {
        next_app_state.set(AppState::Menu);
        next_simulation_state.set(SimulationState::Paused);
        println!("Transitioning to Menu state");
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut game_over_message_reader: MessageReader<GameOverMessage>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for message in game_over_message_reader.read() {
        println!("Your final score is: {}", message.score);
        next_app_state.set(AppState::GameOver);
    }
}
