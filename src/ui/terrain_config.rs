use std::sync::Arc;

use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self, Ui},
};

use crate::{
    pcg::WorldNoise,
    ui::{DisplayUi, InspectorUi, MenuState, NavigateMenu},
    voxel::{ForceWorldRebuildEvent, TerrainWorld},
};

pub struct TerrainConfigUiPlugin;

impl Plugin for TerrainConfigUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainConfigUiResource>()
            .add_observer(menu_state_observer)
            .add_systems(EguiPrimaryContextPass, terrain_config_window_system);
    }
}

#[derive(Resource)]
pub struct TerrainConfigUiResource {
    pub open: bool,
    pub changed: bool,
    pub default: bool,
}

impl Default for TerrainConfigUiResource {
    fn default() -> Self {
        Self {
            open: true,
            changed: false,
            default: true,
        }
    }
}

fn menu_state_observer(_on: On<NavigateMenu>, mut config: ResMut<TerrainConfigUiResource>) {
    match _on.event().0 {
        MenuState::Closed => {
            config.open = false;
        }
        _ => {
            config.open = true;
        }
    }
}

fn terrain_config_window_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    keys: Res<ButtonInput<KeyCode>>,
    mut terrain: ResMut<TerrainWorld>,
    mut config: ResMut<TerrainConfigUiResource>,
    mut pending: Local<Option<WorldNoise>>,
) {
    if keys.just_pressed(KeyCode::Escape) && config.open {
        *pending = Some((*terrain.noise).clone())
    }

    if !config.open {
        return;
    }

    if pending.is_none() {
        *pending = Some((*terrain.noise).clone())
    }

    let new_noise = pending.as_mut().unwrap();

    let mut clear_pending: bool = false;

    let (mut open, mut changed, mut default) = (config.open, config.changed, config.default);

    egui::Window::new(new_noise.name())
        .open(&mut open)
        .title_bar(true)
        .resizable(false)
        .collapsible(false)
        .show(
            contexts.ctx_mut().expect("Context should be available."),
            |ui| {
                changed |= new_noise.ui(ui, ui.id().with("world_noise"));
                ui.separator();
                ui.horizontal(|ui| {
                    ui.add_enabled_ui(changed, |ui: &mut Ui| {
                        if ui.button("Apply").clicked() {
                            terrain.noise = Arc::new(new_noise.clone());

                            commands.trigger(ForceWorldRebuildEvent);
                            changed = false;
                            default = false;
                            clear_pending = true;
                        }
                    });
                    ui.add_enabled_ui(changed || !default, |ui: &mut Ui| {
                        if ui.button("Reset").clicked() {
                            terrain.noise = Arc::new(WorldNoise::default());
                            commands.trigger(ForceWorldRebuildEvent);
                            changed = false;
                            default = true;

                            clear_pending = true;
                        }
                    });
                });
            },
        );
    if clear_pending {
        *pending = None;
    }
    config.open = open;
    config.changed = changed;
    config.default = default;
}
