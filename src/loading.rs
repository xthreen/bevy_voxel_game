use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};
use bevy_asset_loader::prelude::*;

use crate::AppState;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Ready)
                .load_collection::<FontAssets>()
                .load_collection::<TextureAssets>(),
        )
        .add_plugins(UiMaterialPlugin::<CompassMaterial>::default());
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/VT323/VT323-Regular.ttf")]
    pub vt323_regular: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/compass.png")]
    pub compass: Handle<Image>,
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct CompassMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub north: f32,
    #[uniform(3)]
    pub dir: f32,
    #[uniform(4)]
    pub alpha: f32,
    #[uniform(5)]
    pub tau: f32,
    #[uniform(6)]
    pub fade_width: f32,
}

impl UiMaterial for CompassMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/compass.wgsl".into()
    }
}
