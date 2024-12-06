use crate::gameplay::skills::events::ActivateSkillEvent;
use crate::gameplay::hitbox::HitboxThrower;
use crate::animations::Animatable;
use bevy::prelude::*;

// ToDo: make these systems run after whatever is writing the event to avoid frame gap

pub fn ds_activate_basic_attack (
    mut ev_activate_skill: EventReader<ActivateSkillEvent>,
    mut animatable_query: Query<&mut Animatable>,
    mut hitbox_thrower_query: Query<&mut HitboxThrower>
) {
    for ev in ev_activate_skill.read() {
        if ev.skill != "ds_basic_attack" {
            continue;
        }

        let mut animatable = animatable_query.get_mut(ev.sprite_entity).unwrap();
        animatable.trigger_animation(ev.animation_index, ev.animation_lock);
        
        // throw the sensor colliders
        let mut hitbox_thrower = hitbox_thrower_query.get_mut(ev.sprite_entity).unwrap();
        hitbox_thrower.trigger_hitbox(ev.hitbox_index, ev.hitbox_lock);
        
    }
}