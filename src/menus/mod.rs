pub mod start_menu;
mod button;

use bevy::prelude::*;

enum MenuState {
    StartMenu,
    
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
    }
}



#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(-1.15, 0.15, 0.15),
            hovered: Color::linear_rgb(-1.25, 0.25, 0.25),
        }
    }
}