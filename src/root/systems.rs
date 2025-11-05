use bevy::{app::AppExit, prelude::*, render::view::Hdr, window::PrimaryWindow};

use super::messages::GameOverMessage;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().unwrap();

    commands.spawn((
        Camera2d::default(),
        Hdr::default(),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    ));
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.write(AppExit::Success);
    }
}

pub fn handle_game_over(mut game_over_message_reader: MessageReader<GameOverMessage>) {
    for message in game_over_message_reader.read() {
        println!("Your final score is: {}", message.score);
    }
}
