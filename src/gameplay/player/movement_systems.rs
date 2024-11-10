use crate::gameplay::actions::Actions;
use crate::gameplay::player::Player;
use crate::gameplay::player::PlayerGroundState;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;


const MAX_VELOCITY: f32 = 500.0;
const MAX_VELOCITY_SQUARED: f32 = 250_000.0;

// ToDo: given player input & player ground state & animation states, do things to the rigid body
/*
pub fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_input.is_none() {
        return;
    }
    let speed = 150.0;
    let movement = Vec3::new(
        actions.player_input.unwrap().x * speed * time.delta_seconds(),
        actions.player_input.unwrap().y * speed * time.delta_seconds(),
        0.0,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
*/

pub fn grounded_movement(
    actions: Res<Actions>,
    mut player_query: Query<&mut ExternalImpulse, With<Player>>,
) {
    for (mut external_impulse) in &mut player_query {
        if actions.player_input.is_none() {
            external_impulse.impulse = Vec2::ZERO;
        } else {
            external_impulse.impulse = actions.player_input.unwrap() * 10_000.0;
        }
    }
}

pub fn limit_velocity(mut player_query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in &mut player_query {
        println!("velocity of player {}", velocity.linvel.length());
        if velocity.linvel.length_squared() >= MAX_VELOCITY_SQUARED {
            velocity.linvel = velocity.linvel.normalize() * MAX_VELOCITY;
        }
    }
}

pub fn detect_grounded(
    state: Res<State<PlayerGroundState>>,
    mut next_state: ResMut<NextState<PlayerGroundState>>,
    player_query: Query<Entity, With<Player>>
) {

}