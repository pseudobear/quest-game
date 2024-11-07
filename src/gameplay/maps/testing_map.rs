use crate::gameplay::maps::LocationState;
use bevy::prelude::*;


#[derive(Component)]
struct TestingMap;

pub struct TestingMapPlugin;

impl Plugin for TestingMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LocationState::TestingMap), setup_testing_map)
           .add_systems(OnExit(LocationState::TestingMap), cleanup_testing_map);
    }
}

fn setup_testing_map(mut commands:  Commands) {
    info!("testing map");
}

fn cleanup_testing_map(mut commands:  Commands, testing_map: Query<Entity, With<TestingMap>>) {
    for entity in testing_map.iter() {
        commands.entity(entity).despawn_recursive();
    }
}