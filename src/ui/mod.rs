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
        ))
        .add_systems(Update, (
            set_dragging,
            move_dragging,
        ));
    }
}

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Dragging;

fn set_dragging(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Draggable>)>
) {
    for (interaction, entity) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                commands.entity(entity).insert(Dragging);
            }
            _ => {
                commands.entity(entity).remove::<Dragging>();
            }
        }
    }
}

fn move_dragging(
    mut dragging_query: Query<&mut Style, With<Dragging>>,
    mut evr_cursor: EventReader<CursorMoved>
) {
    if let Ok(mut style) = dragging_query.get_single_mut() {
        for ev in evr_cursor.read() {
            let cursor_delta = ev.delta.unwrap_or_default();

            if let Val::Px(left) = &mut style.left {
                *left += cursor_delta.x;
            }

            if let Val::Px(top) = &mut style.top {
                *top += cursor_delta.y;
            }
        }
    }
}