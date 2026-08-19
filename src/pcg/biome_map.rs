use bevy_egui::egui;
use noise::{HybridMulti, NoiseFn, Perlin};

use crate::{
    pcg::{ClampedNoiseValue, NoiseGenerator},
    ui::InspectorUi,
};

#[derive(Clone, Debug)]
pub struct BiomeMap {
    pub seed: u32,
    pub noise: HybridMulti<Perlin>,
}

impl BiomeMap {
    pub fn new(noise: NoiseGenerator) -> Self {
        Self {
            seed: noise.seed(),
            noise: noise.into(),
        }
    }

    fn get(&self, x: f64, z: f64) -> f32 {
        self.noise.get([x, z]) as f32
    }
}

impl From<NoiseGenerator> for BiomeMap {
    fn from(value: NoiseGenerator) -> Self {
        Self::new(value)
    }
}

impl InspectorUi for BiomeMap {
    fn ui(&mut self, ui: &mut egui::Ui, id: egui::Id) -> bool {
        let mut changed = false;
        egui::Grid::new(id.with("biomemap_grid"))
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Seed:");
                changed |= ui.add(egui::DragValue::new(&mut self.seed)).changed();
                ui.end_row();
                ui.label("Octaves:");
                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.octaves, 1..=6))
                    .changed();
                ui.end_row();
                ui.label("Frequency:");
                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.frequency, 0.01..=1.0).logarithmic(true))
                    .changed();
                ui.end_row();
            });

        changed
    }
}

impl ClampedNoiseValue for BiomeMap {
    fn get_clamped_value(&self, x: i32, z: i32, multiplier: f32, min: f32, max: f32) -> f32 {
        self.get(x as f64 * multiplier as f64, z as f64 * multiplier as f64)
            .clamp(min, max)
    }
}
