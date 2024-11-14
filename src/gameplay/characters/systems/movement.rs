use crate::gameplay::inputs::PlayerActions;
use crate::gameplay::characters::components::{
    CharacterPhysics,
    GroundStatus,
};
use crate::gameplay::maps::Ground;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;


pub const MINIMUM_MOVEMENT: f32 = 0.2;

const MAX_VELOCITY: f32 = 500.0;
const MAX_VELOCITY_SQUARED: f32 = 250_000.0;

const MAX_GROUNDED_VELOCITY: f32 = 100.0;
pub const MAX_GROUNDED_VELOCITY_SQUARED: f32 = 10_000.0;

pub fn player_grounded_movement(
    actions: Res<PlayerActions>,
    mut player_query: Query<(&mut ExternalImpulse, &Velocity), With<CharacterPhysics>>,
) {
    for (mut external_impulse, velocity) in &mut player_query {

        // Handle no directional input
        if actions.player_input.is_none() {
            // slow down in opposite direction 
            external_impulse.impulse = -(velocity.linvel * 60.0);
        
        // Handle directional input if under velocity limit
        } else if velocity.linvel.length_squared() < MAX_GROUNDED_VELOCITY_SQUARED {
            external_impulse.impulse = actions.player_input.unwrap() * 6_000.0;

        // Apply velocity limit
        } else {
            external_impulse.impulse = Vec2::ZERO;
        }

        external_impulse.impulse.y = 0.0;

        // Handle jump
        if actions.jump {
            external_impulse.impulse += Vec2::Y * 80_000.0;
        }
    }
}

pub fn player_air_movement(
    actions: Res<PlayerActions>,
    mut player_query: Query<(&mut ExternalImpulse, &mut ExternalForce), With<CharacterPhysics>>,
) {
    for (mut external_impulse, mut external_force) in &mut player_query {
        external_impulse.impulse = Vec2::ZERO;

        // Handle no directional input
        if actions.player_input.is_none() {
            external_force.force = Vec2::ZERO;
        } else {
            external_force.force = actions.player_input.unwrap() * 40_000.0;
        }
    }
}

pub fn limit_velocity(mut player_query: Query<&mut Velocity, With<CharacterPhysics>>) {
    for mut velocity in &mut player_query {
        if velocity.linvel.length_squared() >= MAX_VELOCITY_SQUARED {
            velocity.linvel = velocity.linvel.normalize() * MAX_VELOCITY;
        }
    }
}

pub fn detect_grounded(
    mut player_query: Query<(&CollidingEntities, &mut Damping, &mut GroundStatus), With<CharacterPhysics>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    for (colliding_entities, mut damping, mut ground_state) in &mut player_query {

        *ground_state = GroundStatus::Air;
        damping.linear_damping = 1.0;

        for entity in colliding_entities.iter() {
            if ground_query.contains(entity) {
                *ground_state = GroundStatus::Grounded;
                damping.linear_damping = 3.0;
                break;
            }
        }
    }
}