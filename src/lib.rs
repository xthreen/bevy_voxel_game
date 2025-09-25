use bevy::{app::Plugin, prelude::*};

use crate::{
    environment::EnvironmentPlugin, fly_controller::FlyControllerPlugin,
    loading::AssetLoaderPlugin, ui::UiPlugin, voxel::VoxelPlugin,
};

// mod debug_ui;
mod environment;
mod fly_controller;
mod loading;
mod ui;
mod voxel;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    #[default]
    Loading,
    Ready,
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>().add_plugins((
            AssetLoaderPlugin,
            UiPlugin,
            VoxelPlugin,
            EnvironmentPlugin,
            FlyControllerPlugin,
        ));
    }
}
