use bevy::prelude::*;

pub const DEFAULT_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn default_button_layout() -> (Button, Node, BackgroundColor) {
    (
        Button,
        Node {
            height: Val::Px(80.0),
            width: Val::Px(200.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Node::default()
        },
        BackgroundColor(DEFAULT_BUTTON_COLOR),
    )
}

pub fn button_text(label: &str, font: Handle<Font>) -> impl Bundle {
    (
        Text::new(label),
        TextFont {
            font,
            font_size: 32.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout {
            justify: Justify::Center,
            ..Default::default()
        },
    )
}

pub fn title_image_layout() -> Node {
    Node {
        height: Val::Px(64.0),
        width: Val::Px(64.0),
        margin: UiRect::all(Val::Px(8.0)),
        ..Node::default()
    }
}

pub fn title_container_layout() -> Node {
    Node {
        width: Val::Px(600.0),
        height: Val::Px(120.0),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        row_gap: Val::Px(8.0),
        ..Default::default()
    }
}

pub fn title_text(label: &str, font: Handle<Font>) -> impl Bundle {
    (
        Text::new(label),
        TextFont {
            font,
            font_size: 64.0,
            ..Default::default()
        },
        TextColor(Color::WHITE),
        TextLayout {
            justify: Justify::Center,
            ..Default::default()
        },
    )
}
