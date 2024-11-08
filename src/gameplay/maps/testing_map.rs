use crate::gameplay::maps::LocationState;
use crate::gameplay::GameCamera;
use crate::loading::MapAssets;
use crate::gameplay::maps::ObstacleBundle;
use bevy_ecs_ldtk::{app::LdtkIntCellAppExt, LdtkWorldBundle, LevelSelection};
use bevy::prelude::*;


#[derive(Component)]
struct TestingMap;

pub struct TestingMapPlugin;

impl Plugin for TestingMapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
           .add_systems(OnEnter(LocationState::TestingMap), setup_testing_map)
           .add_systems(OnExit(LocationState::TestingMap), cleanup_testing_map)
           .register_ldtk_int_cell::<ObstacleBundle>(1);
    }
}

// Exclusive system: ensure we spawn wall collisions after this synchronously
fn setup_testing_map(
    world: &mut World, 
    game_camera_transform: &mut QueryState<&mut Transform, With<GameCamera>>,
    game_camera_projection: &mut QueryState<&mut OrthographicProjection, With<GameCamera>>,
) {
    info!("testing map");

    {
        let mut camera_projection = game_camera_projection.single_mut(world);
        camera_projection.scale = 0.4;

        let mut camera_transform = game_camera_transform.single_mut(world);
        camera_transform.translation.x += 1280.0 / 5.0;
        camera_transform.translation.y += 720.0 / 5.0;
    }

    if let Some(maps) = world.get_resource::<MapAssets>() {
        world.spawn(LdtkWorldBundle {
            ldtk_handle: maps.testing_map.clone(),
            ..Default::default()
        });
    } else {
        panic!("Tried to setup testing map, but no MapAssets were found");
    }
}

fn cleanup_testing_map(mut commands:  Commands, testing_map: Query<Entity, With<TestingMap>>) {
    for entity in testing_map.iter() {
        commands.entity(entity).despawn_recursive();
    }
}