mod hitbox_frame;
use std::time::Duration;
use crate::gameplay::hitbox::hitbox_frame::HitboxFrame;
use bevy::prelude::*;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_hitboxes);
    }
}


#[derive(Component)]
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
    fps: u8,
    pub hitbox_frames: Vec<HitboxFrame>,
    frame_index: usize,
    frame_timer: Timer,
    repeat: bool,
}

impl HitboxConfig {
    pub fn new(fps: u8, repeat: bool, hitbox_frames: Vec<HitboxFrame>) -> Self {
        Self {
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
        self.frame_index = 0;
    }
}

// This system bumps active HitboxConfig indices, spawn and despawn colliders appropriately
fn execute_hitboxes(
    time: Res<Time>,
    mut query: Query<&mut HitboxThrower>,
) {
    for mut hitbox_thrower in &mut query {
        let config: &mut HitboxConfig;
        if let Some(active_idx) = hitbox_thrower.active {
            config = &mut hitbox_thrower.hitboxes[active_idx];
        } else {
            return;
        }

        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if config.frame_index < config.hitbox_frames.len() {

                // increment index
                config.frame_index += 1;
                config.frame_timer = HitboxConfig::timer_from_fps(config.fps);

                // ToDo: set correct collider 
                //

            } else {

                // hitbox finished, unlock and reset (& repeat)
                if config.repeat {
                    config.reset_index();
                    config.frame_timer = HitboxConfig::timer_from_fps(config.fps);
                }

                // ToDo: set correct collider 
                //

                hitbox_thrower.active = None;
                hitbox_thrower.locked = false;
            }
        }
    }
}