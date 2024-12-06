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
    mut animatable_query: Query<(&mut Animatable, &Parent), With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (mut animatable, parent) in &mut animatable_query {
        let (velocity, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Air {
            continue;
        }

        if velocity.linvel.length_squared() > MINIMUM_MOVEMENT && 
            velocity.linvel.length_squared() < MAX_GROUNDED_VELOCITY_SQUARED * WALK_VELOCITY_PERCENTAGE && 
            animatable.active.unwrap_or(0) != 1 {

            // play walk animation
            animatable.trigger_animation(1, false);
        }
    }
}

pub fn run_animation(
    mut animatable_query: Query<(&mut Animatable, &Parent), With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (mut animatable, parent) in &mut animatable_query {
        let (velocity, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Air {
            continue;
        }

        if velocity.linvel.length_squared() >= MAX_GROUNDED_VELOCITY_SQUARED * WALK_VELOCITY_PERCENTAGE && 
            animatable.active.unwrap_or(0) != 2 {

            // play run animation
            animatable.trigger_animation(2, false);
        }
    }
}

pub fn jump_animation(
    mut animatable_query: Query<(&mut Animatable, &Parent), With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (mut animatable, parent) in &mut animatable_query {
        let (velocity, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Grounded {
            continue;
        }

        if velocity.linvel.y > JUMP_FALL_TRANSITION_VELOCITY && 
            animatable.active.unwrap_or(0) != 4 {

            // play jump animation
            animatable.trigger_animation(4, false);
        }
    }
}

pub fn jump_fall_transition_animation(
    mut animatable_query: Query<(&mut Animatable, &Parent), With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (mut animatable, parent) in &mut animatable_query {
        let (velocity, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Grounded {
            continue;
        }

        if velocity.linvel.y < JUMP_FALL_TRANSITION_VELOCITY && 
            velocity.linvel.y > 0.0 && 
            animatable.active.unwrap_or(0) != 5 {

            // play jump animation
            animatable.trigger_animation(5, false);  // don't lock, but don't play fall while this is playing
        }
    }
}

pub fn fall_animation(
    mut animatable_query: Query<(&mut Animatable, &Parent), With<CharacterSprite>>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (mut animatable, parent) in &mut animatable_query {
        let (velocity, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Grounded {
            continue;
        }

        if velocity.linvel.y < -JUMP_FALL_TRANSITION_VELOCITY && 
            animatable.active.unwrap_or(0) != 6  &&
            animatable.active.unwrap_or(0) != 5 {

            // play jump animation
            animatable.trigger_animation(6, false);
        }
    }
}

pub fn grounded_turn_character(
    mut facing_query: Query<(&mut Facing, &mut Transform, &CharacterSprite, &Parent)>,
    physics_query: Query<(&Velocity, &GroundStatus), With<CharacterPhysics>>,
) {
    for (mut facing, mut transform, character_sprite, parent) in &mut facing_query {

        let (velocity, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Air {
            continue;
        }

        // Turn Right 
        if *facing == Facing::Left && velocity.linvel.x > MINIMUM_MOVEMENT {
            transform.rotation = Quat::default();
            transform.translation = character_sprite.centering_transform;
            *facing = Facing::Right;
        }

        // Turn Left
        if *facing == Facing::Right && velocity.linvel.x < -MINIMUM_MOVEMENT {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            transform.translation = character_sprite.centering_transform * Vec3::new(-1., 1., 1.);
            *facing = Facing::Left;
        }
    }
}

pub fn air_turn_character(
    mut facing_query: Query<(&mut Facing, &mut Transform, &CharacterSprite, &Parent)>,
    physics_query: Query<(&ExternalForce, &GroundStatus), With<CharacterPhysics>>,
) {
    for (mut facing, mut transform, character_sprite, parent) in &mut facing_query {
        let (external_force, ground_status) = physics_query.get(parent.get()).unwrap();

        if *ground_status == GroundStatus::Grounded {
            continue;
        }

        // Turn Right 
        if *facing == Facing::Left && external_force.force.x > 0.0 {
            transform.rotation = Quat::default();
            transform.translation = character_sprite.centering_transform;
            *facing = Facing::Right;
        }

        // Turn Left
        if *facing == Facing::Right && external_force.force.x < 0.0 {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            transform.translation = character_sprite.centering_transform * Vec3::new(-1., 1., 1.);
            *facing = Facing::Left;
        }
    }
}