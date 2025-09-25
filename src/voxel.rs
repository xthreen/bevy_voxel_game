use std::sync::Arc;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_voxel_world::{custom_meshing::generate_chunk_mesh, prelude::*};

use noise::{HybridMulti, NoiseFn, Perlin};
use splines::{Interpolation, Key, Spline};

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoxelWorldPlugin::with_config(TerrainWorld::default()));
    }
}

#[derive(Clone, Debug, Component)]
pub struct VoxelChunk;

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

#[derive(Resource, Clone, Default)]
pub struct TerrainWorld {
    params: TerrainWorldParams,
}

#[derive(Clone, Debug, Copy)]
pub struct TerrainWorldParams {
    pub continents: NoiseParams,
    pub erosion: NoiseParams,
    pub peaks_valleys: NoiseParams,
    pub temperatures: NoiseParams,
    pub humidity: NoiseParams,
    pub weirdness: NoiseParams,
    pub density_seed_a: u32,
    pub density_seed_b: u32,
    pub density_seed_c: u32,
    pub spaghetti_seed_a: u32,
    pub spaghetti_seed_b: u32,
}

impl TerrainWorldParams {
    fn explicit_copy(&self) -> TerrainWorldParams {
        TerrainWorldParams {
            continents: self.continents,
            erosion: self.erosion,
            peaks_valleys: self.peaks_valleys,
            temperatures: self.temperatures,
            humidity: self.humidity,
            weirdness: self.weirdness,
            density_seed_a: self.density_seed_a,
            density_seed_b: self.density_seed_b,
            density_seed_c: self.density_seed_c,
            spaghetti_seed_a: self.spaghetti_seed_a,
            spaghetti_seed_b: self.spaghetti_seed_b,
        }
    }
}

impl Default for TerrainWorldParams {
    fn default() -> Self {
        Self {
            continents: NoiseParams::new(1234, 5, 1.1, 2.8, 0.4),
            erosion: NoiseParams::new(5678, 3, 0.5, 2.0, 0.3),
            peaks_valleys: NoiseParams::new(7890, 4, 0.3, 2.0, 0.5),
            temperatures: NoiseParams::new(2233, 1, 0.2, 0.0, 0.0),
            humidity: NoiseParams::new(4455, 2, 0.3, 0.0, 0.0),
            weirdness: NoiseParams::new(6677, 3, 0.8, 0.0, 0.0),
            density_seed_a: 9876,
            density_seed_b: 5432,
            density_seed_c: 1111,
            spaghetti_seed_a: 31337,
            spaghetti_seed_b: 73313,
        }
    }
}

impl VoxelWorldConfig for TerrainWorld {
    type MaterialIndex = BlockMaterial;
    type ChunkUserBundle = VoxelChunk;

    fn spawning_distance(&self) -> u32 {
        10
    }

    fn min_despawn_distance(&self) -> u32 {
        4
    }

    fn max_spawn_per_frame(&self) -> usize {
        1728 // 12^3
    }

    // fn chunk_despawn_strategy(&self) -> ChunkDespawnStrategy {
    //     ChunkDespawnStrategy::FarAway
    // }

    // fn chunk_spawn_strategy(&self) -> ChunkSpawnStrategy {
    //     ChunkSpawnStrategy::Close
    // }

    fn voxel_lookup_delegate(&self) -> VoxelLookupDelegate<Self::MaterialIndex> {
        let params = self.params.explicit_copy();
        Box::new(move |_chunk_pos| get_voxel_fn(params, NoiseGenerators::get_generators_fn()))
    }

    fn chunk_meshing_delegate(
        &self,
    ) -> ChunkMeshingDelegate<Self::MaterialIndex, Self::ChunkUserBundle> {
        Some(Box::new(move |pos| {
            Box::new(move |voxels, texture_index_mapper| {
                let mesh = generate_chunk_mesh(voxels, pos, texture_index_mapper);
                (mesh, Some(VoxelChunk))
            })
        }))
    }

    fn texture_index_mapper(&self) -> Arc<dyn Fn(Self::MaterialIndex) -> [u32; 3] + Send + Sync> {
        Arc::new(BlockMaterial::get_texture_index_map)
    }

    fn voxel_texture(&self) -> Option<(String, u32)> {
        Some(("textures/voxel_atlas.png".into(), 21))
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NoiseParams(u32, usize, f64, f64, f64);

impl NoiseParams {
    pub fn new(
        seed: u32,
        octaves: usize,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
    ) -> Self {
        Self(seed, octaves, frequency, lacunarity, persistence)
    }
    pub fn seed(&self) -> u32 {
        self.0
    }
    pub fn octaves(&self) -> usize {
        self.1
    }
    pub fn frequency(&self) -> f64 {
        self.2
    }
    pub fn lacunarity(&self) -> f64 {
        self.3
    }
    pub fn persistence(&self) -> f64 {
        self.4
    }
}

type ColumnData = (f64, f64, f64, f64, f64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ColumnIndex(i32, i32);

fn get_voxel_fn(
    params: TerrainWorldParams,
    generators: Box<dyn Fn(TerrainWorldParams) -> NoiseGenerators + Send + Sync>,
) -> Box<dyn FnMut(IVec3) -> WorldVoxel<BlockMaterial> + Send + Sync> {
    let NoiseGenerators(
        continent_noise,
        continent_spline,
        erosion_noise,
        erosion_spline,
        pv_noise,
        pv_spline,
        squash_factor_spline,
        temperature_noise,
        humidity_noise,
        weirdness_noise,
        density_noise_a,
        density_noise_b,
        density_noise_c,
        spaghetti_a,
        spaghetti_b,
    ) = generators(params);

    // We use this to cache the noise and biome values for each y column so we only need
    // to calculate it once per x/z coordinate
    let mut column_data_cache = HashMap::<ColumnIndex, ColumnData>::new();

    // Then we return this boxed closure that captures the noise and the cache
    // This will get sent off to a separate thread for meshing by bevy_voxel_world
    Box::new(move |pos: IVec3| {
        if pos.y < -255 {
            return WorldVoxel::Solid(BlockMaterial::Lava); // Lava will be our bedrock for now. TODO: Fluid stuff to make molten and sea level less uniform
        }
        if pos.y > 255 {
            return WorldVoxel::Air;
        }
        let (pos_x_64, pos_y_64, pos_z_64) = (pos.x as f64, pos.y as f64, pos.z as f64);
        let index = ColumnIndex(pos.x, pos.z);
        // Pass 1: Base terrain
        let (height_offset, squashing_factor, temp_val, humidity_val, weirdness_val) =
            match column_data_cache.get(&index) {
                Some(data) => *data,
                None => {
                    let continent_val =
                        continent_noise.get([pos_x_64 * 0.00025, pos_z_64 * 0.00025]);
                    let mut height_sample = continent_spline
                        .clamped_sample(continent_val)
                        .unwrap_or(0.0);

                    let erosion_val = erosion_noise.get([pos_x_64 * 0.0025, pos_z_64 * 0.0025]);
                    height_sample += erosion_spline.clamped_sample(erosion_val).unwrap_or(0.0);

                    let pv_val = pv_noise.get([pos_x_64 * 0.01, pos_z_64 * 0.01]);
                    height_sample += pv_spline.clamped_sample(pv_val).unwrap_or(0.0);

                    let s_factor = squash_factor_spline.clamped_sample(pv_val).unwrap_or(0.3);

                    let temp_val =
                        temperature_noise.get([pos_x_64 * 0.0006667, pos_z_64 * 0.0006667]);
                    let humidity_val =
                        humidity_noise.get([pos_x_64 * 0.0006667, pos_z_64 * 0.0006667]);
                    let weirdness_val =
                        weirdness_noise.get([pos_x_64 * 0.00033, pos_z_64 * 0.00033]);

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
        let base_density = density_noise_a.get([pos_x_64 * 0.01, pos_y_64 * 0.01, pos_z_64 * 0.01]);

        let height_gradient = (pos_y_64 - height_offset) * squashing_factor;

        let final_density = base_density - height_gradient;
        let cave_density = density_noise_b.get([
            pos_x_64 * 0.030303030303,
            pos_y_64 * 0.030303030303,
            pos_z_64 * 0.030303030303,
        ]);
        let cave_warp = density_noise_c.get([
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

        // Pass 2: Carve out air for cheese and spaghetti.
        if pos_y_64 <= height_offset + 1. && (should_carve_cheese || should_carve_meatballs || should_carve_spaghetti) {
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

struct NoiseGenerators(
    HybridMulti<Perlin>,
    Spline<f64, f64>,
    HybridMulti<Perlin>,
    Spline<f64, f64>,
    HybridMulti<Perlin>,
    Spline<f64, f64>,
    Spline<f64, f64>,
    HybridMulti<Perlin>,
    HybridMulti<Perlin>,
    HybridMulti<Perlin>,
    Perlin,
    Perlin,
    Perlin,
    Perlin,
    Perlin,
);
impl NoiseGenerators {
    fn get_generators_fn() -> Box<dyn Fn(TerrainWorldParams) -> NoiseGenerators + Send + Sync> {
        Box::new(move |params| {
            let TerrainWorldParams {
                continents,
                erosion,
                peaks_valleys,
                temperatures,
                humidity,
                weirdness,
                density_seed_a,
                density_seed_b,
                density_seed_c,
                spaghetti_seed_a,
                spaghetti_seed_b,
            } = params;
            // Set up some noise to use as the terrain height map
            let mut continent_noise = HybridMulti::<Perlin>::new(continents.seed());
            continent_noise.octaves = continents.octaves();
            continent_noise.frequency = continents.frequency();
            continent_noise.lacunarity = continents.lacunarity();
            continent_noise.persistence = continents.persistence();

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

            let mut erosion_noise = HybridMulti::<Perlin>::new(erosion.seed());
            erosion_noise.octaves = erosion.octaves();
            erosion_noise.frequency = erosion.frequency();
            erosion_noise.lacunarity = erosion.lacunarity();
            erosion_noise.persistence = erosion.persistence();

            let erosion_spline = Spline::from_iter([
                Key::new(-1.0, 48.0, Interpolation::Linear),
                Key::new(0.0, 36.0, Interpolation::Linear),
                Key::new(0.667, 6.0, Interpolation::Linear),
                Key::new(1.0, -48.01, Interpolation::Linear),
            ]);

            let mut pv_noise = HybridMulti::<Perlin>::new(peaks_valleys.seed());
            pv_noise.octaves = peaks_valleys.octaves();
            pv_noise.frequency = peaks_valleys.frequency();
            pv_noise.lacunarity = peaks_valleys.lacunarity();
            pv_noise.persistence = peaks_valleys.persistence();

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

            let mut temperature_noise = HybridMulti::<Perlin>::new(temperatures.seed());
            temperature_noise.octaves = temperatures.octaves();
            temperature_noise.frequency = temperatures.frequency();

            let mut humidity_noise = HybridMulti::<Perlin>::new(humidity.seed());
            humidity_noise.octaves = humidity.octaves();
            humidity_noise.frequency = humidity.frequency();

            let mut weirdness_noise = HybridMulti::<Perlin>::new(weirdness.seed());
            weirdness_noise.octaves = weirdness.octaves();
            weirdness_noise.frequency = weirdness.frequency();

            let density_noise_a = Perlin::new(density_seed_a);
            let density_noise_b = Perlin::new(density_seed_b);
            let density_noise_c = Perlin::new(density_seed_c);

            let spaghetti_a = Perlin::new(spaghetti_seed_a);
            let spaghetti_b = Perlin::new(spaghetti_seed_b);
            NoiseGenerators(
                continent_noise,
                continent_spline,
                erosion_noise,
                erosion_spline,
                pv_noise,
                pv_spline,
                squash_factor_spline,
                temperature_noise,
                humidity_noise,
                weirdness_noise,
                density_noise_a,
                density_noise_b,
                density_noise_c,
                spaghetti_a,
                spaghetti_b,
            )
        })
    }
}
