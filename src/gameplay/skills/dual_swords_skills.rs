use crate::gameplay::skills::events::ActivateSkillEvent;
use crate::gameplay::GameState;
use crate::gameplay::hitbox::{
    HitboxThrower,
    HitboxConfig,
};
use crate::gameplay::hitbox::hitbox_frame::{
    HitboxFrame,
    CuboidColliderSpec,
};
use crate::animations::Animatable;
use bevy::prelude::*;


pub struct DualSwordSkillsPlugin;

impl Plugin for DualSwordSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                ds_activate_basic_attack
            ).run_if(in_state(GameState::Playing))
        );
    }
}

/* Hitboxes List:
0: slash_1 (basic attack)
*/
pub fn create_dual_swords_hitbox_thrower() -> HitboxThrower {

    let basic_attack_config = HitboxConfig::new(0, 6, 1, false, ds_basic_attack_frames());

    HitboxThrower::new(Vec::from([
        basic_attack_config
    ]))
}


fn ds_basic_attack_frames() -> Vec<HitboxFrame> {
    Vec::from([
        HitboxFrame::new(1, 2, Vec::from([CuboidColliderSpec::new(
            24.0, 8.0,
            14.0, -8.0
        )])),
    ])
}

// ToDo: make these systems run after whatever is writing the event to avoid frame gap

fn ds_activate_basic_attack (
    mut ev_activate_skill: EventReader<ActivateSkillEvent>,
    mut animatable_query: Query<&mut Animatable>,
    mut hitbox_thrower_query: Query<&mut HitboxThrower>
) {
    for ev in ev_activate_skill.read() {
        if ev.skill != "basic_attack" {
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