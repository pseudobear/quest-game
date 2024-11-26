use crate::ui::buttons::ButtonColors;
use bevy::prelude::*;

// how much larger is the frame compared to the inner (both x and y)
const WINDOW_FRAME_OFFSET: f32 = 6.0;
const WINDOW_TITLE_BAR_SIZE: f32 = 20.0;

const WINDOW_FRAME_COLOR: Color = Color::WHITE;
const WINDOW_INNER_COLOR: Color = Color::linear_rgb(0.01, 0.05, 0.1);

const CLOSE_BUTTON_SIZE: f32 = 17.0;

pub struct UiWindowPlugin;

impl Plugin for UiWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, detect_close_window_button);
    }
}

#[derive(Component)]
struct WindowCloseButton;

fn detect_close_window_button(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<WindowCloseButton>)>,
    parent_query: Query<&Parent>
) {
    for (interaction, entity) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                for ancestor in parent_query.iter_ancestors(entity) {
                    commands.entity(ancestor).despawn_recursive();
                }
            }
            _ => {}
        }
    }
}

pub fn spawn_ui_window<WindowComponentTag: Component>(
    commands: &mut Commands,
    window_size: Vec2,
    window_name: &str,
) -> Entity where WindowComponentTag: Default {
    let window_container = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(window_size.x + WINDOW_FRAME_OFFSET),
                height: Val::Px(window_size.y + WINDOW_FRAME_OFFSET + WINDOW_TITLE_BAR_SIZE),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(WINDOW_FRAME_OFFSET / 2.)),
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                ..default()
            },
            background_color: WINDOW_FRAME_COLOR.into(),
            ..default()
        },
        WindowComponentTag::default(),
    )).id();

    let inner_window = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Px(window_size.x),
                height: Val::Px(window_size.y),
                margin: UiRect::top(Val::Px(WINDOW_FRAME_OFFSET / 2.)),
                ..default()
            },
            background_color: WINDOW_INNER_COLOR.into(),
            ..default()
        }
    ).id();

    let title_bar = spawn_title_bar_elements_helper(commands, window_name);

    commands.entity(window_container).push_children(&[title_bar, inner_window]);

    return inner_window;
}

fn spawn_title_bar_elements_helper(
    commands: &mut Commands,
    text: &str,
) -> Entity {

    let close_button = commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(CLOSE_BUTTON_SIZE),
                height: Val::Px(CLOSE_BUTTON_SIZE),
                ..default()
            },
            ..default()
        },
        ButtonColors {
            normal: Color::linear_rgb(1., 0., 0.),
            hovered: Color::linear_rgb(1., 0.2, 0.2)
        },
        WindowCloseButton,
    )).id();

    let title = commands.spawn(
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: CLOSE_BUTTON_SIZE,
                color: WINDOW_INNER_COLOR,
                ..default()
            }
        )
    ).id();

    let title_bar = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(CLOSE_BUTTON_SIZE),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }
    ).id();

    commands.entity(title_bar).push_children(&[title, close_button]);

    return title_bar;
}