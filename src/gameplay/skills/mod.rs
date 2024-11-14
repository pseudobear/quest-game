mod dual_swords_skills;
mod fists_skills;
pub mod events;

use crate::gameplay::GameState;
use crate::gameplay::items::CharacterEquips;
use crate::gameplay::items::weapons::WeaponType;
use crate::gameplay::hitbox::HitboxThrower;
use crate::gameplay::skills::events::{
    ActivateSkillEvent,
    EndSkillEvent,
};
use crate::gameplay::skills::dual_swords_skills::{
    DualSwordSkillsPlugin,
    create_dual_swords_hitbox_thrower
};
use crate::gameplay::skills::fists_skills::{
    FistSkillsPlugin,
    create_fists_hitbox_thrower
};
use bevy::prelude::*;


pub struct SkillsPlugin;

/// This plugin is the parent of all weapon specific skill plugins
/// Each skill plugin controls the running of their own systems
impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActivateSkillEvent>()
           .add_event::<EndSkillEvent>()
           .add_plugins((
                FistSkillsPlugin,
                DualSwordSkillsPlugin,
           ))
           .add_systems(Update, switch_weapon_type.run_if(in_state(GameState::Playing)));
    }
}


// This system detects changes in weapon type and sets the appropriate hitbox thrower
// We do this here 
fn switch_weapon_type(
    changed_equips_parent_query: Query<(&CharacterEquips, &Parent), Changed<CharacterEquips>>,
    children_query: Query<&Children>, 
    mut sprite_child_query: Query<Entity, With<TextureAtlas>>,
    mut commands: Commands
) {
    for (character_equip, parent) in changed_equips_parent_query.iter() {

        // now  we have parent, need to get children of parent with PlayerSprite
        for child in children_query.iter_descendants(parent.get()) {

            // get the correct entity
            if let Ok(sprite_entity) = sprite_child_query.get_mut(child) {
                let new_hitbox_thrower = match character_equip.weapon.weapon_type {
                    WeaponType::DualSwords => create_dual_swords_hitbox_thrower(),
                    WeaponType::Fists => create_fists_hitbox_thrower(),
                };
                commands.entity(sprite_entity)
                    .remove::<HitboxThrower>()
                    .insert(new_hitbox_thrower);
            }
        }
    }
}