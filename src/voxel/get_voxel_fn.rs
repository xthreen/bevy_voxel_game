use arc_swap::ArcSwap;

use bevy::math::IVec3;
use bevy_voxel_world::{
    custom_meshing::{CHUNK_SIZE_I, CHUNK_SIZE_U},
    prelude::WorldVoxel,
};

use crate::pcg::{BlockMaterial, Generator2D, Generator3D, WorldNoise};

pub type VoxelBox = Box<
    dyn FnMut(IVec3, Option<WorldVoxel<BlockMaterial>>) -> WorldVoxel<BlockMaterial> + Send + Sync,
>;

pub fn get_voxel_fn(
    noise_container: ArcSwap<WorldNoise>,
    chunk_pos: IVec3,
    lod_level: u8,
) -> VoxelBox {
    let chunk_min = chunk_pos * CHUNK_SIZE_I;
    let chunk_max = chunk_min + IVec3::splat(CHUNK_SIZE_I);
    let skirt_enabled = lod_level > 1;

    let noise = noise_container.load();

    let mut column_data_cache = [None; (CHUNK_SIZE_U * CHUNK_SIZE_U) as usize];

    Box::new(move |pos: IVec3, previous| {
        if let Some(voxel) = previous
            && !voxel.is_unset()
        {
            return voxel;
        }

        let local_x = pos.x - chunk_min.x;
        let local_z = pos.z - chunk_min.z;
        let is_inside_chunk =
            (0..CHUNK_SIZE_I).contains(&local_x) && (0..CHUNK_SIZE_I).contains(&local_z);
        if skirt_enabled {
            let outside = !(chunk_min.x..chunk_max.x).contains(&pos.x)
                || !(chunk_min.y..chunk_max.y).contains(&pos.y)
                || !(chunk_min.z..chunk_max.z).contains(&pos.z);
            if outside {
                return WorldVoxel::Unset;
            }
        }
        if !is_inside_chunk {
            return WorldVoxel::Unset;
        }
        if pos.y > 255 {
            return WorldVoxel::Air;
        }
        if pos.y < -255 {
            return WorldVoxel::Air;
        }

        let index = (local_x as usize) + (local_z as usize) * (CHUNK_SIZE_U as usize);
        let column_data = match column_data_cache[index] {
            Some(data) => data,
            None => {
                let data = noise.generate_2d(pos.x, pos.z);
                column_data_cache[index] = Some(data);
                data
            }
        };

        noise.generate_3d(&column_data, pos)
    })
}
