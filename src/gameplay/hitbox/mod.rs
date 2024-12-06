pub mod hitbox_frame;
use std::time::Duration;
use crate::gameplay::GameState;
use crate::gameplay::hitbox::hitbox_frame::{
    HitboxFrame,
    Hitbox,
};
use crate::gameplay::skills::events::EndSkillEvent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            execute_hitboxes
        ).run_if(in_state(GameState::Playing)));
    }
}


#[derive(Component, Debug)]
pub struct HitboxThrower {
    pub hitboxes: Vec<HitboxConfig>,
    pub active: Option<usize>,
    locked: bool,
}

impl HitboxThrower {
    pub fn new(hitboxes: Vec<HitboxConfig>) -> Self {
        Self {
            hitboxes: hitboxes,
            active: None,
            locked: false
        }
    }

    /// activates hitbox given by index
    /// if the hitbox_thrower is locked, trigger_hitbox will not do anything. Use the lock parameter
    /// to lock the hitbox_thrower during the first loop of the hitbox
    pub fn trigger_hitbox(&mut self, index: usize, lock: bool) {
        if !self.locked {
            self.active = Some(index);
            self.locked = lock;
            self.hitboxes[index].frame_timer = HitboxConfig::timer_from_fps(self.hitboxes[index].fps);
            self.hitboxes[index].reset_index();
        }
    }
}

#[derive(Clone, Debug)]
pub struct HitboxConfig {
    skill_name: String,
    first_index: usize,
    last_index: usize,
    fps: u8,
    pub hitbox_frames: Vec<HitboxFrame>,
    frame_index: usize,
    frame_timer: Timer,
    repeat: bool,
}

impl HitboxConfig {
    pub fn new(skill_name: String, first_index: usize, last_index: usize, fps: u8, repeat: bool, hitbox_frames: Vec<HitboxFrame>) -> Self {
        Self {
            skill_name: skill_name,
            first_index: first_index,
            last_index: last_index,
            fps: fps,
            hitbox_frames: hitbox_frames,
            frame_index: 0,
            frame_timer: Self::timer_from_fps(fps),
            repeat: repeat
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }

    fn reset_index(&mut self) {
        self.frame_index = self.first_index;
    }

    pub fn get_frame_for_index(&self, index: usize) -> Option<HitboxFrame> {
        for frame in self.hitbox_frames.iter() {
            if frame.index_start <= index && index <= frame.index_stop {
                return Some(frame.clone());
            }
        }
        None
    }
}

// This system bumps active HitboxConfig indices, spawn and despawn colliders appropriately
// TODO: get this working when we have multiple characters on the screen (hitboxes need ownership from hitbox thrower)
fn execute_hitboxes(
    time: Res<Time>,
    mut hitbox_thrower_query: Query<(&Parent, Entity, &mut HitboxThrower)>,
    hitbox_query: Query<(Entity, &Hitbox)>,
    mut ev_end_skill: EventWriter<EndSkillEvent>,
    mut commands: Commands
) {
    for (parent, entity, mut hitbox_thrower) in &mut hitbox_thrower_query {
        let config: &mut HitboxConfig;
        if let Some(active_idx) = hitbox_thrower.active {
            config = &mut hitbox_thrower.hitboxes[active_idx];
        } else {
            continue;
        }

        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {

            // despawn expired hitboxes
            for (entity, hitbox) in hitbox_query.iter() {
                if config.frame_index >= hitbox.expiry_index {
                    commands.entity(entity).despawn_recursive();
                }
            }

            if config.frame_index < config.last_index {

                // increment index
                config.frame_index += 1;
                config.frame_timer = HitboxConfig::timer_from_fps(config.fps);

                if let Some(current_frame) = config.get_frame_for_index(config.frame_index) {
                    for spec in current_frame.collider_specs.iter() {
                        commands.spawn((
                            Sensor,
                            spec.get_collider(),
                            spec.get_transform(),
                            ColliderMassProperties::Mass(0.0),
                            Hitbox {expiry_index: current_frame.index_stop}
                        )).set_parent(entity);
                    }
                }

            } else {

                // hitbox finished, unlock and reset (& repeat)
                if config.repeat {
                    config.reset_index();
                    config.frame_timer = HitboxConfig::timer_from_fps(config.fps);
                    ev_end_skill.send(EndSkillEvent{
                        entity: parent.get(),
                        skill: config.skill_name.clone(),
                    });
                }

                // REPEATS DO NOT WORK RIGHT NOW
                // NEED TO FIND A WAY TO HANDLE ENDING SKILL

                hitbox_thrower.active = None;
                hitbox_thrower.locked = false;
            }
        }
    }
}