use bevy::prelude::*;
use bevy_voxel_world::{
    custom_meshing::CHUNK_SIZE_F,
    prelude::{VoxelWorld, VoxelWorldCamera},
};

use crate::{
    ui::{OverlayColor, debug::debug_ui_config::DebugUiConfig},
    voxel::{TerrainWorld, calculate_chunk_position},
};

pub fn update_current_chunk_gizmo(
    camera_query: Query<&Transform, With<VoxelWorldCamera<TerrainWorld>>>,
    voxel_world: VoxelWorld<TerrainWorld>,
    config: Res<DebugUiConfig>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut elapsed: Local<f32>,
) {
    *elapsed += time.delta_secs();
    if *elapsed < config.refresh_interval {
        return;
    }
    let Ok(transform) = camera_query.single() else {
        return;
    };
    let chunk_pos = calculate_chunk_position(transform.translation);
    let Some(chunk) = voxel_world.get_chunk_data(chunk_pos) else {
        return;
    };
    let chunk_world_pos = chunk.world_position();
    gizmos.cube(
        Transform::from_translation(chunk_world_pos).with_scale(Vec3::ONE * CHUNK_SIZE_F),
        OverlayColor::GREEN,
    );
}
