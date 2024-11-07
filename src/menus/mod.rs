pub mod start_menu;

use bevy::prelude::*;

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