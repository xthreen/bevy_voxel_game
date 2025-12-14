use bevy::prelude::*;

use crate::{
    AppState,
    loading::{CompassMaterial, TextureAssets},
};

pub struct CompassPlugin;

impl Plugin for CompassPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Ready), setup)
            .add_systems(Update, update.run_if(in_state(AppState::Ready)));
    }
}

fn setup(
    mut commands: Commands,
    mut compass_mat: ResMut<Assets<CompassMaterial>>,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            width: Val::Vw(100.0),
            height: Val::Px(38.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(1080.0),
                    height: Val::Px(38.0),
                    ..default()
                },
                MaterialNode(compass_mat.add(CompassMaterial {
                    texture: textures.compass.clone(),
                    north: -1.570_796_4,
                    dir: 0.0,
                    alpha: 0.8,
                    tau: TAU,
                    fade_width: 0.336,
                })),
            ));
        });
}

fn update(
    mut mat: ResMut<Assets<CompassMaterial>>,
    camera_query: Query<&Transform, With<Camera3d>>,
    handle_query: Query<&MaterialNode<CompassMaterial>>,
    time: Res<Time>,
) {
    for transform in &camera_query {
        for handle in &handle_query {
            if let Some(shader) = mat.get_mut(handle) {
                let lerp_weight = 1.0 - (-10.0 * time.delta_secs()).exp();
                shader.dir = lerp_angle(
                    shader.dir,
                    transform.rotation.to_euler(EulerRot::YXZ).0,
                    lerp_weight,
                );
            }
        }
    }
}

use std::f32::consts::{PI, TAU};

/// Linearly interpolates between two angles in radians, finding the shortest path.
///
/// - `start`: The starting angle in radians.
/// - `end`: The target angle in radians.
/// - `weight`: The interpolation factor (typically between 0.0 and 1.0).
pub fn lerp_angle(start: f32, end: f32, weight: f32) -> f32 {
    let mut diff = end - start;

    // Wrap the difference to be between -PI and PI
    if diff > PI {
        diff -= TAU;
    }
    if diff < -PI {
        diff += TAU;
    }

    start + diff * weight
}
