use bevy::prelude::*;
use crate::gameplay::GameState;
use crate::gameplay::ui::setup_game_ui;
use crate::gameplay::ui::button::{
    ui_button,
    ButtonColors,
    ui_button_text,
};


#[derive(Component)]
pub struct GameUiBottomBar;

pub struct BottomBarPlugin;

// This plugin is responsible to control the game audio
impl Plugin for BottomBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_bottom_bar.after(setup_game_ui));
        app.add_systems(OnExit(GameState::Playing), cleanup_bottom_bar);
    }
}

fn setup_bottom_bar(mut commands: Commands, bottom_bar: Query<Entity, With<GameUiBottomBar>>) {
    let test_button = commands.spawn((
        ui_button(140.0, 50.0),
        ButtonColors::default(),
    )).id();
    commands.entity(bottom_bar.single()).add_child(test_button);
}

fn cleanup_bottom_bar(mut commands: Commands, bottom_bar: Query<Entity, With<GameUiBottomBar>>) {
    for entity in bottom_bar.iter() {
        commands.entity(entity).despawn_recursive();
    }
}