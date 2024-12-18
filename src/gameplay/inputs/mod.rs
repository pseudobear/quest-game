mod game_control_helpers;
use bevy::prelude::*;

use crate::gameplay::inputs::game_control_helpers::{get_movement, GameControl};
use crate::GameState;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerActions>().add_systems(
            Update, (
                set_player_input.run_if(in_state(GameState::Playing)),
                set_jump_input.run_if(in_state(GameState::Playing)),
            )
        );
    }
}


// this should eventually contain skill inputs too (actions will handle keymapping)
// for now, keys are hardcoded for skills
#[derive(Default, Resource)]
pub struct PlayerActions {
    pub player_input: Option<Vec2>,
    pub jump: bool,
}

pub fn set_player_input(
    mut actions: ResMut<PlayerActions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let player_input = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if player_input != Vec2::ZERO {
        actions.player_input = Some(player_input.normalize());
    } else {
        actions.player_input = None;
    }
}

pub fn set_jump_input(
    mut actions: ResMut<PlayerActions>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    actions.jump = keyboard_input.just_pressed(KeyCode::Space);
}
