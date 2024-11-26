mod inventory;

use std::collections::HashMap;
use crate::gameplay::ui::windows::inventory::open_inventory;
use bevy::prelude::*;
use bevy::ecs::system::SystemId;


#[derive(Resource, Deref, DerefMut)]
pub struct UiWindowSystems(pub HashMap<String, SystemId>);

impl FromWorld for UiWindowSystems {
    fn from_world(world: &mut World) -> Self {
        let mut ui_window_systems = UiWindowSystems(HashMap::new());

        ui_window_systems.insert(
            "open_inventory".into(),
            world.register_system(open_inventory)
        );

        return ui_window_systems;
    }
}

pub struct UiWindowsPlugin;

impl Plugin for UiWindowsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiWindowSystems>();
    }
}