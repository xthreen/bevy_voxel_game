use bevy_egui::egui;
use noise::{HybridMulti, NoiseFn, Perlin};
use splines::Spline;

use crate::{
    pcg::{NoiseGenerator, NoiseValue, SplineGenerator, SplineSample},
    ui::InspectorUi,
};

#[derive(Clone, Debug)]
pub struct HeightMap {
    pub seed: u32,
    pub noise: HybridMulti<Perlin>,
    pub spline: Spline<f32, f32>,
}

impl HeightMap {
    pub fn new(noise: NoiseGenerator, spline: SplineGenerator) -> Self {
        Self {
            seed: noise.seed(),
            noise: noise.into(),
            spline: spline.into(),
        }
    }

    fn get(&self, x: f64, z: f64) -> f32 {
        self.noise.get([x, z]) as f32
    }

    pub fn _set_spline(&mut self, spline: SplineGenerator) {
        self.spline = spline.into();
    }
}

impl From<(NoiseGenerator, SplineGenerator<'_>)> for HeightMap {
    fn from(value: (NoiseGenerator, SplineGenerator)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl InspectorUi for HeightMap {
    fn ui(&mut self, ui: &mut egui::Ui, id: egui::Id) -> bool {
        let mut changed = false;
        egui::Grid::new(id.with("heightmap_grid"))
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
                    .add(
                        egui::Slider::new(&mut self.noise.frequency, 0.0001..=2.0)
                            .logarithmic(true),
                    )
                    .changed();
                ui.end_row();

                ui.label("Lacunarity:");
                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.lacunarity, 1.0..=4.0))
                    .changed();
                ui.end_row();

                ui.label("Persistence:");
                changed |= ui
                    .add(egui::Slider::new(&mut self.noise.persistence, 0.0..=1.0))
                    .changed();
                ui.end_row();
            });
        ui.separator();
        changed
    }
}

impl NoiseValue for HeightMap {
    fn get_value(&self, x: i32, z: i32, multiplier: f32) -> f32 {
        self.get(x as f64 * multiplier as f64, z as f64 * multiplier as f64)
    }
}

impl SplineSample for HeightMap {
    fn clamped_sample(&self, x: f32, default: f32) -> f32 {
        self.spline.clamped_sample(x).unwrap_or(default)
    }
}
