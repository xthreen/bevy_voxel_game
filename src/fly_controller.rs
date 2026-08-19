use core::time::Duration;

use bevy::{
    camera::Exposure,
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::{MouseMotion, MouseWheel},
    pbr::{Atmosphere, ScatteringMedium},
    post_process::bloom::Bloom,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use bevy_egui::EguiContexts;
use bevy_voxel_world::prelude::VoxelWorldCamera;

use crate::{AppState, settings::GameSettings, voxel::TerrainWorld};

pub struct FlyControllerPlugin;

impl Plugin for FlyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControllerSpatialData::default())
            .add_systems(Startup, setup)
            .add_systems(
                PreUpdate,
                (update_controller_transforms, update_controller_speed)
                    .run_if(in_state(AppState::Ready)),
            )
            .add_systems(
                Update,
                (
                    mouse_capture,
                    (camera_look, camera_move, camera_speed).run_if(is_camera_captured),
                )
                    .run_if(in_state(AppState::Ready)),
            );
    }
}

#[derive(Resource)]
pub struct ControllerSpatialData {
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub speed: f32,
    refresh_interval: f32,
}

impl Default for ControllerSpatialData {
    fn default() -> Self {
        Self {
            global_transform: GlobalTransform::IDENTITY,
            transform: Transform::IDENTITY,
            speed: FlyController::SPEED_INITIAL,
            refresh_interval: Duration::from_millis(100).as_secs_f32(),
        }
    }
}

fn is_camera_captured(query: Query<&FlyController>) -> bool {
    query.single().map(|c| c.captured).unwrap_or(false)
}

fn update_controller_transforms(
    camera_query: Query<(&GlobalTransform, &Transform), With<VoxelWorldCamera<TerrainWorld>>>,
    time: Res<Time>,
    mut spatial_data: ResMut<ControllerSpatialData>,
    mut elapsed: Local<f32>,
) {
    *elapsed += time.delta_secs();
    if *elapsed < spatial_data.refresh_interval {
        return;
    }

    let Ok((cam_gtf, cam_transform)) = camera_query.single() else {
        return;
    };

    spatial_data.global_transform = *cam_gtf;
    spatial_data.transform = *cam_transform;
}

fn update_controller_speed(
    query: Query<&FlyController, With<Camera3d>>,
    time: Res<Time>,
    mut spatial_data: ResMut<ControllerSpatialData>,
    mut elapsed: Local<f32>,
) {
    *elapsed += time.delta_secs();
    if *elapsed < spatial_data.refresh_interval {
        return;
    }

    let Ok(controller) = query.single() else {
        return;
    };

    spatial_data.speed = controller.speed;
}

#[derive(Component)]
pub struct FlyController {
    pub speed: f32,
    pub captured: bool,
    pub yaw: f32,
    pub pitch: f32,
}

impl FlyController {
    pub const MOUSE_SENSITIVITY: f32 = 0.15;
    pub const SPEED_MIN: f32 = 2.0;
    pub const SPEED_MAX: f32 = 100.0;
    pub const SPEED_STEP: f32 = 2.0;
    pub const SPEED_INITIAL: f32 = 10.0;
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

fn setup(mut commands: Commands, mut scattering_mediums: ResMut<Assets<ScatteringMedium>>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 64.0, 0.0),
        VoxelWorldCamera::<TerrainWorld>::default(),
        FlyController::default(),
        Msaa::Sample4,
        // AtmosphereSettings {
        //     transmittance_lut_size: UVec2::new(32, 16),
        //     transmittance_lut_samples: 5,
        //     multiscattering_lut_size: UVec2::new(8, 8),
        //     multiscattering_lut_dirs: 8,
        //     multiscattering_lut_samples: 3,
        //     sky_view_lut_size: UVec2::new(64, 32),
        //     sky_view_lut_samples: 4,
        //     aerial_view_lut_size: UVec3::new(4, 4, 4),
        //     aerial_view_lut_samples: 1,
        //     aerial_view_lut_max_distance: 1.6e4,
        //     scene_units_to_m: 1.0,
        //     sky_max_samples: 2,
        //     rendering_method: AtmosphereMode::LookupTexture,
        // },
        Atmosphere::earthlike(scattering_mediums.add(ScatteringMedium::default())),
        DistanceFog {
            color: Color::srgb(0.5, 0.6, 0.8),
            falloff: FogFalloff::Atmospheric {
                extinction: Vec3::new(1.0, 1.0, 1.0) * 0.00025,
                inscattering: Vec3::new(0.5, 0.6, 0.8) * 0.00025,
            },
            ..default()
        },
        Exposure::SUNLIGHT,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        // AtmosphereEnvironmentMapLight::default(),
    ));
}

fn mouse_capture(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut primary_cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut query: Query<&mut FlyController>,
    mut contexts: EguiContexts,
) {
    let mut camera = query
        .single_mut()
        .expect("A FlyController component should be present before mouse_capture_system is run");

    let ctx = contexts.ctx_mut().expect("Context should be available");

    let ui_wants_input = ctx.wants_pointer_input() || ctx.wants_keyboard_input();

    if mouse_button_input.just_pressed(MouseButton::Left) && !camera.captured && !ui_wants_input {
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
    settings: Res<GameSettings>,
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
        camera.yaw -= delta.x * settings.mouse_sensitivity * 0.01;
        camera.pitch -= delta.y * settings.mouse_sensitivity * 0.01;
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
