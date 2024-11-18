use bevy::prelude::*;
use crate::gameplay::GameState;


#[derive(Component)]
pub struct GameUiBottomBar;

pub struct BottomBarPlugin;

// This plugin is responsible to control the game audio
impl Plugin for BottomBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_bottom_bar);
        app.add_systems(OnExit(GameState::Playing), cleanup_bottom_bar);
    }
}

fn setup_bottom_bar() {}

fn cleanup_bottom_bar(mut commands: Commands, bottom_bar: Query<Entity, With<GameUiBottomBar>>) {
    for entity in bottom_bar.iter() {
        commands.entity(entity).despawn_recursive();
    }
}