use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Hitbox {
    // exclusive, despawn should occur on the frame 
    expiry_index: usize
}

#[derive(Clone, Debug)]
pub struct HitboxFrame {
    // (inclusive, exclusive) for start/stop respectively
    index_start: usize,
    index_stop: usize,
    sensor_colliders: Collider,
}