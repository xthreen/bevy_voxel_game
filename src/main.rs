use bevy::{
    asset::AssetMetaCheck, ecs::system::NonSendMarker, prelude::*, window::PrimaryWindow,
    winit::WINIT_WINDOWS,
};
use std::io::Cursor;
use winit::window::Icon;

use gcd_voxel_game::MainPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BevyCraft".to_string(), // ToDo
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(MainPlugin)
        .add_systems(Startup, set_window_icon)
        .run();
}

fn set_window_icon(
    _non_send_marker: NonSendMarker,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let Ok(primary_entity) = primary_window.single() else {
        return;
    };
    WINIT_WINDOWS.with_borrow_mut(|windows| {
        let Some(primary) = windows.get_window(primary_entity) else {
            return;
        };
        let icon_buf = Cursor::new(include_bytes!("../assets/icons/gcd-logo-2025-small.png"));
        if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            let icon = Icon::from_rgba(rgba, width, height).unwrap();
            primary.set_window_icon(Some(icon));
        };
    });
}
