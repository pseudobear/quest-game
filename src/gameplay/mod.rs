mod inputs;
mod characters;
mod audio;
mod maps;
mod resources;
mod items;
mod skills;
mod hitbox;
mod ui;
use crate::GameState;
use crate::gameplay::characters::CharacterPlugin;
use crate::gameplay::inputs::ActionsPlugin;
use crate::gameplay::audio::InternalAudioPlugin;
use crate::gameplay::maps::MapsPlugin;
use crate::gameplay::resources::ScreenBottomLeft;
use crate::gameplay::items::ItemsPlugin;
use crate::gameplay::skills::SkillsPlugin;
use crate::gameplay::hitbox::HitboxPlugin;
use crate::gameplay::ui::GameUiPlugin;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct Gameplay;

#[derive(Component)]
struct GameCamera;


pub struct GameplayPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenBottomLeft { ..Default::default() })
           .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(35.0))  // account for scaling, x2.5
           //.add_plugins(RapierDebugRenderPlugin::default())
           .add_systems(OnEnter(GameState::Playing), setup_gameplay)
           .add_plugins((
                ActionsPlugin,
                InternalAudioPlugin,
                CharacterPlugin,
                MapsPlugin,
                ItemsPlugin,
                SkillsPlugin,
                HitboxPlugin,
                GameUiPlugin,
            ))
           .add_systems(OnExit(GameState::Playing), cleanup_gameplay);
    }
}

fn setup_gameplay(mut commands: Commands) {
    info!("gameplay");
    let camera = Camera2dBundle::default();

    commands.spawn((camera, Gameplay, GameCamera));
}

fn cleanup_gameplay(mut commands: Commands, gameplay: Query<Entity, With<Gameplay>>) {
    for entity in gameplay.iter() {
        commands.entity(entity).despawn_recursive();
    }
}