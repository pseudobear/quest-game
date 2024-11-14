use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


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

#[derive(Bundle)]
pub struct CharacterPhysicsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub gravity_scale: GravityScale,
    pub locked_axes: LockedAxes,
    pub active_events: ActiveEvents,
    // markers to access rigidbody attributes
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub damping: Damping,
    pub velocity: Velocity,
    pub colliding_entities: CollidingEntities,
    pub ground_status: GroundStatus,
    pub character_physics: CharacterPhysics,
}

impl Default for CharacterPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule_y(6.0, 6.0),
            gravity_scale: GravityScale(2.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_events: ActiveEvents::COLLISION_EVENTS,
            // markers to access rigidbody attributes
            external_force: ExternalForce { ..Default::default() },
            external_impulse: ExternalImpulse { ..Default::default() },
            damping: Damping { ..Default::default() },
            velocity: Velocity { ..Default::default() },
            colliding_entities: CollidingEntities::default(),
            ground_status: GroundStatus::default(),
            character_physics: CharacterPhysics,
        }
    }
}