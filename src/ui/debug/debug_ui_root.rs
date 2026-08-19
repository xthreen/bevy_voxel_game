use bevy::prelude::*;

use crate::ui::{OverlayColor, debug::debug_ui_config::DebugUiConfig};

#[derive(Component)]
pub struct DebugUiRoot;

#[derive(Bundle)]
pub struct DebugUiNode(Node, BackgroundColor, DebugUiRoot);

impl Default for DebugUiNode {
    fn default() -> Self {
        Self(
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(128.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(OverlayColor::BG_COLOR),
            DebugUiRoot,
        )
    }
}

pub fn toggle_debug_ui_root(
    config: Res<DebugUiConfig>,
    mut query: Query<&mut Visibility, With<DebugUiRoot>>,
) {
    for mut visibility in &mut query {
        *visibility = if config.show_voxel_data || config.show_chunk_data || config.show_camera_data
        {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub fn debug_ui_toggle_actions(mut config: ResMut<DebugUiConfig>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F1) {
        config.show_camera_data = !config.show_camera_data;
    }
    if keys.just_pressed(KeyCode::F2) {
        config.show_chunk_data = !config.show_chunk_data;
    }
    if keys.just_pressed(KeyCode::F3) {
        config.show_voxel_data = !config.show_voxel_data;
    }
}
