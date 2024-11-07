mod actions;
mod player;
mod audio;
use crate::GameState;
use bevy::prelude::*;
use crate::gameplay::player::PlayerPlugin;
use crate::gameplay::actions::ActionsPlugin;
use crate::gameplay::audio::InternalAudioPlugin;

pub struct GameplayPlugin;

#[derive(Component)]
struct Gameplay;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
        ))
           .add_systems(OnEnter(GameState::Playing), setup_gameplay)
           .add_systems(OnExit(GameState::Playing), cleanup_gameplay);
    }
}

fn setup_gameplay(mut commands: Commands) {
    info!("gameplay");
    commands.spawn((Camera2dBundle::default(), Gameplay));
}

fn cleanup_gameplay(mut commands: Commands, gameplay: Query<Entity, With<Gameplay>>) {
    for entity in gameplay.iter() {
        commands.entity(entity).despawn_recursive();
    }
}