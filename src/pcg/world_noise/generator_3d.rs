use bevy_voxel_world::prelude::WorldVoxel;

use crate::pcg::{BlockMaterial, ColumnData, Generator3D, NoiseValue3D, WorldNoise};

impl Generator3D<WorldVoxel<BlockMaterial>> for WorldNoise {
    fn generate_3d(
        &self,
        column_data: &ColumnData,
        ipos: bevy::math::IVec3,
    ) -> WorldVoxel<BlockMaterial> {
        if ipos.y < -255 {
            return WorldVoxel::Solid(BlockMaterial::Lava);
        }

        let pos = ipos.as_vec3();

        let base_density = self.density_a.get_value(ipos.x, ipos.y, ipos.z, 0.01);

        let height_gradient = (pos.y - column_data.height_offset) * column_data.squashing_factor;

        let final_density = base_density - height_gradient;

        let is_solid = final_density > 0.0;
        let is_underwater = ipos.y < 0;

        if !is_solid && !is_underwater {
            return WorldVoxel::Air;
        }

        let depth_below_surface = base_density - height_gradient;

        let mut voxel = if is_solid {
            if depth_below_surface < 1.0 {
                WorldVoxel::Solid(column_data.palette.surface)
            } else if depth_below_surface < 10.0 {
                WorldVoxel::Solid(column_data.palette.subsurface)
            } else {
                WorldVoxel::Solid(BlockMaterial::Stone)
            }
        } else if ipos.y == -1 {
            WorldVoxel::Solid(column_data.palette.water_surface)
        } else {
            WorldVoxel::Solid(BlockMaterial::Water)
        };

        let is_carvable = voxel == WorldVoxel::Solid(BlockMaterial::Stone)
            || voxel == WorldVoxel::Solid(column_data.palette.subsurface)
            || voxel == WorldVoxel::Solid(column_data.palette.surface);

        let cave_density = self
            .density_b
            .get_value(ipos.x, ipos.y, ipos.z, 0.030_303_031);

        let cave_warp = self
            .density_c
            .get_value(ipos.x, ipos.y, ipos.z, 0.030_303_031);

        let spaghetti_a_val = self
            .spaghetti_a
            .get_value(ipos.x, ipos.y, ipos.z, 0.0025)
            .abs();

        let spaghetti_b_val = self
            .spaghetti_b
            .get_value(ipos.x, ipos.y, ipos.z, 0.0025)
            .abs();

        if is_carvable && pos.y <= column_data.height_offset + 1.0 {
            let cheese = cave_density > self.cave_thresholds.a;
            let meatballs = (cave_warp + spaghetti_a_val < self.cave_thresholds.b)
                && (cave_warp + spaghetti_b_val < self.cave_thresholds.b);
            let spaghetti = spaghetti_a_val < self.cave_thresholds.c
                && spaghetti_b_val < self.cave_thresholds.c;

            if cheese || meatballs || spaghetti {
                voxel = if cheese && !meatballs && !spaghetti {
                    WorldVoxel::Solid(column_data.palette.ore_a)
                } else if cheese && meatballs && !spaghetti {
                    WorldVoxel::Solid(column_data.palette.ore_b)
                } else {
                    WorldVoxel::Air
                };
            }
        }

        voxel
    }
}
