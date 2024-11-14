use crate::gameplay::skills::events::ActivateSkillEvent;
use crate::gameplay::GameState;
use crate::gameplay::hitbox::HitboxThrower;
use crate::animations::Animatable;
use bevy::prelude::*;


pub struct FistSkillsPlugin;

impl Plugin for FistSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                fists_activate_basic_attack
            ).run_if(in_state(GameState::Playing))
        );
    }
}

/* Hitboxes List:

*/
pub fn create_fists_hitbox_thrower() -> HitboxThrower {
    let hitbox_thrower = HitboxThrower::new(Vec::from([
        // fill this out with hitbox configs, probably would want to move this to skills specifically
        // skills make their own hitbox thrower builders to be imported by anything that needs it?
    ]));

    return hitbox_thrower;
}

// ToDo: make these systems run after whatever is writing the event to avoid frame gap

fn fists_activate_basic_attack (
    mut ev_activate_skill: EventReader<ActivateSkillEvent>,
    mut animatable_query: Query<&mut Animatable>,
    mut hitbox_thrower_query: Query<&mut HitboxThrower>
) {
    for ev in ev_activate_skill.read() {
        if ev.skill != "fists_basic_attack" {
            continue;
        }

        let mut animatable = animatable_query.get_mut(ev.sprite_entity).unwrap();
        animatable.trigger_animation(ev.animation_index, ev.animation_lock);
        
        // throw the sensor colliders
        let mut hitbox_thrower = hitbox_thrower_query.get_mut(ev.sprite_entity).unwrap();
        hitbox_thrower.trigger_hitbox(ev.hitbox_index, ev.hitbox_lock);
        
        // lock the movement state to momentum 

        // emit a EndSkill event when finished
    }
}