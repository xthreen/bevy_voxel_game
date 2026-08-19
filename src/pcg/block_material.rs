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
    const TEXTURE_INDICES: [[u32; 3]; 20] = [
        [0, 1, 2],    // Grass
        [2, 2, 2],    // Dirt
        [3, 3, 3],    // Stone
        [4, 4, 4],    // Water
        [5, 5, 5],    // Marble
        [6, 6, 6],    // Sand
        [7, 7, 7],    // Snow
        [8, 8, 8],    // Ice
        [9, 9, 9],    // Wood
        [10, 10, 10], // Leaves
        [11, 11, 11], // Clay
        [12, 12, 12], // Iron
        [13, 13, 13], // Gold
        [14, 14, 14], // Coal
        [15, 15, 15], // Copper
        [16, 16, 16], // Tin
        [17, 17, 17], // Silver
        [18, 18, 18], // Platinum
        [19, 19, 19], // Lava
        [20, 20, 20], // Adamantine
    ];

    pub fn get_texture_index_map(mat: BlockMaterial) -> [u32; 3] {
        Self::TEXTURE_INDICES[mat as usize]
    }
}
