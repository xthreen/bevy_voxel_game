use std::sync::Arc;

use bevy::{app::Plugin, prelude::*};
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self, Id},
};

use crate::{
    AppState,
    loading::FontAssets,
    ui::{
        compass::CompassPlugin, debug::plugin::DebugUiPlugin, instrument::InstrumentPlugin,
        menu::MenuPlugin, terrain_config::TerrainConfigUiPlugin,
    },
};

mod compass;
mod debug;
mod instrument;
mod menu;
mod terrain_config;

pub trait InspectorUi {
    fn ui(&mut self, ui: &mut egui::Ui, id: Id) -> bool;
}

pub trait DisplayUi
where
    Self: Sized + InspectorUi,
{
    fn name(&self) -> &'static str;
    fn fields(&mut self) -> Box<[(&'static str, &mut dyn InspectorUi)]>;
}

pub struct OverlayColor;

impl OverlayColor {
    pub const RED: Color = Color::srgba(1.0, 0.0, 0.0, 0.7);
    pub const YELLOW: Color = Color::srgba(1.0, 1.0, 0.0, 0.7);
    pub const GREEN: Color = Color::srgba(0.0, 1.0, 0.0, 0.7);
    pub const BG_COLOR: Color = Color::srgba(0.1, 0.1, 0.15, 0.85);
}

pub struct TextOptions;

impl TextOptions {
    pub const DATA_TEXT_SIZE: f32 = 24.0;
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            InstrumentPlugin,
            DebugUiPlugin::default(),
            CompassPlugin,
            MenuPlugin,
            TerrainConfigUiPlugin,
        ))
        .insert_resource(MenuState::default())
        .add_systems(
            OnEnter(AppState::Ready),
            (configure_egui_style, configure_egui_fonts),
        )
        .add_systems(EguiPrimaryContextPass, toggle_menu_hotkey);
    }
}

#[derive(Resource, Default, Clone, Copy, Eq, PartialEq, Debug)]
enum MenuState {
    Closed,
    #[default]
    Main,
    Settings,
}

#[derive(Event)]
struct NavigateMenu(pub MenuState);

pub fn configure_egui_fonts(
    mut contexts: EguiContexts,
    font_assets: Res<FontAssets>,
    fonts: Res<Assets<Font>>,
) {
    if !font_assets.is_changed() {
        return;
    }

    let Some(bevy_font) = fonts.get(&font_assets.vt323_regular) else {
        error!("VT323 font failed to load before egui configuration.");
        return;
    };

    let mut font_defs = egui::FontDefinitions::default();

    font_defs.font_data.insert(
        "vt323".to_owned(),
        Arc::new(egui::FontData::from_owned(bevy_font.data.to_vec())),
    );

    font_defs
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "vt323".to_owned());

    font_defs
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "vt323".to_owned());

    contexts.ctx_mut().unwrap().set_fonts(font_defs);
}

pub fn configure_egui_style(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut().unwrap();

    let mut style = (*ctx.style()).clone();

    style.text_styles = [
        (
            egui::TextStyle::Heading,
            egui::FontId::new(48.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(32.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Monospace,
            egui::FontId::new(24.0, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Small,
            egui::FontId::new(18.0, egui::FontFamily::Proportional),
        ),
    ]
    .into();

    let mut visuals = egui::Visuals::dark();

    visuals.override_text_color = Some(egui::Color32::from_rgb(0, 248, 24));
    visuals.window_fill = egui::Color32::from_rgb(25, 25, 25);

    style.visuals = visuals;

    ctx.set_style(style);
}

fn toggle_menu_hotkey(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<MenuState>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let target_state = match *state {
            MenuState::Closed => MenuState::Main,
            MenuState::Main => MenuState::Closed,
            MenuState::Settings => MenuState::Main, // Escaping settings goes back a layer
        };
        commands.trigger(NavigateMenu(target_state));
    }
}
