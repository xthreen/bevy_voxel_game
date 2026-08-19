use std::sync::Arc;

use bevy::prelude::*;
use bevy_voxel_world::{
    custom_meshing::CHUNK_SIZE_U,
    prelude::{LodLevel, VoxelLookupDelegate, VoxelWorldConfig, padded_chunk_shape_uniform},
};

use crate::{
    pcg::{BlockMaterial, WorldNoise},
    voxel::{calculate_chunk_position, get_voxel_fn},
};

#[derive(Resource, Clone)]
pub struct TerrainWorld {
    pub noise: Arc<WorldNoise>,
}

impl Default for TerrainWorld {
    fn default() -> Self {
        let noise = WorldNoise::default();
        Self {
            noise: Arc::new(noise),
        }
    }
}

impl VoxelWorldConfig for TerrainWorld {
    type MaterialIndex = BlockMaterial;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        16
    }

    fn min_despawn_distance(&self) -> u32 {
        4
    }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let noise = Arc::clone(&self.noise);
        Box::new(move |chunk_pos, lod_level, previous| {
            let mut uniform_voxel = None;

            if let Some(prev_data) = &previous
                && prev_data.is_full()
                && prev_data.voxels_arc().is_none()
            {
                uniform_voxel = Some(prev_data.get_voxel(UVec3::ZERO));
            }
            if let Some(uniform) = uniform_voxel {
                return Box::new(move |_, _| uniform);
            }

            let noise = Arc::clone(&noise);
            get_voxel_fn(noise.into(), chunk_pos, lod_level)
        })
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(BlockMaterial::get_texture_index_map)
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("textures/voxel_atlas.png".into(), 21))
    }

    fn chunk_data_shape(&self, lod_level: LodLevel) -> UVec3 {
        padded_chunk_shape_uniform(CHUNK_SIZE_U / lod_level.max(1) as u32)
    }

    fn chunk_meshing_shape(&self, lod_level: LodLevel) -> UVec3 {
        padded_chunk_shape_uniform(CHUNK_SIZE_U / lod_level.max(1) as u32)
    }

    fn chunk_lod(
        &self,
        chunk_position: IVec3,
        _previous_lod: Option<LodLevel>,
        camera_position: Vec3,
    ) -> LodLevel {
        let camera_chunk = calculate_chunk_position(camera_position);
        let distance = chunk_position.distance_squared(camera_chunk);

        if distance < 8 * 8 {
            1
        } else if distance < 12 * 12 {
            2
        } else if distance < 14 * 14 {
            4
        } else if distance < 16 * 16 {
            8
        } else if distance < 24 * 24 {
            16
        } else {
            32
        }
    }

    fn attach_chunks_to_root(&self) -> bool {
        false
    }
}
