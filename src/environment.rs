use bevy::{
    light::{CascadeShadowConfigBuilder, light_consts::lux},
    prelude::*,
};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_environment)
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

    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(0.98, 0.95, 0.92),
        brightness: 5000.0,
        affects_lightmapped_meshes: true,
    });
}

#[derive(Component)]
struct Sun;

fn daylight_cycle(
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    timer: Local<Timer>,
) {
    // Calculate the continuous time factor based on the elapsed seconds.
    // let t = timer.elapsed_secs_wrapped() * 0.0001;
    let t = timer.elapsed_secs() * 0.0001;

    // Mutate the sun's transform every frame to guarantee fluid visual motion.
    for (mut light_trans, mut directional) in &mut query {
        light_trans.rotation = Quat::from_rotation_x(-t);
        let sine_of_time = t.sin();
        directional.color = Color::srgb(0.5 + sine_of_time, sine_of_time, sine_of_time);
    }
}
