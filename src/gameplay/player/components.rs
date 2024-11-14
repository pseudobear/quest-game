use bevy::prelude::*;


#[derive(Component, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GroundStatus {
    #[default]
    Grounded,
    Air,
}

#[derive(Component, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Facing {
    #[default]
    Right,
    Left,
}

#[derive(Component)]
pub struct CharacterAttributes;

#[derive(Component)]
pub struct CharacterPhysics;

#[derive(Component)]
pub struct CharacterSprite;

pub fn rc_grounded<T: Component>(ground_status_query: Query<&GroundStatus, With<T>>) -> bool {
    if let Ok(status) = ground_status_query.get_single() {
        *status == GroundStatus::Grounded
    } else {
        false
    }
}

pub fn rc_air<T: Component>(ground_status_query: Query<&GroundStatus, With<T>>) -> bool {
    if let Ok(status) = ground_status_query.get_single() {
        *status == GroundStatus::Air
    } else {
        false
    }
}