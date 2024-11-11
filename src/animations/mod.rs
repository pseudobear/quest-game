use std::time::Duration;
use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_animations);
    }
}


#[derive(Component)]
pub struct Animatable {
    pub animations: Vec<AnimationConfig>,
    pub active: Option<usize>,
}

impl Animatable {
    pub fn new(animations: Vec<AnimationConfig>) -> Self {
        Self {
            animations: animations,
            active: None
        }
    }

    // activates animation given by index
    pub fn trigger_animation(&mut self, index: usize) {
        self.active = Some(index);
        self.animations[index].frame_timer = AnimationConfig::timer_from_fps(self.animations[index].fps);
        self.animations[index].reset_index();
    }
}

#[derive(Clone, Debug)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    fps: u8,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    atlas_index: usize,
    frame_timer: Timer,
    repeat: bool,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8, repeat: bool, layout: Handle<TextureAtlasLayout>) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps: fps,
            atlas_layout: layout,
            atlas_index: first,
            frame_timer: Self::timer_from_fps(fps),
            repeat: repeat
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }

    fn reset_index(&mut self) {
        self.atlas_index = self.first_sprite_index;
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut Animatable, &mut TextureAtlas)>,
) {
    for (mut animatable, mut atlas) in &mut query {
        let config: &mut AnimationConfig;
        if let Some(active_idx) = animatable.active {
            config = &mut animatable.animations[active_idx];
        } else {
            return;
        }

        // set correct layout
        atlas.layout = config.atlas_layout.clone();

        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if config.atlas_index >= config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                config.reset_index();
                if config.repeat {
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                config.atlas_index += 1;
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }

        atlas.index = config.atlas_index;
    }
}
