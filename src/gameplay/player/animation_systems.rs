use crate::gameplay::actions::Actions;
use crate::animations::Animatable;
use crate::gameplay::player::movement_systems::{
    MAX_GROUNDED_VELOCITY_SQUARED,
    MINIMUM_MOVEMENT,
};
use crate::gameplay::player::{
    Player,
    PlayerSprite,
    PlayerAnimationState,
};
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

pub const WALK_VELOCITY_PERCENTAGE: f32 = 0.5;

pub fn idle_animation(
    mut next_state: ResMut<NextState<PlayerAnimationState>>,
    actions: Res<Actions>,
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<Player>>,
) {
    for velocity in velocity_query.iter() {
        for mut animatable in &mut animatable_query {
            if velocity.linvel.length_squared() <= MINIMUM_MOVEMENT && 
            animatable.active.unwrap_or(1) != 0 &&
                !actions.jump {

                // play idle animation
                animatable.trigger_animation(0);

                next_state.set(PlayerAnimationState::Free);
            }
        }
    }
}

pub fn walk_animation(
    mut next_state: ResMut<NextState<PlayerAnimationState>>,
    actions: Res<Actions>,
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<Player>>,
) {
    for velocity in velocity_query.iter() {
        for mut animatable in &mut animatable_query {
            if velocity.linvel.length_squared() > MINIMUM_MOVEMENT && 
                velocity.linvel.length_squared() < MAX_GROUNDED_VELOCITY_SQUARED * WALK_VELOCITY_PERCENTAGE && 
                animatable.active.unwrap_or(0) != 1 &&
                !actions.jump {

                // play walk animation
                animatable.trigger_animation(1);

                next_state.set(PlayerAnimationState::Free);
            }
        }
    }
}

pub fn run_animation(
    mut next_state: ResMut<NextState<PlayerAnimationState>>,
    actions: Res<Actions>,
    mut animatable_query: Query<&mut Animatable, With<PlayerSprite>>,
    velocity_query: Query<&Velocity, With<Player>>,
) {
    for velocity in velocity_query.iter() {
        for mut animatable in &mut animatable_query {
            if velocity.linvel.length_squared() >= MAX_GROUNDED_VELOCITY_SQUARED * WALK_VELOCITY_PERCENTAGE && 
                animatable.active.unwrap_or(0) != 2 &&
                !actions.jump {

                // play run animation
                animatable.trigger_animation(2);

                next_state.set(PlayerAnimationState::Free);
            }
        }
    }
}