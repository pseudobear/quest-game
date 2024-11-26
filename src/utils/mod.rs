use bevy::prelude::*;
use bevy::ecs::system::SystemId;


#[derive(Component, Deref, DerefMut)]
pub struct OneShotCallback(pub SystemId);

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_one_shot_callbacks_on_button);
    }
}

fn trigger_one_shot_callbacks_on_button(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &OneShotCallback), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, callback) in interaction_query.iter() {
        match *interaction {
            Interaction::Pressed => {
                commands.run_system(**callback);
            }
            _ => {}
        }
    }
}