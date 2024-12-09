mod dual_swords_skills;
mod fists_skills;
pub mod events;

use std::collections::HashMap;
use std::time::Duration;
use crate::gameplay::GameState;
use crate::gameplay::items::CharacterEquips;
use crate::gameplay::items::weapons::WeaponType;
use crate::gameplay::hitbox::HitboxThrower;
use crate::gameplay::skills::events::{
    ActivateSkillEvent,
    EndSkillEvent,
};
use crate::gameplay::skills::dual_swords_skills::DualSwordSkillsPlugin;
use crate::gameplay::skills::dual_swords_skills::hitbox::create_dual_swords_hitbox_thrower;
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
           .add_systems(Update, (
                switch_weapon_type,
                tick_skill_cooldowns,
           ).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct SkillCooldowns(pub HashMap<String, Timer>);

impl SkillCooldowns {
    /// Creates a new Cooldown with the relevant name and a new timer with a duration set
    /// in seconds. This will tick and reset independently. I
    fn add_cooldown(&mut self, skill_name: &str, duration: f32) {
        self.insert(
            skill_name.into(),
            Timer::new(Duration::from_secs_f32(duration), TimerMode::Once)
        );
    }

    fn tick(&mut self, duration: Duration) {
        for timer in self.values_mut() {
            timer.tick(duration);
        }
    }
}

fn tick_skill_cooldowns(
    time: Res<Time>,
    mut cooldowns_query: Query<&mut SkillCooldowns>
) {
    for mut cooldowns in &mut cooldowns_query {
        cooldowns.tick(time.delta());
        
        let mut finished_cooldowns = Vec::<String>::new();
        for (skill, timer) in cooldowns.iter_mut() {
            if timer.finished() {
                finished_cooldowns.push(skill.clone());
            }
        }
        for skill in finished_cooldowns {
            println!("FINISHED COOLDOWN {}", skill);
            cooldowns.remove(&skill);
        }
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