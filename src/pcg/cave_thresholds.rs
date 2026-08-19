use bevy_egui::egui;

use crate::ui::InspectorUi;

#[derive(Clone, Copy, Debug)]
pub struct CaveThresholds {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Default for CaveThresholds {
    fn default() -> Self {
        Self {
            a: 0.9813,
            b: -0.494321,
            c: 0.007654321,
        }
    }
}

impl InspectorUi for CaveThresholds {
    fn ui(&mut self, ui: &mut egui::Ui, id: egui::Id) -> bool {
        let mut changed = false;
        egui::Grid::new(id.with("cave_thresholds_grid"))
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Cheese Threshold:");
                changed |= ui
                    .add(egui::DragValue::new(&mut self.a).speed(0.001))
                    .changed();
                ui.end_row();

                ui.label("Meatball Threshold:");
                changed |= ui
                    .add(egui::DragValue::new(&mut self.b).speed(0.001))
                    .changed();
                ui.end_row();

                ui.label("Spaghetti Threshold:");
                changed |= ui
                    .add(egui::DragValue::new(&mut self.c).speed(0.001))
                    .changed();
                ui.end_row();
            });

        changed
    }
}
