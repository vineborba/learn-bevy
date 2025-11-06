use bevy::prelude::*;

use crate::main_menu::{
    components::{MainMenu, PlayButton, QuitButton},
    styles::*,
};

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(8.0),
                ..Default::default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

            parent
                .spawn(title_container_layout())
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode {
                            image: asset_server.load("sprites/ball_blue_large.png"),
                            ..Default::default()
                        },
                        title_image_layout(),
                    ));

                    parent.spawn(title_text("Bevy Ball Game", font_handle.clone()));

                    parent.spawn((
                        ImageNode {
                            image: asset_server.load("sprites/ball_red_large.png"),
                            ..Default::default()
                        },
                        title_image_layout(),
                    ));
                });

            parent
                .spawn((default_button_layout(), PlayButton {}))
                .with_children(|parent| {
                    parent.spawn(button_text("Play", font_handle.clone()));
                });

            parent
                .spawn((default_button_layout(), QuitButton {}))
                .with_children(|parent| {
                    parent.spawn(button_text("Quit", font_handle.clone()));
                });
        })
        .id();

    main_menu_entity
}
