use crate::GameState;
use crate::main_menu::{
    MainMenuState,
    ChangeGameState,
    ChangeMenuState,
};
use crate::ui::buttons::{
    ui_button,
    ui_button_text,
    ButtonColors,
};
use bevy::prelude::*;

pub struct StartMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::StartMenu), setup_menu)
           .add_systems(OnExit(MainMenuState::StartMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct StartMenu;

fn setup_menu(mut commands: Commands) {
    info!("start menu");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            StartMenu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ui_button(140.0, 50.0),
                    button_colors,
                    ChangeGameState(GameState::Playing),  // attach component to specify interaction with button
                ))
                .with_children(|parent| {
                    parent.spawn(ui_button_text("play"));
                });
        })
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    ui_button(140.0, 50.0),
                    button_colors,
                    ChangeMenuState(MainMenuState::OptionsMenu),
                ))
                .with_children(|parent| {
                    parent.spawn(ui_button_text("options"));
                });
        });
}

fn cleanup_menu(mut commands: Commands, start_menu: Query<Entity, With<StartMenu>>) {
    for entity in start_menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}