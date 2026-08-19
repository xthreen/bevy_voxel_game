use core::time::Duration;

use bevy::{
    asset::Handle,
    color::Color,
    ecs::{
        component::Component,
        resource::Resource,
        system::{Res, ResMut},
    },
    text::{Font, TextFont},
    transform::components::{GlobalTransform, Transform},
    utils::default,
};

use crate::{
    fly_controller::{ControllerSpatialData, FlyController},
    ui::{OverlayColor, TextOptions},
};

#[derive(Resource, Component, Clone)]
pub struct DebugUiConfig {
    pub text_config: TextFont,
    pub text_color: Color,
    pub refresh_interval: f32,
    pub show_camera_data: bool,
    pub show_chunk_data: bool,
    pub show_voxel_data: bool,
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub speed: f32,
}

impl Default for DebugUiConfig {
    fn default() -> Self {
        Self {
            text_config: TextFont {
                font: Handle::<Font>::default(),
                font_size: TextOptions::DATA_TEXT_SIZE,
                ..default()
            },
            text_color: OverlayColor::GREEN,
            show_camera_data: false,
            show_chunk_data: false,
            show_voxel_data: false,
            refresh_interval: Duration::from_millis(100).as_secs_f32(),
            global_transform: GlobalTransform::IDENTITY,
            transform: Transform::IDENTITY,
            speed: FlyController::SPEED_INITIAL,
        }
    }
}

pub fn update_debug_ui_config_spatial_data(
    mut config: ResMut<DebugUiConfig>,
    spatial_data: Res<ControllerSpatialData>,
) {
    config.global_transform = spatial_data.global_transform;
    config.transform = spatial_data.transform;
    config.speed = spatial_data.speed;
}
