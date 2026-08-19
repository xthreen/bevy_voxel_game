use splines::Interpolation;

use crate::pcg::{CaveThresholds, NoiseGenerator, SplineGenerator, WorldNoise};

impl Default for WorldNoise {
    fn default() -> Self {
        Self {
            continents: (
                NoiseGenerator::new(1234)
                    .with_octaves(5)
                    .with_frequency(1.1)
                    .with_lacunarity(2.8)
                    .with_persistence(0.4),
                SplineGenerator::new(
                    &[
                        (-1.0, -128.0),
                        (-0.96, -96.0),
                        (-0.91, -80.0),
                        (-0.8, -64.0),
                        (-0.7, -60.0),
                        (-0.5, -50.0),
                        (-0.4, -40.0),
                        (-0.3, -36.0),
                        (-0.2, -30.0),
                        (-0.1, -26.0),
                        (0.0, -20.0),
                        (0.1, -16.0),
                        (0.2, 10.0),
                        (0.7, 10.0),
                        (0.8, 64.0),
                        (0.9, 80.0),
                        (1.0, 96.0),
                    ],
                    Interpolation::Linear,
                ),
            )
                .into(),
            erosion: (
                NoiseGenerator::new(5678)
                    .with_octaves(3)
                    .with_frequency(0.5)
                    .with_lacunarity(2.0)
                    .with_persistence(0.3),
                SplineGenerator::new(
                    &[(-1.0, 48.0), (0.0, 36.0), (0.667, 6.0), (1.0, -48.01)],
                    Interpolation::Linear,
                ),
            )
                .into(),
            peaks_valleys: (
                NoiseGenerator::new(7890)
                    .with_octaves(4)
                    .with_frequency(0.3)
                    .with_lacunarity(2.0)
                    .with_persistence(0.5),
                SplineGenerator::new(
                    &[(-1.0, -10.0), (0.0, 10.0), (1.0, 20.0)],
                    Interpolation::Linear,
                ),
            )
                .into(),
            squashing_spline: SplineGenerator::new(
                &[(-1.0, 1.0), (0.0, 0.4), (1.0, 0.03)],
                Interpolation::Linear,
            )
            .into(),
            temperature: NoiseGenerator::new(2233)
                .with_octaves(1)
                .with_frequency(0.2)
                .into(),
            humidity: NoiseGenerator::new(4455)
                .with_octaves(2)
                .with_frequency(0.3)
                .into(),
            weirdness: NoiseGenerator::new(6677)
                .with_octaves(3)
                .with_frequency(0.8)
                .into(),
            density_a: NoiseGenerator::new(9876).into(),
            density_b: NoiseGenerator::new(5432).into(),
            density_c: NoiseGenerator::new(1111).into(),
            spaghetti_a: NoiseGenerator::new(31337).into(),
            spaghetti_b: NoiseGenerator::new(73313).into(),

            cave_thresholds: CaveThresholds::default(),
        }
    }
}
