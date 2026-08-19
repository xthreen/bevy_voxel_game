use bevy::prelude::*;

use crate::ui::debug::debug_ui_config::DebugUiConfig;

#[derive(Component)]
pub struct CameraInfoText;

pub fn update_camera_ui(
    query: Query<Entity, With<CameraInfoText>>,
    time: Res<Time>,
    config: Res<DebugUiConfig>,
    mut writer: TextUiWriter,
    mut elapsed: Local<f32>,
) {
    if !config.show_camera_data {
        return;
    }
    *elapsed += time.delta_secs();

    if *elapsed < config.refresh_interval {
        return;
    }

    for entity in &query {
        let rot = config.transform.rotation.to_euler(EulerRot::YXZ);
        let position = config.transform.translation;
        let distance = (position.x.powf(2.) + position.z.powf(2.)).sqrt();
        *writer.text(entity, 1) = format!(
            "Pos: {:.2}, Dist: {:.2}, Rot: [{:.2}, {:.2}, {:.2}]",
            position, distance, rot.0, rot.1, rot.2
        );
    }
}

pub fn customize_camera_data(
    config: Res<DebugUiConfig>,
    query: Query<Entity, With<CameraInfoText>>,
    mut writer: TextUiWriter,
) {
    for entity in &query {
        writer.for_each_font(entity, |mut font| {
            *font = config.text_config.clone();
        });
        writer.for_each_color(entity, |mut color| color.0 = config.text_color);
    }
}

pub fn toggle_camera_data(
    config: Res<DebugUiConfig>,
    mut query: Query<&mut Node, With<CameraInfoText>>,
) {
    for mut node in &mut query {
        node.display = if config.show_camera_data {
            Display::Flex
        } else {
            Display::None
        }
    }
}

#[derive(Bundle)]
pub struct CameraInfoTextNode(Text, TextColor, TextFont, CameraInfoText);

impl CameraInfoTextNode {
    pub fn setup(config: &Res<DebugUiConfig>) -> Self {
        Self(
            Text::new("Camera: "),
            TextColor(config.text_color),
            config.text_config.clone(),
            CameraInfoText,
        )
    }
}

#[derive(Bundle)]
pub struct CameraInfoTextSpanNode(TextSpan, TextColor, TextFont);

impl CameraInfoTextSpanNode {
    pub fn setup(config: &Res<DebugUiConfig>) -> Self {
        Self(
            TextSpan::default(),
            TextColor(config.text_color),
            config.text_config.clone(),
        )
    }
}
