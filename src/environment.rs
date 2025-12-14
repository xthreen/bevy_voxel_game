use bevy::{
    light::{CascadeShadowConfigBuilder, light_consts::lux},
    prelude::*,
};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CycleTimer(Timer::new(
            core::time::Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup_environment)
        .add_systems(Update, daylight_cycle);
    }
}

fn setup_environment(mut commands: Commands) {
    let shadow_cfg = CascadeShadowConfigBuilder { ..default() }.build();
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::FULL_DAYLIGHT,
            ..default()
        },
        Transform::from_xyz(1.0, -0.4, 90.0).looking_at(Vec3::ZERO, Vec3::Y),
        shadow_cfg,
        Sun,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.92),
        brightness: 2500.0,
        affects_lightmapped_meshes: true,
    });
}

#[derive(Component)]
struct Sun;

#[derive(Resource)]
struct CycleTimer(Timer);

fn daylight_cycle(
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        let t = time.elapsed_secs_wrapped() * 0.0001;

        if let Some(Ok((mut light_trans, mut directional))) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            let sine_of_time = t.sin();
            directional.color = Color::srgb(0.5 + sine_of_time, sine_of_time, sine_of_time);
        }
    }
}
