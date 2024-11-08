use bevy::prelude::*;

#[derive(Resource)]
pub struct ScreenBottomLeft {
    pub x: u32,
    pub y: u32,
}

// this default is based on standard camera for 512x288 stage given 1280x720 screen
impl Default for ScreenBottomLeft { 
    fn default() -> Self {
        ScreenBottomLeft{x: 1280 / 5, y: 720 / 5}
    }
}


