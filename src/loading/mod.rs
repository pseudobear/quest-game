pub mod swordsmaster;
pub mod test_enemy;
pub mod ui;

use crate::GameState;
use swordsmaster::SwordsMasterSpriteAssets;
use test_enemy::TestEnemySpriteAssets;
use ui::UiSpriteAssets;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
/// Loading state is set on launch, we load everythjing at the start of the game
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<MapAssets>()
                // sprites
                .load_collection::<SwordsMasterSpriteAssets>()
                .load_collection::<TestEnemySpriteAssets>()
                // ui assets
                .load_collection::<UiSpriteAssets>()
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct MapAssets {
    #[asset(path = "maps/testing.ldtk")]
    pub testing_map: Handle<LdtkProject>,
}
