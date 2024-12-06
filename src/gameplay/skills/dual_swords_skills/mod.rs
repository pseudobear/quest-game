mod end_readers;
mod start_readers;
pub mod hitbox;

use crate::gameplay::GameState;
use crate::gameplay::skills::dual_swords_skills::start_readers::ds_activate_basic_attack;
use crate::gameplay::skills::dual_swords_skills::end_readers::ds_end_basic_attack;
use bevy::prelude::*;


pub struct DualSwordSkillsPlugin;

impl Plugin for DualSwordSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            ((
                ds_activate_basic_attack,
            ),
            (
                ds_end_basic_attack,
            )).run_if(in_state(GameState::Playing))
           );
    }
}
