use bevy::{
    pbr::{CascadeShadowConfigBuilder, light_consts::lux::AMBIENT_DAYLIGHT},
    prelude::*,
};
use bevy_atmosphere::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AtmosphereModel::new(Nishita {
            mie_coefficient: 5e-5,
            rayleigh_coefficient: Vec3::new(5.8e-6, 13.5e-6, 33.1e-6),
            ..default()
        }))
        .insert_resource(CycleTimer(Timer::new(
            core::time::Duration::from_millis(100),
            TimerMode::Repeating,
        )))
        .add_plugins(AtmospherePlugin)
        .add_systems(Startup, setup_environment)
        .add_systems(Update, daylight_cycle);
    }
}

fn setup_environment(mut commands: Commands) {
    let shadow_cfg = CascadeShadowConfigBuilder { ..default() }.build();
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.15, -0.1, 0.15), Vec3::Y),
        shadow_cfg,
        Sun,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 100.0,
        affects_lightmapped_meshes: true,
    });
}

#[derive(Component)]
struct Sun;

#[derive(Resource)]
struct CycleTimer(Timer);

fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let t = time.elapsed_secs_wrapped() * 0.001;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some(Ok((mut light_trans, mut directional))) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT;
            directional.color = Color::srgb(1.0, t.sin(), t.sin());
        }
    }
}
