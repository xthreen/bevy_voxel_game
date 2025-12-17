use crate::{
    AppState,
    loading::FontAssets,
    ui::{OverlayColor, TextOptions},
};
use bevy::{
    dev_tools::fps_overlay::{FPS_OVERLAY_ZINDEX, FpsOverlayConfig, FpsOverlayPlugin},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use core::time::Duration;

pub struct InstrumentPlugin;

impl Plugin for InstrumentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FpsOverlayPlugin::default(),
            FrameTimeOverlayPlugin::default(),
        ))
        .add_systems(OnEnter(AppState::Ready), set_font)
        .add_systems(
            Update,
            modulate_color_by_fps_value.run_if(in_state(AppState::Ready)),
        );
    }
}

fn set_font(
    mut fps_overlay: ResMut<FpsOverlayConfig>,
    mut ft_overlay: ResMut<FrameTimeOverlayConfig>,
    fonts: Res<FontAssets>,
) {
    fps_overlay.text_config = TextFont {
        font: fonts.vt323_regular.clone(),
        ..default()
    };
    ft_overlay.text_config = TextFont {
        font: fonts.vt323_regular.clone(),
        ..default()
    };
}

fn modulate_color_by_fps_value(
    mut overlay: ResMut<FpsOverlayConfig>,
    diagnostic: Res<DiagnosticsStore>,
) {
    if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            overlay.text_color = if value < 30.0 {
                OverlayColor::RED
            } else if value < 60.0 {
                OverlayColor::YELLOW
            } else {
                OverlayColor::GREEN
            };
        }
    }
}

#[derive(Default)]
struct FrameTimeOverlayPlugin {
    config: FrameTimeOverlayConfig,
}

impl Plugin for FrameTimeOverlayPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        }
        app.insert_resource(self.config.clone())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    (customize_text, toggle_display)
                        .run_if(resource_changed::<FrameTimeOverlayConfig>),
                    update_text,
                ),
            );
    }
}

#[derive(Component)]
struct FrameTimeText;

#[derive(Resource, Clone)]
struct FrameTimeOverlayConfig {
    text_config: TextFont,
    text_color: Color,
    refresh_interval: Duration,
    enabled: bool,
}

impl Default for FrameTimeOverlayConfig {
    fn default() -> Self {
        Self {
            text_config: TextFont {
                font: Handle::<Font>::default(),
                font_size: TextOptions::DATA_TEXT_SIZE,
                ..default()
            },
            text_color: OverlayColor::GREEN,
            enabled: true,
            refresh_interval: Duration::from_millis(100),
        }
    }
}

fn setup(mut commands: Commands, config: Res<FrameTimeOverlayConfig>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(100.0),
                ..default()
            },
            GlobalZIndex(FPS_OVERLAY_ZINDEX),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Text::new("FT: "),
                    config.text_config.clone(),
                    TextColor(config.text_color),
                    FrameTimeText,
                ))
                .with_child((TextSpan::default(), config.text_config.clone()));
        });
}

fn customize_text(
    overlay_config: Res<FrameTimeOverlayConfig>,
    query: Query<Entity, With<FrameTimeText>>,
    mut writer: TextUiWriter,
) {
    for entity in &query {
        writer.for_each_font(entity, |mut font| {
            *font = overlay_config.text_config.clone();
        });
        writer.for_each_color(entity, |mut color| color.0 = overlay_config.text_color);
    }
}

fn toggle_display(
    overlay_config: Res<FrameTimeOverlayConfig>,
    mut query: Query<&mut Visibility, With<FrameTimeText>>,
) {
    for mut visibility in &mut query {
        visibility.set_if_neq(match overlay_config.enabled {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        });
    }
}

fn update_text(
    diagnostic: Res<DiagnosticsStore>,
    query: Query<Entity, With<FrameTimeText>>,
    mut writer: TextUiWriter,
    time: Res<Time>,
    config: Res<FrameTimeOverlayConfig>,
    mut time_since_rerender: Local<Duration>,
) {
    *time_since_rerender += time.delta();
    if *time_since_rerender >= config.refresh_interval {
        *time_since_rerender = Duration::ZERO;
        for entity in &query {
            if let Some(frame_time) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                if let Some(value) = frame_time.smoothed() {
                    *writer.text(entity, 1) = format!("{value:.2}");
                }
            }
        }
    }
}
