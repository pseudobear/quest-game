use crate::gameplay::actions::Actions;
use crate::gameplay::player::Player;
use bevy::prelude::*;


// ToDo: given player input & player ground state & animation states, do things to the rigid body
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