mod get_voxel_fn;
mod terrain_world;

pub use get_voxel_fn::get_voxel_fn;
pub use terrain_world::TerrainWorld;

use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoxelWorldPlugin::with_config(TerrainWorld::default()))
            .add_observer(flush_voxel_world::<TerrainWorld>);
    }
}

#[derive(Event)]
pub struct ForceWorldRebuildEvent;

fn flush_voxel_world<C: VoxelWorldConfig>(
    _on: On<ForceWorldRebuildEvent>,
    mut commands: Commands,
    mut voxel_world: VoxelWorld<C>,
) {
    voxel_world.clear_chunk_map(&mut commands);
}

use bevy::math::{IVec3, Vec3, ops::floor};
use bevy_voxel_world::custom_meshing::CHUNK_SIZE_F;

#[inline]
pub fn calculate_chunk_position(position: Vec3) -> IVec3 {
    let (x, y, z) = (
        floor(position.x / CHUNK_SIZE_F),
        floor(position.y / CHUNK_SIZE_F),
        floor(position.z / CHUNK_SIZE_F),
    );
    let (x2, y2, z2) = (x * CHUNK_SIZE_F, y * CHUNK_SIZE_F, z * CHUNK_SIZE_F);
    let (diff_x, diff_y, diff_z) = (position.x - x2, position.y - y2, position.z - z2);
    let (mut x, mut y, mut z) = (x as i32, y as i32, z as i32);
    if diff_x > 16.0 {
        x += 1
    }
    if diff_y > 16.0 {
        y += 1
    }
    if diff_z > 16.0 {
        z += 1
    }
    IVec3::new(x, y, z)
}
