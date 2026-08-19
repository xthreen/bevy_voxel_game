use bevy::prelude::*;

use crate::{
    AppState,
    fly_controller::ControllerSpatialData,
    loading::FontAssets,
    ui::debug::{
        camera_info_text::{
            CameraInfoTextNode, CameraInfoTextSpanNode, customize_camera_data, toggle_camera_data,
            update_camera_ui,
        },
        camera_velocity_bar::{spawn_velocity_ui, sync_velocity_bar},
        chunk_info_text::{
            ChunkInfoTextNode, ChunkInfoTextSpanNode, customize_chunk_data, toggle_chunk_data,
            update_chunk_data_text,
        },
        debug_ui_config::{DebugUiConfig, update_debug_ui_config_spatial_data},
        debug_ui_root::{DebugUiNode, debug_ui_toggle_actions, toggle_debug_ui_root},
        gizmo::update_current_chunk_gizmo,
        voxel_info_text::{
            VoxelInfoTextNode, VoxelInfoTextSpanNode, customize_voxel_data, toggle_voxel_data,
            update_voxel_ui_text,
        },
    },
};

#[derive(Default)]
pub struct DebugUiPlugin {
    config: DebugUiConfig,
}

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .add_systems(OnEnter(AppState::Ready), setup)
            .add_systems(OnEnter(AppState::Ready), spawn_velocity_ui)
            .add_systems(OnEnter(AppState::Ready), set_font)
            .add_systems(
                PreUpdate,
                (update_debug_ui_config_spatial_data
                    .run_if(resource_changed::<ControllerSpatialData>))
                .run_if(in_state(AppState::Ready)),
            )
            .add_systems(
                Update,
                (
                    (
                        update_voxel_ui_text,
                        update_current_chunk_gizmo,
                        update_chunk_data_text,
                        update_camera_ui,
                        debug_ui_toggle_actions,
                        sync_velocity_bar,
                    )
                        .run_if(in_state(AppState::Ready)),
                    (
                        customize_voxel_data,
                        customize_camera_data,
                        customize_chunk_data,
                        toggle_camera_data,
                        toggle_chunk_data,
                        toggle_debug_ui_root,
                        toggle_voxel_data,
                    )
                        .run_if(resource_changed::<DebugUiConfig>),
                ),
            );
    }
}

pub fn setup(mut commands: Commands, config: Res<DebugUiConfig>) {
    commands
        .spawn(DebugUiNode::default())
        .with_children(|children| {
            children
                .spawn(VoxelInfoTextNode::setup(&config))
                .with_child(VoxelInfoTextSpanNode::setup(&config));
            children
                .spawn(CameraInfoTextNode::setup(&config))
                .with_child(CameraInfoTextSpanNode::setup(&config));
            children
                .spawn(ChunkInfoTextNode::setup(&config))
                .with_child(ChunkInfoTextSpanNode::setup(&config));
        });
}

pub fn set_font(mut config: ResMut<DebugUiConfig>, fonts: Res<FontAssets>) {
    config.text_config = TextFont {
        font: fonts.vt323_regular.clone(),
        ..default()
    };
}
