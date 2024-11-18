use bevy::prelude::*;
use crate::gameplay::GameState;


#[derive(Component)]
pub struct GameUiTopBar;

pub struct TopBarPlugin;

// This plugin is responsible to control the game audio
impl Plugin for TopBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_top_bar);
        app.add_systems(OnExit(GameState::Playing), cleanup_top_bar);
    }
}

fn setup_top_bar() {}

fn cleanup_top_bar(mut commands: Commands, top_bar: Query<Entity, With<GameUiTopBar>>) {
    for entity in top_bar.iter() {
        commands.entity(entity).despawn_recursive();
    }
}