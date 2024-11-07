mod actions;
mod player;
mod audio;
mod maps;
use crate::GameState;
use crate::gameplay::player::PlayerPlugin;
use crate::gameplay::actions::ActionsPlugin;
use crate::gameplay::audio::InternalAudioPlugin;
use crate::gameplay::maps::MapsPlugin;
use bevy::prelude::*;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing)]
enum PauseState {
    #[default]
    NotPaused,
    IsPaused,
}

pub struct GameplayPlugin;

#[derive(Component)]
struct Gameplay;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PauseState>()
           .add_systems(OnEnter(GameState::Playing), setup_gameplay)
           .add_plugins((
                ActionsPlugin,
                InternalAudioPlugin,
                PlayerPlugin,
                MapsPlugin,
            ))
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