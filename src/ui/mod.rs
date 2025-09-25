use bevy::{app::Plugin, prelude::Color};

use crate::ui::{compass::CompassPlugin, debug::ChunkUiPlugin, instrument::InstrumentPlugin};

mod compass;
mod debug;
mod instrument;

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
        app.add_plugins((InstrumentPlugin, ChunkUiPlugin::default(), CompassPlugin));
    }
}
