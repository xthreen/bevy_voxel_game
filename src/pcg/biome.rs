use crate::pcg::BlockMaterial;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Biome {
    pub fn get_biome(temperature: f32, humidity: f32, weirdness: f32, height: f32) -> Biome {
        if temperature > 0.4 {
            // Hot Climate
            if humidity < -0.3 {
                if height > 50.0 {
                    Biome::Desert
                } else {
                    Biome::Savanna
                }
            } else {
                Biome::ScrubDesert
            }
        } else if temperature < -0.4 {
            // Cold Climate
            if humidity < -0.3 {
                if height < 50.0 {
                    Biome::Tundra
                } else {
                    Biome::Taiga
                }
            } else if height < 50.0 {
                Biome::Taiga
            } else {
                Biome::PineForest
            }
        } else {
            // Temperate Climate (the transition zone)
            if humidity < -0.3 {
                if height < 50.0 {
                    Biome::Grassland
                } else {
                    Biome::ScrubDesert
                }
            } else if humidity > 0.2 {
                if weirdness > 0.0 {
                    Biome::PineForest
                } else {
                    Biome::Forest
                }
            } else if weirdness > 0.0 {
                Biome::Forest
            } else {
                Biome::Grassland
            }
        }
    }

    pub fn get_ore_for_biome(&self, weirdness: f32) -> BlockMaterial {
        match self {
            Biome::Grassland | Biome::Forest | Biome::PineForest => {
                if weirdness < 15. {
                    BlockMaterial::Marble
                } else if weirdness < 17. {
                    BlockMaterial::Clay
                } else if weirdness < 19. {
                    BlockMaterial::Coal
                } else if weirdness < 20. {
                    BlockMaterial::Iron
                } else if weirdness < 22. {
                    BlockMaterial::Copper
                } else {
                    BlockMaterial::Silver
                }
            }
            Biome::Desert | Biome::Savanna | Biome::ScrubDesert => {
                if weirdness < 10. {
                    BlockMaterial::Tin
                } else if weirdness < 12. {
                    BlockMaterial::Iron
                } else if weirdness < 14. {
                    BlockMaterial::Gold
                } else {
                    BlockMaterial::Platinum
                }
            }
            Biome::Taiga | Biome::Tundra => {
                if weirdness < 10. {
                    BlockMaterial::Coal
                } else if weirdness < 15. {
                    BlockMaterial::Iron
                } else if weirdness < 17. {
                    BlockMaterial::Silver
                } else {
                    BlockMaterial::Adamantine
                }
            }
        }
    }
}
