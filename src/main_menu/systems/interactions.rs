use bevy::prelude::*;

use crate::{
    AppState,
    main_menu::{
        components::*,
        styles::{DEFAULT_BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    },
};

pub fn interact_with_play_button(
    mut play_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = play_button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(PRESSED_BUTTON_COLOR);
                next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *background_color = BackgroundColor(DEFAULT_BUTTON_COLOR);
            }
        }
    }
}

pub fn interact_with_quit_button(
    mut quit_button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: MessageWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = quit_button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BackgroundColor(PRESSED_BUTTON_COLOR);
                app_exit_event_writer.write(AppExit::Success);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *background_color = BackgroundColor(DEFAULT_BUTTON_COLOR);
            }
        }
    }
}
