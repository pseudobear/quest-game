use crate::animations::Animatable;
use crate::gameplay::characters::systems::movement::{
    MAX_GROUNDED_VELOCITY_SQUARED,
    MINIMUM_MOVEMENT,
};
use crate::gameplay::characters::PlayerMovementState;
use crate::gameplay::characters::components::{
    CharacterPhysics,
    CharacterSprite,
    GroundStatus,
    Facing,
};
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

pub const WALK_VELOCITY_PERCENTAGE: f32 = 0.5;
pub const JUMP_FALL_TRANSITION_VELOCITY: f32 = 30.0;

pub fn idle_animation(
    mut next_state: ResMut<NextState<PlayerMovementState>>,
    mut animatable_query: Query<&mut Animatable, With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (velocity, ground_status) in physics_query.iter() {
        if *ground_status == GroundStatus::Air {
            continue;
        }
        for mut animatable in &mut animatable_query {
            if velocity.linvel.length_squared() <= MINIMUM_MOVEMENT && animatable.active.unwrap_or(1) != 0 {

                // play idle animation
                animatable.trigger_animation(0, false);

                next_state.set(PlayerMovementState::Free);
            }
        }
    }
}

pub fn walk_animation(
    mut next_state: ResMut<NextState<PlayerMovementState>>,
    mut animatable_query: Query<&mut Animatable, With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (velocity, ground_status) in physics_query.iter() {
        if *ground_status == GroundStatus::Air {
            continue;
        }
        for mut animatable in &mut animatable_query {
            if velocity.linvel.length_squared() > MINIMUM_MOVEMENT && 
                velocity.linvel.length_squared() < MAX_GROUNDED_VELOCITY_SQUARED * WALK_VELOCITY_PERCENTAGE && 
                animatable.active.unwrap_or(0) != 1 {

                // play walk animation
                animatable.trigger_animation(1, false);

                next_state.set(PlayerMovementState::Free);
            }
        }
    }
}

pub fn run_animation(
    mut next_state: ResMut<NextState<PlayerMovementState>>,
    mut animatable_query: Query<&mut Animatable, With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (velocity, ground_status) in physics_query.iter() {
        if *ground_status == GroundStatus::Air {
            continue;
        }
        for mut animatable in &mut animatable_query {
            if velocity.linvel.length_squared() >= MAX_GROUNDED_VELOCITY_SQUARED * WALK_VELOCITY_PERCENTAGE && 
                animatable.active.unwrap_or(0) != 2 {

                // play run animation
                animatable.trigger_animation(2, false);

                next_state.set(PlayerMovementState::Free);
            }
        }
    }
}

pub fn jump_animation(
    mut next_state: ResMut<NextState<PlayerMovementState>>,
    mut animatable_query: Query<&mut Animatable, With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (velocity, ground_status) in physics_query.iter() {
        if *ground_status == GroundStatus::Grounded {
            continue;
        }
        for mut animatable in &mut animatable_query {
            if velocity.linvel.y > JUMP_FALL_TRANSITION_VELOCITY && 
                animatable.active.unwrap_or(0) != 4 {

                // play jump animation
                animatable.trigger_animation(4, false);

                next_state.set(PlayerMovementState::Free);
            }
        }
    }
}

pub fn jump_fall_transition_animation(
    mut next_state: ResMut<NextState<PlayerMovementState>>,
    mut animatable_query: Query<&mut Animatable, With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (velocity, ground_status) in physics_query.iter() {
        if *ground_status == GroundStatus::Grounded {
            continue;
        }
        for mut animatable in &mut animatable_query {
            if velocity.linvel.y < JUMP_FALL_TRANSITION_VELOCITY && 
               velocity.linvel.y > 0.0 && 
                animatable.active.unwrap_or(0) != 5 {

                // play jump animation
                animatable.trigger_animation(5, false);  // don't lock, but don't play fall while this is playing

                next_state.set(PlayerMovementState::Free);
            }
        }
    }
}

pub fn fall_animation(
    mut next_state: ResMut<NextState<PlayerMovementState>>,
    mut animatable_query: Query<&mut Animatable, With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (velocity, ground_status) in physics_query.iter() {
        if *ground_status == GroundStatus::Grounded {
            continue;
        }
        for mut animatable in &mut animatable_query {
            if velocity.linvel.y < -JUMP_FALL_TRANSITION_VELOCITY && 
                animatable.active.unwrap_or(0) != 6  &&
                animatable.active.unwrap_or(0) != 5 {

                // play jump animation
                animatable.trigger_animation(6, false);

                next_state.set(PlayerMovementState::Free);
            }
        }
    }
}

pub fn grounded_turn_character(
    mut player_facing_query: Query<(&mut Facing, &mut Transform), With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (velocity, ground_status) in physics_query.iter() {
        if *ground_status == GroundStatus::Air {
            continue;
        }
        for (mut facing, mut transform) in &mut player_facing_query {

            // Turn Right 
            if *facing == Facing::Left && velocity.linvel.x > MINIMUM_MOVEMENT {
                transform.rotation = Quat::default();
                transform.translation = Vec3::new(17.0, 3.0, 0.0);
                *facing = Facing::Right
            }

            // Turn Left
            if *facing == Facing::Right && velocity.linvel.x < -MINIMUM_MOVEMENT {
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                transform.translation = Vec3::new(-17.0, 3.0, 0.0);
                *facing = Facing::Left
            }
        }
    }
}

pub fn air_turn_character(
    mut player_facing_query: Query<(&mut Facing, &mut Transform), With<CharacterSprite>>,
    physics_query: Query<(&ExternalForce, &GroundStatus), With<CharacterPhysics>>,
) {
    for (external_force, ground_status) in physics_query.iter() {
        if *ground_status == GroundStatus::Grounded {
            continue;
        }
        for (mut facing, mut transform) in &mut player_facing_query {

            // Turn Right 
            if *facing == Facing::Left && external_force.force.x > 0.0 {
                transform.rotation = Quat::default();
                transform.translation = Vec3::new(17.0, 3.0, 0.0);
                *facing = Facing::Right
            }

            // Turn Left
            if *facing == Facing::Right && external_force.force.x < 0.0 {
                transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
                transform.translation = Vec3::new(-17.0, 3.0, 0.0);
                *facing = Facing::Left
            }
        }
    }
}