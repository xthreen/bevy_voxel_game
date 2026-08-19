use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

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

// A local tracker to keep our state unwrapped between frames
#[derive(Default)]
struct CompassTracker {
    last_camera_yaw: f32,
    accumulated_yaw: f32,
    initialized: bool,
}

fn update(
    mut mat: ResMut<Assets<CompassMaterial>>,
    camera_query: Query<&Transform, With<Camera3d>>,
    handle_query: Query<&MaterialNode<CompassMaterial>>,
    time: Res<Time>,
    mut tracker: Local<CompassTracker>, // Injects our persistent state tracking
) {
    for transform in &camera_query {
        let current_yaw = transform.rotation.to_euler(EulerRot::YXZ).0;

        // On the very first frame, snap our tracker to the current camera yaw
        // so the compass doesn't wildly spin to catch up.
        if !tracker.initialized {
            tracker.last_camera_yaw = current_yaw;
            tracker.accumulated_yaw = current_yaw;
            tracker.initialized = true;
        }

        // 1. Calculate how much the camera rotated this frame
        let mut delta = current_yaw - tracker.last_camera_yaw;

        // 2. Force the delta to be the shortest path (handle wrapping from -PI to PI)
        if delta > PI {
            delta -= TAU;
        } else if delta < -PI {
            delta += TAU;
        }

        // 3. Accumulate this raw change into our continuous target
        tracker.accumulated_yaw += delta;
        tracker.last_camera_yaw = current_yaw;

        // 4. Update the shader for all compass handles
        for handle in &handle_query {
            if let Some(shader) = mat.get_mut(handle) {
                let lerp_weight = 1.0 - (-10.0 * time.delta_secs()).exp();

                // 5. Standard linear lerp. No wrapping logic here!
                shader.dir = shader.dir + (tracker.accumulated_yaw - shader.dir) * lerp_weight;

                // 6. Optional: floating point precision safety
                // If a player spins in a circle 10,000 times, the float might lose precision.
                // We shift both the target and the current visual dir back by TAU to stay near 0.
                if tracker.accumulated_yaw.abs() > 1000.0 {
                    let shift = (tracker.accumulated_yaw / TAU).floor() * TAU;
                    tracker.accumulated_yaw -= shift;
                    shader.dir -= shift;
                }
            }
        }
    }
}
