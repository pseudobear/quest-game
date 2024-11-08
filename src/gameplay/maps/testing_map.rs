use crate::gameplay::maps::LocationState;
use crate::gameplay::GameCamera;
use crate::loading::MapAssets;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
use bevy::prelude::*;


#[derive(Component)]
struct TestingMap;

pub struct TestingMapPlugin;

impl Plugin for TestingMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
           .add_systems(OnEnter(LocationState::TestingMap), setup_testing_map)
           .add_systems(OnExit(LocationState::TestingMap), cleanup_testing_map);
    }
}

fn setup_testing_map(
    mut commands: Commands, 
    mut game_camera_transform: Query<&mut Transform, With<GameCamera>>,
    mut game_camera_projection: Query<&mut OrthographicProjection, With<GameCamera>>,
    maps: Res<MapAssets>,
) {
    info!("testing map");
    let mut camera_transform = game_camera_transform.single_mut();
    let mut camera_projection = game_camera_projection.single_mut();

    camera_projection.scale = 0.4;
    camera_transform.translation.x += 1280.0 / 5.0;
    camera_transform.translation.y += 720.0 / 5.0;

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: maps.testing_map.clone(),
        ..Default::default()
    });
}

fn cleanup_testing_map(mut commands:  Commands, testing_map: Query<Entity, With<TestingMap>>) {
    for entity in testing_map.iter() {
        commands.entity(entity).despawn_recursive();
    }
}