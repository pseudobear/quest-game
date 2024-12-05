mod systems;
mod testing_map;

use crate::GameState;
use crate::gameplay::maps::testing_map::TestingMapPlugin;
use crate::gameplay::maps::systems::{
    spawn_wall_collision,
    spawn_oob_colliders
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;


const MAP_TILE_SIZE: i32 = 16;

/// The 'map' that is currently active 
/// There is a default, but we will also set this on setup based on a resource
#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing)]
enum LocationState {
    #[default]
    TestingMap,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component, Default)]
struct Obstacle;

#[derive(Bundle, Default, LdtkIntCell)]
pub struct ObstacleBundle {
    obstacle: Obstacle
}

#[derive(Component)]
struct Maps;

pub struct MapsPlugin;

/// This plugin is a launching point for all maps, provides map related plugins & entities like camera bundle
impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<LocationState>()
           .add_plugins((
                LdtkPlugin,
                TestingMapPlugin,
            ))
           .add_systems(OnEnter(GameState::Playing), setup_maps)
           .add_systems(Update, (
                spawn_wall_collision,
                spawn_oob_colliders
           ).run_if(in_state(GameState::Playing)))
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