use bevy::prelude::*;

const BUTTON_PADDING: f32 = 5.;
const BUTTON_BORDER_RADIUS: f32 = 5.;
const FONT_SIZE: f32 = 40.0;
const FONT_COLOR: Color = Color::linear_rgb(0.9, 0.9, 0.9);

pub struct UiButtonPlugin;

impl Plugin for UiButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hover_button);
    }
}

fn hover_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
            _ => {}
        }
    }
}

pub fn ui_button(width: f32, height: f32) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(BUTTON_PADDING)),
            ..Default::default()
        },
        border_radius: BorderRadius::all(Val::Px(BUTTON_BORDER_RADIUS)),
        background_color: ButtonColors::default().normal.into(),
        ..Default::default()
    } 
}

pub fn ui_button_with_margin(width: f32, height: f32, margin: f32) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(width),
            height: Val::Px(height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(BUTTON_PADDING)),
            margin: UiRect::all(Val::Px(margin)),
            ..Default::default()
        },
        border_radius: BorderRadius::all(Val::Px(BUTTON_BORDER_RADIUS)),
        background_color: ButtonColors::default().normal.into(),
        ..Default::default()
    } 
}

pub fn ui_button_text(text: &str) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: FONT_SIZE,
            color: FONT_COLOR,
            ..default()
        }
    )
}

pub fn ui_button_text_font_size(text: &str, font_size: f32) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: font_size,
            color: FONT_COLOR,
            ..default()
        }
    )
}

#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(-2.15, 0.15, 0.15),
            hovered: Color::linear_rgb(-2.25, 0.25, 0.25),
        }
    }
}