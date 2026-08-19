mod biome;
mod biome_map;
mod block_material;
mod cave_noise;
mod cave_thresholds;
mod column_data;
mod height_map;
mod noise_generator;
mod spline_generator;
mod world_noise;
// mod world;

use bevy::math::IVec3;

pub use biome::Biome;
pub use biome_map::BiomeMap;
pub use block_material::BlockMaterial;
pub use cave_noise::CaveNoise;
pub use cave_thresholds::CaveThresholds;
pub use column_data::ColumnData;
pub use height_map::HeightMap;
pub use noise_generator::NoiseGenerator;
pub use spline_generator::SplineGenerator;
// pub use world_noise::DisplayUi;
pub use world_noise::WorldNoise;

pub trait NoiseValue3D {
    fn get_value(&self, x: i32, y: i32, z: i32, multiplier: f32) -> f32;
}

pub trait NoiseValue {
    fn get_value(&self, x: i32, z: i32, multiplier: f32) -> f32;
}

pub trait ClampedNoiseValue {
    fn get_clamped_value(&self, x: i32, z: i32, multiplier: f32, min: f32, max: f32) -> f32;
}

pub trait SplineSample {
    fn clamped_sample(&self, x: f32, default: f32) -> f32;
}

pub trait Generator2D<T> {
    fn generate_2d(&self, x: i32, z: i32) -> T;
}

pub trait Generator3D<T> {
    fn generate_3d(&self, column_data: &ColumnData, ipos: IVec3) -> T;
}
