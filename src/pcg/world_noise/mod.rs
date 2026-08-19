mod default;
mod generator_2d;
mod generator_3d;
mod ui;

use splines::Spline;

use crate::pcg::{BiomeMap, CaveNoise, CaveThresholds, HeightMap};

#[derive(Clone, Debug)]
pub struct WorldNoise {
    pub continents: HeightMap,
    pub erosion: HeightMap,
    pub peaks_valleys: HeightMap,

    pub squashing_spline: Spline<f32, f32>,

    pub temperature: BiomeMap,
    pub humidity: BiomeMap,
    pub weirdness: BiomeMap,

    pub density_a: CaveNoise,
    pub density_b: CaveNoise,
    pub density_c: CaveNoise,

    pub spaghetti_a: CaveNoise,
    pub spaghetti_b: CaveNoise,

    pub cave_thresholds: CaveThresholds,
}
