use crate::pcg::{Biome, BlockMaterial};

#[derive(Clone, Copy, Debug)]
pub struct ColumnData {
    pub height_offset: f32,
    pub squashing_factor: f32,
    pub palette: ColumnPalette,
}

#[derive(Clone, Copy, Debug)]
pub struct ColumnPalette {
    pub surface: BlockMaterial,
    pub subsurface: BlockMaterial,
    pub water_surface: BlockMaterial,
    pub ore_a: BlockMaterial,
    pub ore_b: BlockMaterial,
}

impl ColumnPalette {
    pub fn from_biome(biome: Biome, weirdness: f32) -> Self {
        let surface = match biome {
            Biome::Grassland => BlockMaterial::Grass,
            Biome::Forest => BlockMaterial::Leaves,
            Biome::PineForest => BlockMaterial::Coal,
            Biome::Taiga => BlockMaterial::Wood,
            Biome::Desert => BlockMaterial::Sand,
            Biome::Savanna => BlockMaterial::Gold,
            Biome::ScrubDesert => BlockMaterial::Platinum,
            Biome::Tundra => BlockMaterial::Snow,
        };

        let subsurface = match biome {
            Biome::Desert => BlockMaterial::Sand,
            Biome::Taiga | Biome::Tundra => BlockMaterial::Snow,
            _ => BlockMaterial::Dirt,
        };

        let water_surface = if biome == Biome::Tundra {
            BlockMaterial::Ice
        } else {
            BlockMaterial::Water
        };

        Self {
            surface,
            subsurface,
            water_surface,
            ore_a: biome.get_ore_for_biome(weirdness),
            ore_b: biome.get_ore_for_biome(weirdness * 0.5),
        }
    }
}
