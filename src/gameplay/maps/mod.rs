mod testing_map;


use crate::GameState;
use crate::gameplay::maps::testing_map::TestingMapPlugin;
use bevy::prelude::*;

/// The 'map' that is currently active 
/// There is a default, but we will also set this on setup based on a resource
#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing)]
enum LocationState {
    #[default]
    TestingMap,
}


#[derive(Component)]
struct Maps;

pub struct MapsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LocationState>()
           .add_plugins((
                TestingMapPlugin,
            ))
           .add_systems(OnEnter(GameState::Playing), setup_maps)
           .add_systems(OnExit(GameState::Playing), cleanup_maps);
    }
}

fn setup_maps(mut commands: Commands) {
    info!("maps");
}

fn cleanup_maps(mut commands: Commands, gameplay: Query<Entity, With<Maps>>) {
    for entity in gameplay.iter() {
        commands.entity(entity).despawn_recursive();
    }
}