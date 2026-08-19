use bevy::app::{AppExit, Plugin};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow, WindowMode};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::fly_controller::FlyController;

use crate::settings::{GameSettings, SaveSettings};
use crate::ui::{MenuState, NavigateMenu};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_menu_navigation)
            .add_systems(EguiPrimaryContextPass, render_menu_system);
    }
}

fn render_menu_system(
    mut contexts: EguiContexts,
    state: Res<MenuState>,
    mut settings: ResMut<GameSettings>,
    mut commands: Commands,
    dirty: Local<bool>,
) {
    if *state == MenuState::Closed {
        return;
    }

    let ctx = contexts.ctx_mut().expect("Context should be available.");

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(150)))
        .show(ctx, |_| {});

    // The actual menu window
    egui::Window::new("System Menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .show(ctx, |ui| {
            ui.add_space(10.0);

            // Route rendering based on the current ECS state
            match *state {
                MenuState::Main => render_main_menu(ui, &mut commands),
                MenuState::Settings => {
                    render_settings_menu(ui, &mut settings, &mut commands, dirty)
                }
                MenuState::Closed => unreachable!(),
            }

            ui.add_space(10.0);
        });
}

fn render_main_menu(ui: &mut egui::Ui, commands: &mut Commands) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);

        if ui.button("Resume").clicked() {
            commands.trigger(NavigateMenu(MenuState::Closed));
        }

        if ui.button("Settings").clicked() {
            commands.trigger(NavigateMenu(MenuState::Settings));
        }

        ui.add_space(20.0);

        if ui.button("Quit to Desktop").clicked() {
            commands.write_message(AppExit::Success);
        }
    });
}

fn render_settings_menu(
    ui: &mut egui::Ui,
    settings: &mut GameSettings,
    commands: &mut Commands,
    mut dirty: Local<bool>,
) {
    ui.vertical_centered(|ui| {
        ui.heading("SETTINGS");
        ui.add_space(20.0);
    });

    egui::Grid::new("settings_grid")
        .num_columns(2)
        .spacing([40.0, 10.0])
        .show(ui, |ui| {
            ui.label("Field of View:");
            if ui
                .add(egui::Slider::new(&mut settings.fov, 45.0..=120.0).suffix("°"))
                .changed()
            {
                *dirty = true;
            }
            ui.end_row();

            ui.label("Mouse Sensitivity:");
            if ui
                .add(egui::Slider::new(
                    &mut settings.mouse_sensitivity,
                    0.1..=5.0,
                ))
                .changed()
            {
                *dirty = true;
            }
            ui.end_row();

            ui.label("Master Volume:");
            if ui
                .add(egui::Slider::new(&mut settings.master_volume, 0.0..=100.0).suffix("%"))
                .changed()
            {
                *dirty = true;
            }
            ui.end_row();

            let mode_text = match settings.window_mode.into() {
                WindowMode::Windowed => "Windowed",
                WindowMode::BorderlessFullscreen(_) => "Borderless Fullscreen",
                WindowMode::Fullscreen(_, _) => "Exclusive Fullscreen",
            };

            egui::ComboBox::from_id_salt("window_mode_combo")
                .selected_text(mode_text)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(
                            &mut settings.window_mode,
                            WindowMode::Windowed.into(),
                            "Windowed",
                        )
                        .changed()
                    {
                        *dirty = true;
                    }
                    if ui
                        .selectable_value(
                            &mut settings.window_mode,
                            WindowMode::BorderlessFullscreen(MonitorSelection::Current).into(),
                            "Borderless Fullscreen",
                        )
                        .changed()
                    {
                        *dirty = true;
                    }
                    if ui
                        .selectable_value(
                            &mut settings.window_mode,
                            WindowMode::Fullscreen(
                                MonitorSelection::Current,
                                VideoModeSelection::Current,
                            )
                            .into(),
                            "Exclusive Fullscreen",
                        )
                        .changed()
                    {
                        *dirty = true;
                    }
                });
            ui.end_row();
        });

    ui.add_space(30.0);

    ui.horizontal(|ui| {
        if ui.button("Back").clicked() {
            commands.trigger(NavigateMenu(MenuState::Main));
        }

        // Push the save button to the far right
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_enabled_ui(*dirty, |ui| {
                if ui.button("Save").clicked() {
                    *dirty = false;
                    commands.trigger(SaveSettings);
                }
            });
        });
    });
}

fn handle_menu_navigation(
    on: On<NavigateMenu>,
    mut state: ResMut<MenuState>,
    mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut fly_controller: Single<&mut FlyController>,
) {
    let target = on.event().0;

    // Only process hardware lock/unlock if we are transitioning into or out of the Closed state
    if *state == MenuState::Closed && target != MenuState::Closed {
        // Menu Opening: Unlock the cursor so egui can use it
        primary_cursor_options.visible = true;
        primary_cursor_options.grab_mode = CursorGrabMode::None;
        fly_controller.captured = false;
    } else if *state != MenuState::Closed && target == MenuState::Closed {
        // Menu Closing: Lock the cursor so the game resumes
        primary_cursor_options.visible = false;
        primary_cursor_options.grab_mode = CursorGrabMode::Locked;
        fly_controller.captured = true;
    }

    *state = target;
}
