mod dual_swords_skills;
pub mod events;

use crate::gameplay::skills::events::ActivateSkillEvent;
use crate::gameplay::skills::dual_swords_skills::DualSwordSkillsPlugin;
use bevy::prelude::*;


pub struct SkillsPlugin;

/// This plugin is the parent of all weapon specific skill plugins
/// Each skill plugin controls the running of their own systems
impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActivateSkillEvent>()
           .add_plugins(
                DualSwordSkillsPlugin,
           );
    }
}

