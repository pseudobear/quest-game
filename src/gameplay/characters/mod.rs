mod systems;
mod stats;
pub mod enemies;
pub mod player;
pub mod components;

use crate::gameplay::characters::player::PlayerPlugin;
use crate::gameplay::characters::enemies::EnemyPlugin;
use crate::gameplay::characters::systems::movement::{
    detect_grounded,
    limit_velocity,
};
use crate::gameplay::characters::systems::animation::{
    idle_animation,
    walk_animation,
    run_animation,
    jump_animation,
    fall_animation,
    jump_fall_transition_animation,
    air_turn_character,
    grounded_turn_character,
};
use crate::GameState;
use bevy::prelude::*;

pub struct CharacterPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            EnemyPlugin,
        ))
           .add_systems(Update, 
            (
                detect_grounded,
                limit_velocity,
                (   // grounded animation systems
                    idle_animation,
                    walk_animation,
                    run_animation,
                    grounded_turn_character,
                ),
                (   // air animation systems
                    jump_animation,
                    jump_fall_transition_animation,
                    fall_animation,
                    air_turn_character,
                )
            ).run_if(in_state(GameState::Playing))
        );
    }
}