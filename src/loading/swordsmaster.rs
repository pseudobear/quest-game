use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

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
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 8,
        rows = 1,
        offset_x = 0,
        offset_y = 74
    ))]
    pub walk: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 8,
        rows = 1,
        offset_x = 0,
        offset_y = 111
    ))]
    pub run: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 8,
        rows = 1,
        offset_x = 0,
        offset_y = 148
    ))]
    pub run_fast: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 7,
        rows = 1,
        offset_x = 0,
        offset_y = 259
    ))]
    pub slash_0: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 5,
        rows = 1,
        offset_x = 0,
        offset_y = 296
    ))]
    pub slash_1: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 5,
        rows = 1,
        offset_x = 0,
        offset_y = 333
    ))]
    pub slash_2: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 6,
        rows = 1,
        offset_x = 0,
        offset_y = 370
    ))]
    pub slash_3: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 3,
        rows = 1,
        offset_x = 0,
        offset_y = 481
    ))]
    pub jump: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 4,
        rows = 1,
        offset_x = 0,
        offset_y = 518
    ))]
    pub jump_fall_transition: Handle<TextureAtlasLayout>,
    #[asset(texture_atlas_layout(
        tile_size_x = 90,
        tile_size_y = 37,
        columns = 3,
        rows = 1,
        offset_x = 0,
        offset_y = 555
    ))]
    pub fall: Handle<TextureAtlasLayout>,
}
