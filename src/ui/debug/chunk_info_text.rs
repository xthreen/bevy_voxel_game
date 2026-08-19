use bevy::prelude::*;
use bevy_voxel_world::prelude::VoxelWorld;

use crate::{
    ui::debug::debug_ui_config::DebugUiConfig, voxel::TerrainWorld, voxel::calculate_chunk_position,
};

#[derive(Component)]
pub struct ChunkInfoText;

pub fn update_chunk_data_text(
    voxel_world: VoxelWorld<TerrainWorld>,
    query: Query<Entity, With<ChunkInfoText>>,
    config: Res<DebugUiConfig>,
    time: Res<Time>,
    mut writer: TextUiWriter,
    mut elapsed: Local<f32>,
) {
    if !config.show_chunk_data {
        return;
    }
    *elapsed += time.delta_secs();

    if *elapsed < config.refresh_interval {
        return;
    }

    let chunk_pos = calculate_chunk_position(config.transform.translation);
    let Some(chunk) = voxel_world.get_chunk_data(chunk_pos) else {
        return;
    };
    let chunk_world_pos = chunk.world_position();
    for entity in &query {
        *writer.text(entity, 1) = format!("{chunk_pos}, {chunk_world_pos:.2}");
    }
}

pub fn customize_chunk_data(
    config: Res<DebugUiConfig>,
    query: Query<Entity, With<ChunkInfoText>>,
    mut writer: TextUiWriter,
) {
    for entity in &query {
        writer.for_each_font(entity, |mut font| {
            *font = config.text_config.clone();
        });
        writer.for_each_color(entity, |mut color| color.0 = config.text_color);
    }
}

pub fn toggle_chunk_data(
    config: Res<DebugUiConfig>,
    mut query: Query<&mut Node, With<ChunkInfoText>>,
) {
    for mut node in &mut query {
        node.display = if config.show_chunk_data {
            Display::Flex
        } else {
            Display::None
        }
    }
}

#[derive(Bundle)]
pub struct ChunkInfoTextNode(Text, TextColor, TextFont, ChunkInfoText);

impl ChunkInfoTextNode {
    pub fn setup(config: &Res<DebugUiConfig>) -> Self {
        Self(
            Text::new("Chunk: "),
            TextColor(config.text_color),
            config.text_config.clone(),
            ChunkInfoText,
        )
    }
}

#[derive(Bundle)]
pub struct ChunkInfoTextSpanNode(TextSpan, TextColor, TextFont);

impl ChunkInfoTextSpanNode {
    pub fn setup(config: &Res<DebugUiConfig>) -> Self {
        Self(
            TextSpan::default(),
            TextColor(config.text_color),
            config.text_config.clone(),
        )
    }
}
