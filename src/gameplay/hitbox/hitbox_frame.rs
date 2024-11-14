use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Hitbox {
    // despawn should occur on the this index
    pub expiry_index: usize
}

#[derive(Clone, Debug)]
pub struct HitboxFrame {
    // (inclusive, inclusive) for start/stop respectively
    pub index_start: usize,
    pub index_stop: usize,
    pub collider_specs: Vec<CuboidColliderSpec>,
}

impl HitboxFrame {
    pub fn new(start: usize, stop: usize, collider_specs: Vec<CuboidColliderSpec>) -> Self {
        Self {
            index_start: start,
            index_stop: stop,
            collider_specs: collider_specs,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CuboidColliderSpec {
    half_width: f32,
    half_height: f32,
    x: f32,
    y: f32,
    z: f32
}

impl CuboidColliderSpec {
    pub fn new(half_width: f32, half_height: f32, x: f32, y: f32) -> Self {
        Self {
            half_width: half_width,
            half_height: half_height,
            x: x,
            y: y,
            z: 0.0
        }
    }

    pub fn get_collider(&self) -> Collider {
        Collider::cuboid(self.half_width, self.half_height)
    }

    pub fn get_transform(&self) -> Transform {
        Transform::from_xyz(self.x, self.y, self.z)
    }
}