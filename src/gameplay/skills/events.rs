use crate::gameplay::skills::SkillCooldowns;
use crate::gameplay::hitbox::HitboxThrower;
use crate::animations::Animatable;
use bevy::prelude::*;


#[derive(Event)]
pub struct ActivateSkillEvent {
    pub physics_entity: Entity,
    pub sprite_entity: Entity, 
    pub skill: String,
    pub animation_index: usize,
    pub animation_lock: bool,
    pub hitbox_index: usize,
    pub hitbox_lock: bool,
}

#[derive(Event)]
pub struct EndSkillEvent {
    pub physics_entity: Entity,
    pub sprite_entity: Entity,
    pub skill: String,
}

pub fn activate_skill (
    mut ev_activate_skill: EventReader<ActivateSkillEvent>,
    mut animatable_query: Query<&mut Animatable>,
    mut hitbox_thrower_query: Query<&mut HitboxThrower>
) {
    for ev in ev_activate_skill.read() {
        let mut animatable = animatable_query.get_mut(ev.sprite_entity).unwrap();
        animatable.trigger_animation(ev.animation_index, ev.animation_lock);
        
        // throw the sensor colliders
        let mut hitbox_thrower = hitbox_thrower_query.get_mut(ev.sprite_entity).unwrap();
        hitbox_thrower.trigger_hitbox(ev.hitbox_index, ev.hitbox_lock);
    }
}

pub fn end_skill(
    mut ev_end_skill: EventReader<EndSkillEvent>,
    mut skill_cooldowns_query: Query<&mut SkillCooldowns>
) {
    for ev in ev_end_skill.read() {
        skill_cooldowns_query.get_mut(ev.sprite_entity).unwrap().add_cooldown(ev.skill.as_str(), 1.0);
    }
}