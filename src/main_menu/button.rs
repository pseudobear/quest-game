
use crate::main_menu::ButtonColors;
use bevy::prelude::*;

const BUTTON_PADDING: f32 = 5.;
const FONT_SIZE: f32 = 40.0;
const FONT_COLOR: Color = Color::linear_rgb(0.9, 0.9, 0.9);

pub fn menu_button(width: f32, height: f32) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(BUTTON_PADDING)),
            ..Default::default()
        },
        background_color: ButtonColors::default().normal.into(),
        ..Default::default()
    } 
}

pub fn menu_button_text(text: &str) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: FONT_SIZE,
            color: FONT_COLOR,
            ..default()
        }
    )
}