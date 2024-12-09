mod emitters;
pub mod hitbox;

use crate::gameplay::GameState;
use crate::gameplay::characters::player::{ PlayerPhysics, PlayerSprite };
use crate::gameplay::skills::dual_swords_skills::emitters::emit_ds_skill_activation;
use crate::gameplay::skills::events::{
    activate_skill,
    end_skill,
};
use bevy::prelude::*;


pub struct DualSwordSkillsPlugin;

impl Plugin for DualSwordSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
                emit_ds_skill_activation::<PlayerPhysics, PlayerSprite>,
                activate_skill,
                end_skill,
            ).run_if(in_state(GameState::Playing)));
    }
}
