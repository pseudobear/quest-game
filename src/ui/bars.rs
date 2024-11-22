use bevy::prelude::*;

pub struct UiBarPlugin;

impl Plugin for UiBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_bar);
    }
}

fn update_bar(
    bar_query: Query<(&Children, &Bar), Changed<Bar>>,
    mut active_bar_area_query: Query<(&mut Style, &ActiveBarArea)>
) {
    for (children, bar) in bar_query.iter() {
        let mut drawn_area: f32 = 0.0;
        for &child in children.iter() {
            let (mut style, active_bar_area) = active_bar_area_query.get_mut(child).unwrap();
            let (size, c) = bar.sections[active_bar_area.0];
            
            let current_area = size as f32 / bar.total_size as f32;

            if current_area + drawn_area <= bar.progress {
                style.width = Val::Percent(current_area * 100.0);
                drawn_area += current_area;
            } else if drawn_area >= bar.progress {
                style.width = Val::Px(0.0);
            } else {
                style.width = Val::Percent((bar.progress - drawn_area) * 100.0);
                drawn_area += current_area;
            }
        }
    } 
}

pub fn spawn_bar(
    commands: &mut Commands,
    bar: Bar,
    parent: Entity,
    margin_vert: Val,
    margin_hor: Val,
) {
    let bar = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(bar.dimensions.x),
                height: Val::Px(bar.dimensions.y),
                margin: UiRect::axes(margin_hor, margin_vert),
                ..default()
            },
            background_color: bar.empty_color.into(),
            ..default()
        },
        bar.clone()
    )).with_children(|parent| {

        for (index, (_, color)) in bar.sections.iter().enumerate() {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(30.0),
                        height: Val::Px(bar.dimensions.y),
                        ..default()
                    },
                    background_color: (*color).into(),
                    ..default()
                },
                ActiveBarArea(index),
            ));
        }
    }).id();
    commands.entity(parent).add_child(bar);
}

#[derive(Component)]
struct ActiveBarArea(usize);

#[derive(Component, Clone)]
pub struct Bar {
    /// The Progress
    /// a f32 between 0.0 and 1.0
    progress: f32,
    /// The Different Sections
    /// The amount is the space relative to the other Sections.
    pub sections: Vec<(u32, Color)>,
    /// Sum of all u32 in sections
    total_size: u32,
    /// The Color of the space that is not progressed to
    pub empty_color: Color,
    pub dimensions: Vec2,
}

impl Bar {
    pub fn new(dimensions: Vec2, sections: Vec<(u32, Color)>, empty_color: Color) -> Self {
        let total_size = sections.iter().fold(0, |acc, e| acc + e.0);
        Self {
            progress: 0.0,
            sections: sections,
            total_size: total_size,
            empty_color: empty_color,
            dimensions: dimensions
        }
    }

    pub fn new_single(dimensions: Vec2, bar_color: Color, empty_color: Color) -> Self {
        Self {
            progress: 0.0,
            sections: Vec::from([(1, bar_color)]),
            total_size: 1,
            empty_color: empty_color,
            dimensions: dimensions,
        }
    }

    pub fn set_progress(&mut self, amount: f32) -> &mut Self {
        self.progress = amount.clamp(0.0, 1.0);
        self
    }

    /// Returns the current progress
    pub fn get_progress(&self) -> f32 {
        self.progress
    }

    pub fn increase_progress(&mut self, amount: f32) -> &mut Self {
        self.progress += amount;
        self.progress = self.progress.clamp(0.0, 1.0);
        self
    }

    /// Resets the progress to 0.0
    pub fn reset(&mut self) -> &mut Self {
        self.progress = 0.0;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.progress <= 0.0
    }

    pub fn is_full(&self) -> bool {
        self.progress >= 1.0
    }

    pub fn clear_sections(&mut self) -> &mut Self {
        self.sections.clear();
        self.total_size = 0;
        self
    }

    pub fn add_section(&mut self, amount: u32, color: Color) -> &mut Self {
        self.sections.push((amount, color));
        self.total_size += amount;
        self
    }
}