use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_voxel_world::prelude::VoxelWorldCamera;

use crate::voxel::TerrainWorld;

pub struct FlyControllerPlugin;

impl Plugin for FlyControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                mouse_capture,
                camera_look,
                camera_move,
                camera_speed,
            ),
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
        Camera::default(),
        Transform::from_xyz(0.0, 64.0, 0.0),
        VoxelWorldCamera::<TerrainWorld>::default(),
        FlyController::default(),
        Msaa::Sample4,
        AtmosphereCamera::default(),
        DistanceFog {
            color: Color::srgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                5000.0,
                Color::srgb(0.35, 0.5, 0.66),
                Color::srgb(0.8, 0.844, 1.0),
            ),
        },
    ));
}

fn mouse_capture(
    mut windows: Query<&mut Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut FlyController>,
) {
    let mut window = windows
        .single_mut()
        .expect("Some Window should exist when mouse_capture_system is run");
    let mut camera = query
        .single_mut()
        .expect("A FlyController component should be present before mouse_capture_system is run");

    if mouse_button_input.just_pressed(MouseButton::Left) && !camera.captured {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
        camera.captured = true;
    }
    if key_input.just_pressed(KeyCode::Escape) && camera.captured {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
        camera.captured = false;
    }
}

fn camera_look(
    mut query: Query<(&mut FlyController, &mut Transform), With<VoxelWorldCamera<TerrainWorld>>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
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
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let mut camera = query
        .single_mut()
        .expect("A FlyController component should be present before camera_speed_system is run");
    for event in mouse_wheel_events.read() {
        camera.speed = (camera.speed + event.y * FlyController::SPEED_STEP)
            .clamp(FlyController::SPEED_MIN, FlyController::SPEED_MAX);
    }
}
