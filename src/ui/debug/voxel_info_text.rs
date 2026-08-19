use bevy::prelude::*;
use bevy_voxel_world::prelude::{VoxelWorld, VoxelWorldCamera};

use crate::{ui::debug::debug_ui_config::DebugUiConfig, voxel::TerrainWorld};

#[derive(Component)]
pub struct VoxelInfoText;

pub fn update_voxel_ui_text(
    camera_query: Query<&Camera, With<VoxelWorldCamera<TerrainWorld>>>,
    config: Res<DebugUiConfig>,
    voxel_world: VoxelWorld<TerrainWorld>,
    query: Query<Entity, With<VoxelInfoText>>,
    time: Res<Time>,
    mut writer: TextUiWriter,
    mut elapsed: Local<f32>,
) {
    if !config.show_voxel_data {
        return;
    }

    *elapsed += time.delta_secs();

    if *elapsed < config.refresh_interval {
        return;
    }

    let Ok(camera) = camera_query.single() else {
        return;
    };
    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(&config.global_transform, viewport_size * 0.5) else {
        return;
    };

    for entity in &query {
        if let Some(result) = voxel_world.raycast(ray, &|(_pos, _vox)| _vox.is_solid()) {
            let position = result.position;
            let distance = position.distance(config.global_transform.translation());
            *writer.text(entity, 1) = format!("{:?}: {position:.2}, {distance:.2}", result.voxel);
        }
    }
}

pub fn customize_voxel_data(
    config: Res<DebugUiConfig>,
    query: Query<Entity, With<VoxelInfoText>>,
    mut writer: TextUiWriter,
) {
    for entity in &query {
        writer.for_each_font(entity, |mut font| {
            *font = config.text_config.clone();
        });
        writer.for_each_color(entity, |mut color| color.0 = config.text_color);
    }
}

pub fn toggle_voxel_data(
    config: Res<DebugUiConfig>,
    mut query: Query<&mut Node, With<VoxelInfoText>>,
) {
    for mut node in &mut query {
        node.display = if config.show_voxel_data {
            Display::Flex
        } else {
            Display::None
        }
    }
}

#[derive(Bundle)]
pub struct VoxelInfoTextNode(Text, TextColor, TextFont, VoxelInfoText);

impl VoxelInfoTextNode {
    pub fn setup(config: &Res<DebugUiConfig>) -> Self {
        Self(
            Text::new("RayCast hit: "),
            TextColor(config.text_color),
            config.text_config.clone(),
            VoxelInfoText,
        )
    }
}

#[derive(Bundle)]
pub struct VoxelInfoTextSpanNode(TextSpan, TextFont, TextColor);

impl VoxelInfoTextSpanNode {
    pub fn setup(config: &Res<DebugUiConfig>) -> Self {
        Self(
            TextSpan::default(),
            config.text_config.clone(),
            TextColor(config.text_color),
        )
    }
}
