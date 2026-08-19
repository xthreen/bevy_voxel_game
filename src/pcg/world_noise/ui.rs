use bevy_egui::egui;

use crate::{
    pcg::WorldNoise,
    ui::{DisplayUi, InspectorUi},
};

impl DisplayUi for WorldNoise {
    fn name(&self) -> &'static str {
        "World Noise"
    }

    fn fields(&mut self) -> Box<[(&'static str, &mut dyn InspectorUi)]> {
        Box::new([
            ("Continents", &mut self.continents),
            ("Erosion", &mut self.erosion),
            ("Peaks & Valleys", &mut self.peaks_valleys),
            ("Temperature", &mut self.temperature),
            ("Humidity", &mut self.humidity),
            ("Weirdness", &mut self.weirdness),
            ("Density A", &mut self.density_a),
            ("Density B", &mut self.density_b),
            ("Density C", &mut self.density_c),
            ("Spaghetti A", &mut self.spaghetti_a),
            ("Spaghetti B", &mut self.spaghetti_b),
            ("Cave Thresholds", &mut self.cave_thresholds),
        ])
    }
}

impl InspectorUi for WorldNoise {
    fn ui(&mut self, ui: &mut egui::Ui, id: egui::Id) -> bool {
        let mut changed = false;

        let mut active_tab_idx = ui.data_mut(|data| data.get_temp::<usize>(id).unwrap_or(0));
        let original_idx = active_tab_idx;

        let fields = self.fields();

        let selected_name = fields.get(active_tab_idx).map(|(n, _)| *n).unwrap_or("...");

        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt(id.with("layer_selector"))
                .selected_text(selected_name)
                .show_ui(ui, |ui| {
                    for (idx, (name, _)) in fields.iter().enumerate() {
                        ui.selectable_value(&mut active_tab_idx, idx, *name);
                    }
                });
        });
        ui.separator();

        if active_tab_idx != original_idx {
            ui.data_mut(|data| data.insert_temp(id, active_tab_idx));
        }

        if let Some((name, noise)) = fields.into_iter().nth(active_tab_idx) {
            changed |= noise.ui(ui, id.with(name));
        }

        changed
    }
}
