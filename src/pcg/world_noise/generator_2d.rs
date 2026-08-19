use crate::pcg::{
    Biome, ClampedNoiseValue, ColumnData, Generator2D, NoiseValue, SplineSample, WorldNoise,
    column_data::ColumnPalette,
};

impl Generator2D<ColumnData> for WorldNoise {
    fn generate_2d(&self, x: i32, z: i32) -> ColumnData {
        let continent_val = self.continents.get_value(x, z, 0.00025);

        let mut height_offset = self.continents.clamped_sample(continent_val, 0.0);

        let erosion_val = self.erosion.get_value(x, z, 0.0025);

        height_offset += self.erosion.clamped_sample(erosion_val, 0.0);

        let pv_val = self.peaks_valleys.get_value(x, z, 0.01);

        height_offset += self.peaks_valleys.clamped_sample(pv_val, 0.0);

        let squashing_factor = self.squashing_spline.clamped_sample(pv_val).unwrap_or(0.3);

        let temperature = self
            .temperature
            .get_clamped_value(x, z, 0.0006667, -1.0, 1.0);

        let humidity = self.humidity.get_clamped_value(x, z, 0.0006667, -1.0, 1.0);

        let weirdness = self.weirdness.get_clamped_value(x, z, 0.00033, -1.0, 1.0);

        let biome = Biome::get_biome(temperature, humidity, weirdness, height_offset);

        ColumnData {
            height_offset,
            squashing_factor,
            palette: ColumnPalette::from_biome(biome, weirdness),
        }
    }
}
