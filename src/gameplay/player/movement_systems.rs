use crate::gameplay::actions::Actions;
use crate::gameplay::player::Player;
use crate::gameplay::player::PlayerGroundState;
use crate::gameplay::maps::Ground;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;


const MAX_VELOCITY: f32 = 500.0;
const MAX_VELOCITY_SQUARED: f32 = 250_000.0;

const MAX_GROUNDED_VELOCITY: f32 = 150.0;
const MAX_GROUNDED_VELOCITY_SQUARED: f32 = 22_500.0;

pub fn grounded_movement(
    actions: Res<Actions>,
    mut player_query: Query<(&mut ExternalImpulse, &Velocity), With<Player>>,
) {
    for (mut external_impulse, velocity) in &mut player_query {

        if actions.player_input.is_none() {
            // slow down in opposite direction 
            external_impulse.impulse = -(velocity.linvel * 60.0);
            continue;
        } 

        if velocity.linvel.length_squared() < MAX_GROUNDED_VELOCITY_SQUARED {
            external_impulse.impulse = actions.player_input.unwrap() * 6_000.0;
        } else {
            external_impulse.impulse = Vec2::ZERO;
        }

        // println!("impulse: {},{}", external_impulse.impulse.x, external_impulse.impulse.y);
    }
}

pub fn limit_velocity(mut player_query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in &mut player_query {
        if velocity.linvel.length_squared() >= MAX_VELOCITY_SQUARED {
            velocity.linvel = velocity.linvel.normalize() * MAX_VELOCITY;
        }
    }
}

pub fn detect_grounded(
    mut next_state: ResMut<NextState<PlayerGroundState>>,
    player_query: Query<&CollidingEntities, With<Player>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    for colliding_entities in player_query.iter() {

        next_state.set(PlayerGroundState::Air);

        for entity in colliding_entities.iter() {
            if ground_query.contains(entity) {
                next_state.set(PlayerGroundState::Grounded);
                break;
            }
        }
    }
}