use bevy::prelude::*;

use crate::{
    fly_controller::{ControllerSpatialData, FlyController},
    ui::OverlayColor,
};

#[derive(Component)]
pub struct VelocityBarFill;

pub fn spawn_velocity_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Percent(33.0),
                left: Val::Px(20.0),
                width: Val::Px(10.0),
                height: Val::Px(200.0),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(OverlayColor::BG_COLOR),
            BorderColor {
                top: OverlayColor::GREEN,
                right: OverlayColor::GREEN,
                bottom: OverlayColor::GREEN,
                left: OverlayColor::GREEN,
            },
        ))
        .with_children(|parent| {
            // The Inner Fill Bar
            parent.spawn((
                Node {
                    width: Val::Percent(80.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
                VelocityBarFill,
            ));
        });
}

pub fn sync_velocity_bar(
    spatial_data: Res<ControllerSpatialData>,
    mut query: Query<&mut Node, With<VelocityBarFill>>,
) {
    if !spatial_data.is_changed() {
        return;
    }

    if let Ok(mut node) = query.single_mut() {
        let percentage = (spatial_data.speed / FlyController::SPEED_MAX) * 100.0;
        node.height = Val::Percent(percentage);
    }
}
