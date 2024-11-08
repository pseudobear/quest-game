use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
/// Loading state is also set on launch
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<MapAssets>()
                // sprites
                .load_collection::<SwordsMasterSpriteAssets>(),
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
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct MapAssets {
    #[asset(path = "maps/testing.ldtk")]
    pub testing_map: Handle<LdtkProject>,
}


// sprites
#[derive(AssetCollection, Resource)]
pub struct SwordsMasterSpriteAssets {
    #[asset(path = "player/swords/sheet.png")] 
    pub sheet: Handle<Image>,  // total (rows, cols): (27, 10)
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 9,
        rows = 1,
        offset_x = 0,
        offset_y = 37
    ))]
    pub idle: Handle<TextureAtlasLayout>,
}
