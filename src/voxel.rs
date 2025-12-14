use std::sync::Arc;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_voxel_world::{
    custom_meshing::{CHUNK_SIZE_F, CHUNK_SIZE_I, CHUNK_SIZE_U},
    prelude::*,
};
// custom_meshing::{CHUNK_SIZE_F, CHUNK_SIZE_I, CHUNK_SIZE_U, VoxelArray, generate_chunk_mesh},

use noise::{HybridMulti, NoiseFn, Perlin};
use splines::{Interpolation, Key, Spline};

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoxelWorldPlugin::with_config(TerrainWorld::default()));
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Default)]
pub enum BlockMaterial {
    Grass,
    Dirt,
    #[default]
    Stone,
    Water,
    Marble,
    Sand,
    Snow,
    Ice,
    Wood,
    Leaves,
    Clay,
    Iron,
    Gold,
    Coal,
    Copper,
    Tin,
    Silver,
    Platinum,
    Lava,
    Adamantine,
}

impl BlockMaterial {
    fn get_texture_index_map(mat: BlockMaterial) -> [u32; 3] {
        match mat {
            BlockMaterial::Grass => [0, 1, 2],
            BlockMaterial::Dirt => [2, 2, 2],
            BlockMaterial::Stone => [3, 3, 3],
            BlockMaterial::Water => [4, 4, 4],
            BlockMaterial::Marble => [5, 5, 5],
            BlockMaterial::Sand => [6, 6, 6],
            BlockMaterial::Snow => [7, 7, 7],
            BlockMaterial::Ice => [8, 8, 8],
            BlockMaterial::Wood => [9, 9, 9],
            BlockMaterial::Leaves => [10, 10, 10],
            BlockMaterial::Clay => [11, 11, 11],
            BlockMaterial::Iron => [12, 12, 12],
            BlockMaterial::Gold => [13, 13, 13],
            BlockMaterial::Coal => [14, 14, 14],
            BlockMaterial::Copper => [15, 15, 15],
            BlockMaterial::Tin => [16, 16, 16],
            BlockMaterial::Silver => [17, 17, 17],
            BlockMaterial::Platinum => [18, 18, 18],
            BlockMaterial::Lava => [19, 19, 19],
            BlockMaterial::Adamantine => [20, 20, 20],
        }
    }
}

// Biomes are determined by the climate, height and weirdness.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Biome {
    // Temperate biomes
    Grassland,
    Forest,
    PineForest,
    // Arid biomes
    Desert,
    Savanna,
    ScrubDesert,
    // Polar biomes
    Taiga,
    Tundra,
}

#[derive(Resource, Clone)]
pub struct TerrainWorld {
    continents: Arc<(HybridMulti<Perlin>, Spline<f64, f64>)>,
    erosion: Arc<(HybridMulti<Perlin>, Spline<f64, f64>)>,
    peaks_valleys: Arc<(HybridMulti<Perlin>, Spline<f64, f64>)>,
    squashing_spline: Arc<Spline<f64, f64>>,
    temperatures: Arc<HybridMulti<Perlin>>,
    humidity: Arc<HybridMulti<Perlin>>,
    weirdness: Arc<HybridMulti<Perlin>>,
    density_a: Arc<Perlin>,
    density_b: Arc<Perlin>,
    density_c: Arc<Perlin>,
    spaghetti_a: Arc<Perlin>,
    spaghetti_b: Arc<Perlin>,
}

impl Default for TerrainWorld {
    fn default() -> Self {
        let mut continent_noise = HybridMulti::<Perlin>::new(1234);
        continent_noise.octaves = 5;
        continent_noise.frequency = 1.1;
        continent_noise.lacunarity = 2.8;
        continent_noise.persistence = 0.4;

        let continent_spline = Spline::from_iter([
            Key::new(-1.0, -128.0, Interpolation::Linear),
            Key::new(-0.96, -96.0, Interpolation::Linear),
            Key::new(-0.91, -80.0, Interpolation::Linear),
            Key::new(-0.8, -64.0, Interpolation::Linear),
            Key::new(-0.7, -60.0, Interpolation::Linear),
            Key::new(-0.5, -50.0, Interpolation::Linear),
            Key::new(-0.4, -40.0, Interpolation::Linear),
            Key::new(-0.3, -36.0, Interpolation::Linear),
            Key::new(-0.2, -30.0, Interpolation::Linear),
            Key::new(-0.1, -26.0, Interpolation::Linear),
            Key::new(0.0, -20.0, Interpolation::Linear),
            Key::new(0.1, -16.0, Interpolation::Linear),
            Key::new(0.2, 10.0, Interpolation::Linear),
            Key::new(0.7, 10.0, Interpolation::Linear),
            // High plateaus
            Key::new(0.8, 64.0, Interpolation::Linear),
            Key::new(0.9, 80.0, Interpolation::Linear),
            Key::new(1.0, 96.0, Interpolation::Linear),
        ]);

        let mut erosion_noise = HybridMulti::<Perlin>::new(5678);
        erosion_noise.octaves = 3;
        erosion_noise.frequency = 0.5;
        erosion_noise.lacunarity = 2.0;
        erosion_noise.persistence = 0.3;

        let erosion_spline = Spline::from_iter([
            Key::new(-1.0, 48.0, Interpolation::Linear),
            Key::new(0.0, 36.0, Interpolation::Linear),
            Key::new(0.667, 6.0, Interpolation::Linear),
            Key::new(1.0, -48.01, Interpolation::Linear),
        ]);

        let mut pv_noise = HybridMulti::<Perlin>::new(7890);
        pv_noise.octaves = 4;
        pv_noise.frequency = 0.3;
        pv_noise.lacunarity = 2.0;
        pv_noise.persistence = 0.5;

        let pv_spline = Spline::from_iter([
            // Base level for the perlin noise
            Key::new(-1.0, 0.0, Interpolation::Linear),
            // Peaks and valleys
            Key::new(0.0, 10.0, Interpolation::Linear),
            Key::new(1.0, 20.0, Interpolation::Linear),
        ]);

        let squash_factor_spline = Spline::from_iter([
            Key::new(-1.0, 1.0, Interpolation::Linear),
            Key::new(0.0, 0.4, Interpolation::Linear),
            Key::new(1.0, 0.03, Interpolation::Linear),
        ]);

        let mut temperature_noise = HybridMulti::<Perlin>::new(2233);
        temperature_noise.octaves = 1;
        temperature_noise.frequency = 0.2;

        let mut humidity_noise = HybridMulti::<Perlin>::new(4455);
        humidity_noise.octaves = 2;
        humidity_noise.frequency = 0.3;

        let mut weirdness_noise = HybridMulti::<Perlin>::new(6677);
        weirdness_noise.octaves = 3;
        weirdness_noise.frequency = 0.8;

        let density_a = Perlin::new(9876);
        let density_b = Perlin::new(5432);
        let density_c = Perlin::new(1111);

        let spaghetti_a = Perlin::new(31337);
        let spaghetti_b = Perlin::new(73313);

        Self {
            continents: Arc::new((continent_noise, continent_spline)),
            erosion: Arc::new((erosion_noise, erosion_spline)),
            peaks_valleys: Arc::new((pv_noise, pv_spline)),
            squashing_spline: Arc::new(squash_factor_spline),
            temperatures: Arc::new(temperature_noise),
            humidity: Arc::new(humidity_noise),
            weirdness: Arc::new(weirdness_noise),
            density_a: Arc::new(density_a),
            density_b: Arc::new(density_b),
            density_c: Arc::new(density_c),
            spaghetti_a: Arc::new(spaghetti_a),
            spaghetti_b: Arc::new(spaghetti_b),
        }
    }
}

impl VoxelWorldConfig for TerrainWorld {
    type MaterialIndex = BlockMaterial;
    type ChunkUserBundle = ();

    fn spawning_distance(&self) -> u32 {
        64
    }

    fn min_despawn_distance(&self) -> u32 {
        1
    }

    // fn max_spawn_per_frame(&self) -> usize {
    //     1728 // 12^3
    // }

    // fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
    //     ChunkDespawnStrategy::FarAway
    // }

    // fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
    //     ChunkSpawnStrategy::Close
    // }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let continents = Arc::clone(&self.continents);
        let erosion = Arc::clone(&self.erosion);
        let peaks_valleys = Arc::clone(&self.peaks_valleys);
        let squashing_spline = Arc::clone(&self.squashing_spline);
        let temperatures = Arc::clone(&self.temperatures);
        let humidity = Arc::clone(&self.humidity);
        let weirdness = Arc::clone(&self.weirdness);
        let density_a = Arc::clone(&self.density_a);
        let density_b = Arc::clone(&self.density_b);
        let density_c = Arc::clone(&self.density_c);
        let spaghetti_a = Arc::clone(&self.spaghetti_a);
        let spaghetti_b = Arc::clone(&self.spaghetti_b);
        Box::new(move |chunk_pos, lod_level, _previous| {
            let continents = Arc::clone(&continents);
            let erosion = Arc::clone(&erosion);
            let peaks_valleys = Arc::clone(&peaks_valleys);
            let squashing_spline = Arc::clone(&squashing_spline);
            let temperatures = Arc::clone(&temperatures);
            let humidity = Arc::clone(&humidity);
            let weirdness = Arc::clone(&weirdness);
            let density_a = Arc::clone(&density_a);
            let density_b = Arc::clone(&density_b);
            let density_c = Arc::clone(&density_c);
            let spaghetti_a = Arc::clone(&spaghetti_a);
            let spaghetti_b = Arc::clone(&spaghetti_b);
            get_voxel_fn(
                continents,
                erosion,
                peaks_valleys,
                squashing_spline,
                temperatures,
                humidity,
                weirdness,
                density_a,
                density_b,
                density_c,
                spaghetti_a,
                spaghetti_b,
                chunk_pos,
                lod_level,
            )
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
        let camera_chunk = (camera_position / CHUNK_SIZE_F).floor();
        let distance = chunk_position.as_vec3().distance(camera_chunk);

        // directly set lod values to our stride lengths
        if distance < 16.0 {
            1
        } else if distance < 24.0 {
            2
        } else if distance < 32.0 {
            4
        } else if distance < 40.0 {
            8
        } else if distance < 48.0 {
            16
        } else {
            32
        }
    }

    fn attach_chunks_to_root(&self) -> bool {
        false
    }
}

type ColumnData = (f64, f64, f64, f64, f64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ColumnIndex(i32, i32);

fn get_voxel_fn(
    continents: Arc<(HybridMulti<Perlin>, Spline<f64, f64>)>,
    erosion: Arc<(HybridMulti<Perlin>, Spline<f64, f64>)>,
    peaks_valleys: Arc<(HybridMulti<Perlin>, Spline<f64, f64>)>,
    squashing_spline: Arc<Spline<f64, f64>>,
    temperatures: Arc<HybridMulti<Perlin>>,
    humidity: Arc<HybridMulti<Perlin>>,
    weirdness: Arc<HybridMulti<Perlin>>,
    density_a: Arc<Perlin>,
    density_b: Arc<Perlin>,
    density_c: Arc<Perlin>,
    spaghetti_a: Arc<Perlin>,
    spaghetti_b: Arc<Perlin>,
    chunk_pos: IVec3,
    lod_level: u8,
) -> Box<
    dyn FnMut(IVec3, Option<WorldVoxel<BlockMaterial>>) -> WorldVoxel<BlockMaterial> + Send + Sync,
> {
    if chunk_pos.y < -255 {
        return Box::new(|_, _| WorldVoxel::Solid(BlockMaterial::Lava)); // Lava will be our bedrock for now. TODO: Fluid stuff to make molten and sea level less uniform
    }
    if chunk_pos.y > 255 {
        return Box::new(|_, _| WorldVoxel::Unset);
    }
    let chunk_min = chunk_pos * CHUNK_SIZE_I;
    let chunk_max = chunk_min + IVec3::splat(CHUNK_SIZE_I);
    let skirt_enabled = lod_level == 2;

    // We use this to cache the noise and biome values for each y column so we only need
    // to calculate it once per x/z coordinate
    let mut column_data_cache = HashMap::<ColumnIndex, ColumnData>::new();

    // Then we return this boxed closure that captures the noise and the cache
    // This will get sent off to a separate thread for meshing by bevy_voxel_world
    Box::new(move |pos: IVec3, _previous| {
        if skirt_enabled {
            let outside = pos.x < chunk_min.x
                || pos.x >= chunk_max.x
                || pos.y < chunk_min.y
                || pos.y >= chunk_max.y
                || pos.z < chunk_min.z
                || pos.z >= chunk_max.z;
            if outside {
                return WorldVoxel::Unset;
            }
        }

        let (pos_x_64, pos_y_64, pos_z_64) = (pos.x as f64, pos.y as f64, pos.z as f64);
        let index = ColumnIndex(pos.x, pos.z);
        // Pass 1: Base terrain
        let (height_offset, squashing_factor, temp_val, humidity_val, weirdness_val) =
            match column_data_cache.get(&index) {
                Some(data) => *data,
                None => {
                    let continent_val = continents.0.get([pos_x_64 * 0.00025, pos_z_64 * 0.00025]);
                    let mut height_sample =
                        continents.1.clamped_sample(continent_val).unwrap_or(0.0);

                    let erosion_val = erosion.0.get([pos_x_64 * 0.0025, pos_z_64 * 0.0025]);
                    height_sample += erosion.1.clamped_sample(erosion_val).unwrap_or(0.0);

                    let pv_val = peaks_valleys.0.get([pos_x_64 * 0.01, pos_z_64 * 0.01]);
                    height_sample += peaks_valleys.1.clamped_sample(pv_val).unwrap_or(0.0);

                    let s_factor = squashing_spline.clamped_sample(pv_val).unwrap_or(0.3);

                    let temp_val = temperatures.get([pos_x_64 * 0.0006667, pos_z_64 * 0.0006667]);
                    let humidity_val = humidity.get([pos_x_64 * 0.0006667, pos_z_64 * 0.0006667]);
                    let weirdness_val = weirdness.get([pos_x_64 * 0.00033, pos_z_64 * 0.00033]);

                    let data = (
                        height_sample,
                        s_factor,
                        temp_val,
                        humidity_val,
                        weirdness_val,
                    );
                    column_data_cache.insert(index, data);
                    data
                }
            };
        let base_density = density_a.get([pos_x_64 * 0.01, pos_y_64 * 0.01, pos_z_64 * 0.01]);

        let height_gradient = (pos_y_64 - height_offset) * squashing_factor;

        let final_density = base_density - height_gradient;
        let biome = if temp_val > 0.4 {
            // Hot Climate
            if humidity_val < -0.3 {
                if height_offset > 50.0 {
                    Biome::Desert
                } else {
                    Biome::Savanna
                }
            } else {
                Biome::ScrubDesert
            }
        } else if temp_val < -0.4 {
            // Cold Climate
            if humidity_val < -0.3 {
                if height_offset < 50.0 {
                    Biome::Tundra
                } else {
                    Biome::Taiga
                }
            } else {
                if height_offset < 50.0 {
                    Biome::Taiga
                } else {
                    Biome::PineForest
                }
            }
        } else {
            // Temperate Climate (the transition zone)
            if humidity_val < -0.3 {
                if height_offset < 50.0 {
                    Biome::Grassland
                } else {
                    Biome::ScrubDesert
                }
            } else if humidity_val > 0.2 {
                if weirdness_val > 0.0 {
                    Biome::PineForest
                } else {
                    Biome::Forest
                }
            } else if weirdness_val > 0.0 {
                Biome::Forest
            } else {
                Biome::Grassland
            }
        };

        let mut voxel = if final_density > 0.0 {
            let density_above =
                base_density - ((pos_y_64 + 1.0) - height_offset) * squashing_factor;

            if density_above <= 0.0 {
                WorldVoxel::Solid(match biome {
                    Biome::Grassland => BlockMaterial::Grass,
                    Biome::Forest => BlockMaterial::Leaves,
                    Biome::PineForest => BlockMaterial::Coal,
                    Biome::Taiga => BlockMaterial::Wood,
                    Biome::Desert => BlockMaterial::Sand,
                    Biome::Savanna => BlockMaterial::Gold,
                    Biome::ScrubDesert => BlockMaterial::Platinum,
                    Biome::Tundra => BlockMaterial::Snow,
                })
            } else {
                let depth_probe =
                    base_density - ((pos_y_64 + 5.0) - height_offset) * squashing_factor;
                if depth_probe <= 0.0 {
                    match biome {
                        Biome::Desert => WorldVoxel::Solid(BlockMaterial::Sand),
                        Biome::Taiga | Biome::Tundra => WorldVoxel::Solid(BlockMaterial::Snow),
                        _ => WorldVoxel::Solid(BlockMaterial::Dirt),
                    }
                } else {
                    WorldVoxel::Solid(BlockMaterial::Stone)
                }
            }
        } else if pos.y < -10 {
            match biome {
                Biome::Tundra => {
                    if pos.y == -11 {
                        WorldVoxel::Solid(BlockMaterial::Ice)
                    } else {
                        WorldVoxel::Solid(BlockMaterial::Water)
                    }
                }
                _ => WorldVoxel::Solid(BlockMaterial::Water),
            }
        } else {
            WorldVoxel::Air
        };

        if voxel == WorldVoxel::Solid(BlockMaterial::Water)
            || voxel == WorldVoxel::Solid(BlockMaterial::Ice)
            || voxel == WorldVoxel::Air
        {
            return voxel;
        };

        // As above, returning early to leave Water, Ice and Air blocks unchanged by cave generation,
        // we will also protect the subsurface blocks under sea level. At least until we implement more fluid stuff
        if pos.y < -10
            && (voxel == WorldVoxel::Solid(BlockMaterial::Sand)
                || voxel == WorldVoxel::Solid(BlockMaterial::Snow)
                || voxel == WorldVoxel::Solid(BlockMaterial::Dirt))
        {
            return voxel;
        }

        let cave_density = density_b.get([
            pos_x_64 * 0.030303030303,
            pos_y_64 * 0.030303030303,
            pos_z_64 * 0.030303030303,
        ]);
        let cave_warp = density_c.get([
            pos_x_64 * 0.030303030303,
            pos_y_64 * 0.030303030303,
            pos_z_64 * 0.030303030303,
        ]);

        let spaghetti_a_val = spaghetti_a
            .get([pos_x_64 * 0.0025, pos_y_64 * 0.0025, pos_z_64 * 0.0025])
            .abs();
        let spaghetti_b_val = spaghetti_b
            .get([pos_x_64 * 0.0025, pos_y_64 * 0.0025, pos_z_64 * 0.0025])
            .abs();

        let spaghetti_threshold = 0.007654321;
        let meatball_threshold = -0.494321;
        let cheese_threshold = 0.9813;
        let should_carve_cheese = cave_density > cheese_threshold;

        let should_carve_meatballs = cave_warp + spaghetti_a_val < meatball_threshold
            && cave_warp + spaghetti_b_val < meatball_threshold;

        let should_carve_spaghetti =
            spaghetti_a_val < spaghetti_threshold && spaghetti_b_val < spaghetti_threshold;

        // Pass 2: Carve out air for cheese and spaghetti.
        if pos_y_64 <= height_offset + 1.
            && (should_carve_cheese || should_carve_meatballs || should_carve_spaghetti)
        {
            if should_carve_cheese && !should_carve_meatballs && !should_carve_spaghetti {
                match temp_val {
                    t if t < -0.5 => match humidity_val {
                        h if h < -0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Copper),
                        h if h < -0.1 => voxel = WorldVoxel::Solid(BlockMaterial::Adamantine),
                        h if h < 0.1 => voxel = WorldVoxel::Solid(BlockMaterial::Iron),
                        h if h < 0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Marble),
                        _ => voxel = WorldVoxel::Air,
                    },
                    t if t < 0.5 => match humidity_val {
                        h if h < -0.5 => match weirdness_val {
                            w if w < -0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Coal),
                            w if w < -0.1 => voxel = WorldVoxel::Solid(BlockMaterial::Wood),
                            w if w < 0.1 => voxel = WorldVoxel::Solid(BlockMaterial::Gold),
                            w if w < 0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Tin),
                            _ => voxel = WorldVoxel::Air,
                        },
                        h if h < -0.1 => match weirdness_val {
                            w if w < -0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Copper),
                            w if w < 0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Iron),
                            _ => voxel = WorldVoxel::Air,
                        },
                        h if h < 0.1 => match weirdness_val {
                            w if w < -0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Clay),
                            w if w < 0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Dirt),
                            _ => voxel = WorldVoxel::Air,
                        },
                        h if h < 0.5 => match weirdness_val {
                            w if w < -0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Silver),
                            w if w < 0.5 => voxel = WorldVoxel::Solid(BlockMaterial::Gold),
                            _ => voxel = WorldVoxel::Air,
                        },
                        _ => voxel = WorldVoxel::Air,
                    },
                    _ => voxel = WorldVoxel::Air,
                }
            } else {
                voxel = WorldVoxel::Air;
            }
        }

        voxel
    })
}
