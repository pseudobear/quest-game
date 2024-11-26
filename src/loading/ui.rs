use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// sprites
#[derive(AssetCollection, Resource)]
pub struct UiSpriteAssets {
    #[asset(path = "ui/white_x.png")] 
    pub image: Handle<Image>,
}