use crate::animations::Animatable;
use crate::gameplay::characters::systems::movement::{
    MAX_GROUNDED_VELOCITY_SQUARED,
    MINIMUM_MOVEMENT,
};
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
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
    mut animatable_query: Query<(&mut Animatable, &Parent), With<CharacterSprite>>,
) {
    for (mut animatable, parent) in &mut animatable_query {
        let (velocity, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Air {
            continue;
        }

        if velocity.linvel.length_squared() <= MINIMUM_MOVEMENT && animatable.active.unwrap_or(1) != 0 {

            // play idle animation
            animatable.trigger_animation(0, false);
        }
    }
}

pub fn walk_animation(
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
            }
        }
    }
}

pub fn run_animation(
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
            }
        }
    }
}

pub fn jump_animation(
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
            }
        }
    }
}

pub fn jump_fall_transition_animation(
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
            }
        }
    }
}

pub fn fall_animation(
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
            }
        }
    }
}

pub fn grounded_turn_character(
    mut facing_query: Query<(&mut Facing, &mut Transform, &Parent), With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (mut facing, mut transform, parent) in &mut facing_query {

        let (velocity, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Air {
            continue;
        }

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