use bevy_egui::egui;
use noise::{NoiseFn, Perlin};

use crate::{
    pcg::{NoiseGenerator, NoiseValue3D},
    ui::InspectorUi,
};

#[derive(Clone, Debug)]
pub struct CaveNoise {
    pub seed: u32,
    pub noise: Perlin,
}

impl CaveNoise {
    pub fn new(noise: NoiseGenerator) -> Self {
        Self {
            seed: noise.seed(),
            noise: noise.into(),
        }
    }

    fn get(&self, x: f64, y: f64, z: f64) -> f32 {
        self.noise.get([x, y, z]) as f32
    }
}

impl From<NoiseGenerator> for CaveNoise {
    fn from(value: NoiseGenerator) -> Self {
        Self::new(value)
    }
}

impl InspectorUi for CaveNoise {
    fn ui(&mut self, ui: &mut egui::Ui, id: egui::Id) -> bool {
        let mut changed = false;
        egui::Grid::new(id.with("cavenoise_grid"))
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Seed:");
                changed |= ui.add(egui::DragValue::new(&mut self.seed)).changed();
                ui.end_row();
            });

        changed
    }
}

impl NoiseValue3D for CaveNoise {
    fn get_value(&self, x: i32, y: i32, z: i32, multiplier: f32) -> f32 {
        self.get(
            x as f64 * multiplier as f64,
            y as f64 * multiplier as f64,
            z as f64 * multiplier as f64,
        )
    }
}
