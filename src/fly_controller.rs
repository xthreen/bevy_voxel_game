use bevy::{
    camera::Exposure,
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::{MouseMotion, MouseWheel},
    light::AtmosphereEnvironmentMapLight,
    pbr::{Atmosphere, AtmosphereMode, AtmosphereSettings},
    post_process::bloom::Bloom,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use bevy_voxel_world::{custom_meshing::CHUNK_SIZE_F, prelude::VoxelWorldCamera};

use crate::voxel::TerrainWorld;

pub struct FlyControllerPlugin;

impl Plugin for FlyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (mouse_capture, camera_look, camera_move, camera_speed),
        );
    }
}

#[derive(Component)]
pub struct FlyController {
    speed: f32,
    captured: bool,
    yaw: f32,
    pitch: f32,
}

impl FlyController {
    const MOUSE_SENSITIVITY: f32 = 0.15;
    const SPEED_MIN: f32 = 2.0;
    const SPEED_MAX: f32 = 100.0;
    const SPEED_STEP: f32 = 2.0;
    const SPEED_INITIAL: f32 = 10.0;
}

impl Default for FlyController {
    fn default() -> Self {
        FlyController {
            speed: FlyController::SPEED_INITIAL,
            captured: false,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 64.0, 0.0),
        VoxelWorldCamera::<TerrainWorld>::default(),
        FlyController::default(),
        Msaa::Sample4,
        Atmosphere::EARTH,
        AtmosphereSettings {
            transmittance_lut_size: UVec2::new(32, 16),
            transmittance_lut_samples: 5,
            multiscattering_lut_size: UVec2::new(8, 8),
            multiscattering_lut_dirs: 8,
            multiscattering_lut_samples: 3,
            sky_view_lut_size: UVec2::new(64, 32),
            sky_view_lut_samples: 4,
            aerial_view_lut_size: UVec3::new(4, 4, 4),
            aerial_view_lut_samples: 1,
            aerial_view_lut_max_distance: 1.6e4,
            scene_units_to_m: 1.0,
            sky_max_samples: 2,
            rendering_method: AtmosphereMode::Raymarched,
        },
        DistanceFog {
            color: *ClearColor::default(),
            falloff: FogFalloff::Linear {
                start: 16.0 * CHUNK_SIZE_F,
                end: 256.0 * CHUNK_SIZE_F,
            },
            ..default()
        },
        Exposure::SUNLIGHT,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        AtmosphereEnvironmentMapLight::default(),
    ));
}

fn mouse_capture(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut query: Query<&mut FlyController>,
) {
    let mut camera = query
        .single_mut()
        .expect("A FlyController component should be present before mouse_capture_system is run");

    if mouse_button_input.just_pressed(MouseButton::Left) && !camera.captured {
        primary_cursor_options.visible = false;
        primary_cursor_options.grab_mode = CursorGrabMode::Locked;
        camera.captured = true;
    }
    if key_input.just_pressed(KeyCode::Escape) && camera.captured {
        primary_cursor_options.visible = true;
        primary_cursor_options.grab_mode = CursorGrabMode::None;
        camera.captured = false;
    }
}

fn camera_look(
    mut query: Query<(&mut FlyController, &mut Transform), With<VoxelWorldCamera<TerrainWorld>>>,
    mut mouse_motion_events: MessageReader<MouseMotion>,
) {
    let (mut camera, mut transform) = query
        .single_mut()
        .expect("A FlyController component should be present before camera_look_system is run");
    if !camera.captured {
        return;
    }
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }
    if delta != Vec2::ZERO {
        camera.yaw -= delta.x * FlyController::MOUSE_SENSITIVITY * 0.01;
        camera.pitch -= delta.y * FlyController::MOUSE_SENSITIVITY * 0.01;
        camera.pitch = camera.pitch.clamp(-1.54, 1.54); // ~89 deg
        let rot = Quat::from_axis_angle(Vec3::Y, camera.yaw)
            * Quat::from_axis_angle(Vec3::X, camera.pitch);
        transform.rotation = rot;
    }
}

fn camera_move(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&FlyController, &mut Transform), With<VoxelWorldCamera<TerrainWorld>>>,
) {
    let (camera, mut transform) = query
        .single_mut()
        .expect("A FlyController component should be present before camera_move_system is run");
    if !camera.captured {
        return;
    }
    let mut direction = Vec3::ZERO;
    if key_input.pressed(KeyCode::KeyW) {
        direction += *transform.forward();
    }
    if key_input.pressed(KeyCode::KeyS) {
        direction -= *transform.forward();
    }
    if key_input.pressed(KeyCode::KeyA) {
        direction -= *transform.right();
    }
    if key_input.pressed(KeyCode::KeyD) {
        direction += *transform.right();
    }
    if key_input.pressed(KeyCode::Space) {
        direction += *transform.up();
    }
    if key_input.pressed(KeyCode::ShiftLeft) {
        direction -= *transform.up();
    }
    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * camera.speed * time.delta_secs();
    }
}

fn camera_speed(
    mut query: Query<&mut FlyController>,
    mut mouse_wheel_events: MessageReader<MouseWheel>,
) {
    let mut camera = query
        .single_mut()
        .expect("A FlyController component should be present before camera_speed_system is run");
    for event in mouse_wheel_events.read() {
        camera.speed = (camera.speed + event.y * FlyController::SPEED_STEP)
            .clamp(FlyController::SPEED_MIN, FlyController::SPEED_MAX);
    }
}
