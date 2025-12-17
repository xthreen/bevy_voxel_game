use bevy::{math::ops::floor, prelude::*};
use bevy_voxel_world::{custom_meshing::CHUNK_SIZE_F, prelude::*};
use core::time::Duration;

use crate::{
    AppState,
    loading::FontAssets,
    ui::{OverlayColor, TextOptions},
    voxel::TerrainWorld,
};

#[derive(Default)]
pub struct ChunkUiPlugin {
    config: DebugUiConfig,
}

impl Plugin for ChunkUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone())
            .add_systems(OnEnter(AppState::Ready), setup)
            .add_systems(OnEnter(AppState::Ready), set_font)
            .add_systems(
                Update,
                (
                    (
                        update_voxel_ui_text,
                        update_current_chunk_gizmo,
                        update_chunk_data_text,
                        update_camera_ui,
                        ui_toggle_actions,
                    )
                        .run_if(in_state(AppState::Ready)),
                    (
                        customize_voxel_data,
                        customize_camera_data,
                        customize_chunk_data,
                        toggle_chunk_data,
                        toggle_camera_data,
                        toggle_voxel_data,
                    )
                        .run_if(resource_changed::<DebugUiConfig>),
                ),
            );
    }
}

fn set_font(mut config: ResMut<DebugUiConfig>, fonts: Res<FontAssets>) {
    config.text_config = TextFont {
        font: fonts.vt323_regular.clone(),
        ..default()
    };
}

#[derive(Component)]
struct DebugUiRoot;

#[derive(Component)]
struct ChunkInfoText;

#[derive(Component)]
struct CameraInfoText;

#[derive(Component)]
struct VoxelInfoText;

#[derive(Resource, Clone)]
struct DebugUiConfig {
    text_config: TextFont,
    text_color: Color,
    refresh_interval: Duration,
    show_camera_data: bool,
    show_chunk_data: bool,
    show_voxel_data: bool,
}

impl Default for DebugUiConfig {
    fn default() -> Self {
        Self {
            text_config: TextFont {
                font: Handle::<Font>::default(),
                font_size: TextOptions::DATA_TEXT_SIZE,
                ..default()
            },
            text_color: OverlayColor::GREEN,
            show_camera_data: true,
            show_chunk_data: true,
            show_voxel_data: true,
            refresh_interval: Duration::from_millis(100),
        }
    }
}

fn setup(mut commands: Commands, config: Res<DebugUiConfig>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(128.0),
                left: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(OverlayColor::BG_COLOR),
            DebugUiRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("Camera: "),
                    TextColor(config.text_color),
                    config.text_config.clone(),
                    CameraInfoText,
                ))
                .with_child((
                    TextSpan::default(),
                    config.text_config.clone(),
                    TextColor(config.text_color),
                ));
            parent
                .spawn((
                    Text::new("Chunk: "),
                    TextColor(config.text_color),
                    config.text_config.clone(),
                    ChunkInfoText,
                ))
                .with_child((
                    TextSpan::default(),
                    config.text_config.clone(),
                    TextColor(config.text_color),
                ));
            parent
                .spawn((
                    Text::new("RayCast hit: "),
                    TextColor(config.text_color),
                    config.text_config.clone(),
                    VoxelInfoText,
                ))
                .with_child((
                    TextSpan::default(),
                    config.text_config.clone(),
                    TextColor(config.text_color),
                ));
        });
}

fn update_voxel_ui_text(
    camera_query: Query<(&Camera, &GlobalTransform), With<VoxelWorldCamera<TerrainWorld>>>,
    voxel_world: VoxelWorld<TerrainWorld>,
    query: Query<Entity, With<VoxelInfoText>>,
    time: Res<Time>,
    mut writer: TextUiWriter,
    mut elapsed_since_rerender: Local<Duration>,
) {
    let refresh_interval = Duration::from_millis(256);
    *elapsed_since_rerender += time.delta();

    if *elapsed_since_rerender < refresh_interval {
        return;
    }

    let Ok((camera, cam_gtf)) = camera_query.single() else {
        return;
    };
    let Some(viewport_size) = camera.logical_viewport_size() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(cam_gtf, viewport_size * 0.5) else {
        return;
    };

    for entity in &query {
        if let Some(result) = voxel_world.raycast(ray, &|(_pos, _vox)| _vox.is_solid()) {
            let position = result.position;
            let distance = position.distance(cam_gtf.translation());
            *writer.text(entity, 1) = format!("{:?}: {position:.2}, {distance:.2}", result.voxel);
        }
    }
}

fn update_camera_ui(
    camera_query: Query<&Transform, With<VoxelWorldCamera<TerrainWorld>>>,
    query: Query<Entity, With<CameraInfoText>>,
    time: Res<Time>,
    config: Res<DebugUiConfig>,
    mut writer: TextUiWriter,
    mut last_update: Local<Duration>,
) {
    let Ok(transform) = camera_query.single() else {
        return;
    };
    *last_update += time.delta();
    if *last_update >= config.refresh_interval {
        *last_update = Duration::ZERO;
        for entity in &query {
            let rot = transform.rotation.to_euler(EulerRot::YXZ);
            let position = transform.translation;
            let distance = (position.x.powf(2.) + position.z.powf(2.)).sqrt();
            *writer.text(entity, 1) = format!(
                "Pos: {:.2}, Dist: {:.2}, Rot: [{:.2}, {:.2}, {:.2}]",
                position, distance, rot.0, rot.1, rot.2
            );
        }
    }
}

fn update_current_chunk_gizmo(
    camera_query: Query<&Transform, With<VoxelWorldCamera<TerrainWorld>>>,
    voxel_world: VoxelWorld<TerrainWorld>,
    config: Res<DebugUiConfig>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut last_update: Local<Duration>,
) {
    let Ok(transform) = camera_query.single() else {
        return;
    };
    *last_update += time.delta();
    if *last_update >= config.refresh_interval {
        let chunk_pos = calculate_cuboid_position(transform.translation);
        let Some(chunk) = voxel_world.get_chunk_data(chunk_pos) else {
            return;
        };
        let chunk_world_pos = chunk.world_position();
        gizmos.cuboid(
            Transform::from_translation(chunk_world_pos).with_scale(Vec3::ONE * CHUNK_SIZE_F),
            OverlayColor::GREEN,
        );
    }
}

fn update_chunk_data_text(
    camera_query: Query<&Transform, With<VoxelWorldCamera<TerrainWorld>>>,
    voxel_world: VoxelWorld<TerrainWorld>,
    query: Query<Entity, With<ChunkInfoText>>,
    config: Res<DebugUiConfig>,
    time: Res<Time>,
    mut writer: TextUiWriter,
    mut last_update: Local<Duration>,
) {
    let Ok(transform) = camera_query.single() else {
        return;
    };
    *last_update += time.delta();
    if *last_update >= config.refresh_interval {
        let chunk_pos = calculate_cuboid_position(transform.translation);
        let Some(chunk) = voxel_world.get_chunk_data(chunk_pos) else {
            return;
        };
        let chunk_world_pos = chunk.world_position();
        for entity in &query {
            *writer.text(entity, 1) = format!("{chunk_pos}, {chunk_world_pos:.2}");
        }
    }
}

fn calculate_cuboid_position(position: Vec3) -> IVec3 {
    let (x, y, z) = (
        floor(position.x / CHUNK_SIZE_F),
        floor(position.y / CHUNK_SIZE_F),
        floor(position.z / CHUNK_SIZE_F),
    );
    let (x2, y2, z2) = (x * CHUNK_SIZE_F, y * CHUNK_SIZE_F, z * CHUNK_SIZE_F);
    let (diff_x, diff_y, diff_z) = (position.x - x2, position.y - y2, position.z - z2);
    let (mut x, mut y, mut z) = (x as i32, y as i32, z as i32);
    if diff_x > 16.0 {
        x += 1
    }
    if diff_y > 16.0 {
        y += 1
    }
    if diff_z > 16.0 {
        z += 1
    }
    IVec3::new(x, y, z)
}

fn customize_voxel_data(
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

fn customize_chunk_data(
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

fn customize_camera_data(
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

fn despawn_recursive(commands: &mut Commands, entity: Entity, children: &Query<&Children>) {
    if let Ok(child_list) = children.get(entity) {
        for child in child_list.iter() {
            despawn_recursive(commands, child, children);
        }
    }
    commands.entity(entity).despawn();
}

fn toggle_chunk_data(
    mut commands: Commands,
    config: Res<DebugUiConfig>,
    query: Query<Entity, With<ChunkInfoText>>,
    root: Query<Entity, With<DebugUiRoot>>,
    children: Query<&Children>,
) {
    if config.show_chunk_data {
        // spawn under root if missing
        if query.is_empty() {
            if let Ok(root_ent) = root.single() {
                commands.entity(root_ent).with_children(|parent| {
                    parent
                        .spawn((
                            Text::new("Chunk: "),
                            TextColor(config.text_color),
                            config.text_config.clone(),
                            ChunkInfoText,
                        ))
                        .with_child((
                            TextSpan::default(),
                            config.text_config.clone(),
                            TextColor(config.text_color),
                        ));
                });
            }
        }
    } else {
        for e in &query {
            despawn_recursive(&mut commands, e, &children);
        }
    }
}

fn toggle_camera_data(
    mut commands: Commands,
    config: Res<DebugUiConfig>,
    query: Query<Entity, With<CameraInfoText>>,
    root: Query<Entity, With<DebugUiRoot>>,
    children: Query<&Children>,
) {
    if config.show_camera_data {
        if query.is_empty() {
            if let Ok(root_ent) = root.single() {
                commands.entity(root_ent).with_children(|parent| {
                    parent
                        .spawn((
                            Text::new("Camera: "),
                            TextColor(config.text_color),
                            config.text_config.clone(),
                            CameraInfoText,
                        ))
                        .with_child((
                            TextSpan::default(),
                            config.text_config.clone(),
                            TextColor(config.text_color),
                        ));
                });
            }
        }
    } else {
        for e in &query {
            despawn_recursive(&mut commands, e, &children);
        }
    }
}

fn toggle_voxel_data(
    mut commands: Commands,
    config: Res<DebugUiConfig>,
    query: Query<Entity, With<VoxelInfoText>>,
    root: Query<Entity, With<DebugUiRoot>>,
    children: Query<&Children>,
) {
    if config.show_voxel_data {
        if query.is_empty() {
            if let Ok(root_ent) = root.single() {
                commands.entity(root_ent).with_children(|parent| {
                    parent
                        .spawn((
                            Text::new("RayCast hit: "),
                            TextColor(config.text_color),
                            config.text_config.clone(),
                            VoxelInfoText,
                        ))
                        .with_child((
                            TextSpan::default(),
                            config.text_config.clone(),
                            TextColor(config.text_color),
                        ));
                });
            }
        }
    } else {
        for e in &query {
            despawn_recursive(&mut commands, e, &children);
        }
    }
}

// Simple keyboard actions to toggle UI elements. F1 toggles camera data, F2 toggles chunk data, F3 toggles voxel data.
fn ui_toggle_actions(mut config: ResMut<DebugUiConfig>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::F1) {
        config.show_camera_data = !config.show_camera_data;
        info!("Toggled camera UI -> {}", config.show_camera_data);
    }
    if keys.just_pressed(KeyCode::F2) {
        config.show_chunk_data = !config.show_chunk_data;
        info!("Toggled chunk UI -> {}", config.show_chunk_data);
    }
    if keys.just_pressed(KeyCode::F3) {
        config.show_voxel_data = !config.show_voxel_data;
        info!("Toggled voxel UI -> {}", config.show_voxel_data);
    }
}
