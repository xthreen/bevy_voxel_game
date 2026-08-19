use noise::{HybridMulti, Perlin, Seedable};

pub struct NoiseGenerator {
    seed: u32,
    octaves: Option<usize>,
    frequency: Option<f64>,
    lacunarity: Option<f64>,
    persistence: Option<f64>,
}

impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            octaves: None,
            frequency: None,
            lacunarity: None,
            persistence: None,
        }
    }

    pub fn with_octaves(self, octaves: usize) -> Self {
        Self {
            seed: self.seed,
            octaves: Some(octaves),
            frequency: self.frequency,
            lacunarity: self.lacunarity,
            persistence: self.persistence,
        }
    }

    pub fn with_frequency(self, frequency: f64) -> Self {
        Self {
            seed: self.seed,
            octaves: self.octaves,
            frequency: Some(frequency),
            lacunarity: self.lacunarity,
            persistence: self.persistence,
        }
    }

    pub fn with_lacunarity(self, lacunarity: f64) -> Self {
        Self {
            seed: self.seed,
            octaves: self.octaves,
            frequency: self.frequency,
            lacunarity: Some(lacunarity),
            persistence: self.persistence,
        }
    }

    pub fn with_persistence(self, persistence: f64) -> Self {
        Self {
            seed: self.seed,
            octaves: self.octaves,
            frequency: self.frequency,
            lacunarity: self.lacunarity,
            persistence: Some(persistence),
        }
    }

    pub fn seed(&self) -> u32 {
        self.seed
    }
}

impl From<NoiseGenerator> for HybridMulti<Perlin> {
    fn from(value: NoiseGenerator) -> Self {
        let mut perlin = HybridMulti::<Perlin>::new(value.seed);
        if let Some(octaves) = value.octaves {
            perlin.octaves = octaves;
        }
        if let Some(freq) = value.frequency {
            perlin.frequency = freq;
        }
        if let Some(lac) = value.lacunarity {
            perlin.lacunarity = lac;
        }
        if let Some(pers) = value.persistence {
            perlin.persistence = pers;
        }
        perlin
    }
}

impl From<&HybridMulti<Perlin>> for NoiseGenerator {
    fn from(value: &HybridMulti<Perlin>) -> Self {
        Self {
            seed: value.seed(),
            octaves: Some(value.octaves),
            frequency: Some(value.frequency),
            lacunarity: Some(value.lacunarity),
            persistence: Some(value.persistence),
        }
    }
}

impl From<NoiseGenerator> for Perlin {
    fn from(value: NoiseGenerator) -> Self {
        Perlin::new(value.seed)
    }
}

impl From<&Perlin> for NoiseGenerator {
    fn from(value: &Perlin) -> Self {
        Self {
            seed: value.seed(),
            octaves: None,
            frequency: None,
            lacunarity: None,
            persistence: None,
        }
    }
}
