pub mod buttons;
pub mod bars;
pub mod windows;

use crate::ui::buttons::UiButtonPlugin;
use crate::ui::bars::UiBarPlugin;
use crate::ui::windows::UiWindowPlugin;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiButtonPlugin,
            UiBarPlugin,
            UiWindowPlugin,
        ));
    }
}
