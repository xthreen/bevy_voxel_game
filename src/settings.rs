use std::{
    fs,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use bevy::{
    prelude::*,
    tasks::IoTaskPool,
    window::{PrimaryWindow, WindowMode},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::{AppState, fly_controller::FlyController};

pub struct ConfigManagerPlugin;

impl Plugin for ConfigManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSettings::default())
            .insert_resource(ConfigUpdateTracker {
                last_update: None,
                last_saved: None,
                pending_save: Arc::new(AtomicBool::new(false)),
            })
            .add_observer(handle_save_settings)
            .add_observer(config_update_observer)
            .add_observer(config_save_dispatcher)
            .add_systems(OnEnter(AppState::Ready), setup)
            .add_systems(
                Update,
                (settings_application_system.run_if(resource_changed::<GameSettings>))
                    .run_if(in_state(AppState::Ready)),
            );
    }
}

fn setup(mut settings: ResMut<GameSettings>) {
    let path = get_settings_path();
    if !path.exists() {
        return;
    }
    info!("Settings found at path: {:?}", &path);
    if let Ok(contents) = fs::read_to_string(path)
        && let Ok(config) = ron::from_str::<GameSettings>(&contents)
    {
        *settings = config;
    }
}

pub fn settings_application_system(
    settings: Res<GameSettings>,
    mut camera_query: Query<&mut Projection>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    if let Ok(mut projection) = camera_query.single_mut()
        && let Projection::Perspective(ref mut perspective) = *projection
    {
        perspective.fov = settings.fov.to_radians();
    }

    if let Ok(mut window) = windows.single_mut() {
        window.mode = settings.window_mode.into();
    }

    commands.trigger(ConfigUpdate)
}

#[derive(Event)]
pub struct SaveSettings;

pub fn handle_save_settings(
    _on: On<SaveSettings>,
    settings: Res<GameSettings>,
    mut tracker: ResMut<ConfigUpdateTracker>,
    mut is_saving_opt: Local<Option<Arc<AtomicBool>>>,
    mut dirty: Local<bool>,
) {
    *dirty = settings.is_changed();
    // Initialize our thread-safe flag
    let is_saving = is_saving_opt.get_or_insert_with(|| Arc::new(AtomicBool::new(false)));
    tracker.pending_save = is_saving.clone();

    // 2. If settings need saving AND no save task is currently running
    if *dirty && !is_saving.load(Ordering::Relaxed) {
        *dirty = false;
        is_saving.store(true, Ordering::Relaxed); // Lock

        let is_saving_clone = is_saving.clone();
        tracker.last_saved = Some(std::time::Instant::now());
        let contents = settings.to_string();

        // 3. Spawn the isolated task
        IoTaskPool::get()
            .spawn(async move {
                let path_in = get_settings_path();
                let parent = path_in.parent().unwrap();

                // Gracefully handle temp file creation failure
                let path = match tempfile::NamedTempFile::new_in(parent) {
                    Ok(f) => f.into_temp_path(),
                    Err(e) => {
                        error!("Failed to create temporary file: {}", e);
                        is_saving_clone.store(false, Ordering::Relaxed); // Release lock
                        return;
                    }
                };

                let tmp = fs::write(&path, contents).map_err(|e| {
                    error!("Failed to write settings to temporary file: {}", e);
                    e
                });

                if tmp.is_ok()
                    && let Err(e) = fs::rename(&path, &path_in)
                {
                    error!("Failed to rename temporary settings file: {}", e);
                }

                info!("Settings saved at path: {:?}", &path_in);
                // Release lock when finished
                is_saving_clone.store(false, Ordering::Relaxed);
            })
            .detach();
    }
}

#[derive(Event)]
struct ConfigUpdate;

#[derive(Resource)]
pub struct ConfigUpdateTracker {
    last_update: Option<std::time::Instant>,
    last_saved: Option<std::time::Instant>,
    pending_save: Arc<AtomicBool>,
}

fn config_update_observer(_on: On<ConfigUpdate>, mut tracker: ResMut<ConfigUpdateTracker>) {
    tracker.last_update = Some(std::time::Instant::now());
}

fn config_save_dispatcher(
    _on: On<ConfigUpdate>,
    tracker: Res<ConfigUpdateTracker>,
    mut commands: Commands,
) {
    // If there's a pending save, we skip dispatching a new one
    if tracker.pending_save.load(Ordering::Relaxed) {
        return;
    }

    // If the last save was recent (e.g., within 5 seconds), we delay the save
    if let Some(last_saved) = tracker.last_saved
        && last_saved.elapsed() < std::time::Duration::from_secs(10)
    {
        return;
    }

    // Otherwise, we trigger a save
    commands.trigger(SaveSettings);
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum LocalWindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen,
    Fullscreen,
}

impl From<WindowMode> for LocalWindowMode {
    fn from(value: WindowMode) -> Self {
        match value {
            WindowMode::Windowed => LocalWindowMode::Windowed,
            WindowMode::BorderlessFullscreen(_) => LocalWindowMode::BorderlessFullscreen,
            WindowMode::Fullscreen(..) => LocalWindowMode::Fullscreen,
        }
    }
}

impl From<LocalWindowMode> for WindowMode {
    fn from(value: LocalWindowMode) -> Self {
        match value {
            LocalWindowMode::Windowed => WindowMode::Windowed,
            LocalWindowMode::BorderlessFullscreen => {
                WindowMode::BorderlessFullscreen(MonitorSelection::Current)
            }
            LocalWindowMode::Fullscreen => {
                WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
            }
        }
    }
}

#[derive(Asset, Clone, Copy, Debug, TypePath, Resource, Serialize, Deserialize, PartialEq)]
pub struct GameSettings {
    pub fov: f32,
    pub mouse_sensitivity: f32,
    pub master_volume: f32,
    pub window_mode: LocalWindowMode,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fov: 45.0,
            mouse_sensitivity: FlyController::MOUSE_SENSITIVITY,
            master_volume: 50.0,
            window_mode: WindowMode::Windowed.into(),
        }
    }
}

impl std::fmt::Display for GameSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s =
            ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap_or_default();
        write!(f, "{}", s)
    }
}

fn get_settings_path() -> PathBuf {
    let path = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
        .map(|proj_dirs| proj_dirs.config_dir().join("settings.ron"))
        .unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("assets")
                .join("settings.ron")
        });

    if let Some(parent) = path.parent()
        && let Err(e) = fs::create_dir_all(parent)
    {
        error!("Failed to create settings directory: {}", e);
    }

    path
}
