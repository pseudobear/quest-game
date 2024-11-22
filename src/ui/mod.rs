pub mod buttons;
pub mod bars;

use crate::ui::buttons::UiButtonPlugin;
use crate::ui::bars::UiBarPlugin;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiButtonPlugin,
            UiBarPlugin,
        ));
    }
}
