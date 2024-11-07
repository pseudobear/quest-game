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


#[derive(Component)]
struct Gameplay;

#[derive(Component)]
struct GameCamera;


pub struct GameplayPlugin;

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
    let mut camera = Camera2dBundle::default();
    /*
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    */
    commands.spawn((camera, Gameplay, GameCamera));
}

fn cleanup_gameplay(mut commands: Commands, gameplay: Query<Entity, With<Gameplay>>) {
    for entity in gameplay.iter() {
        commands.entity(entity).despawn_recursive();
    }
}