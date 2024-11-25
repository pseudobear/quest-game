
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// sprites
#[derive(AssetCollection, Resource)]
pub struct TestEnemySpriteAssets {
    #[asset(path = "enemies/test_enemy/sheet.png")] 
    pub sheet: Handle<Image>,  // total (rows, cols): (3, 8)
    #[asset(texture_atlas_layout(
        tile_size_x = 100,
        tile_size_y = 100,
        columns = 8,
        rows = 1,
        offset_x = 0,
        offset_y = 0
    ))]
    pub float: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 100,
        tile_size_y = 100,
        columns = 6,
        rows = 1,
        offset_x = 0,
        offset_y = 37
    ))]
    pub slash_1: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 100,
        tile_size_y = 100,
        columns = 6,
        rows = 1,
        offset_x = 0,
        offset_y = 74
    ))]
    pub slash_2: Handle<TextureAtlasLayout>,
}