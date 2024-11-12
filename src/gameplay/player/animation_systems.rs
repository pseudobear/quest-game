use crate::animations::Animatable;
use crate::gameplay::player::movement_systems::{
    MAX_GROUNDED_VELOCITY_SQUARED,
    MINIMUM_MOVEMENT,
};
use crate::gameplay::player::{
    PlayerPhysics,
    PlayerSprite,
    PlayerMovementState,
};
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

pub const WALK_VELOCITY_PERCENTAGE: f32 = 0.5;
pub const JUMP_FALL_TRANSITION_VELOCITY: f32 = 30.0;

pub fn idle_animation(
    mut next_state: ResMut<NextState<PlayerMovementState>>,
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<PlayerPhysics>>,
) {
    for velocity in velocity_query.iter() {
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
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<PlayerPhysics>>,
) {
    for velocity in velocity_query.iter() {
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
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<PlayerPhysics>>,
) {
    for velocity in velocity_query.iter() {
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
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<PlayerPhysics>>,
) {
    for velocity in velocity_query.iter() {
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
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<PlayerPhysics>>,
) {
    for velocity in velocity_query.iter() {
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
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<PlayerPhysics>>,
) {
    for velocity in velocity_query.iter() {
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