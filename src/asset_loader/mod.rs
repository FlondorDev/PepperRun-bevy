pub mod json_loader;

use crate::asset_loader::json_loader::JsonAsset;
use crate::structs::resources::AssetsLoading;
use crate::structs::states::ApplicationState;
use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<json_loader::JsonAsset>();
        app.init_asset_loader::<json_loader::JsonAssetLoader>();
        app.add_systems(PreStartup, setup);
        app.add_systems(
            Update,
            check_assets_ready.distributive_run_if(|load_state: Res<AssetsLoading>| !load_state.1),
        );
        app.insert_resource(AssetsLoading(Vec::new(), false));
    }
}

fn setup(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    // add them all to our collection for tracking
    loading.0.push(server.load_folder("texture"));
    loading.0.push(server.load_folder("levels"));
}

fn check_assets_ready(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    loaded_folder_assets: ResMut<Assets<LoadedFolder>>,
    mut image_assets: ResMut<Assets<Image>>,
    mut json_assets: ResMut<Assets<JsonAsset>>,
    mut app_state: ResMut<NextState<ApplicationState>>,
) {
    if loading
        .0
        .iter()
        .map(|h| {
            match server.is_loaded_with_dependencies(h.id()) {
                false => {
                    // one of our assets had an error
                    false
                }
                true => {
                    // all assets are now ready
                    let folder = loaded_folder_assets.get(h.id()).unwrap();
                    for file in &folder.handles {
                        if let Ok(typed) = file.id().try_typed::<Image>() {
                            let ima = image_assets.get_mut(typed).unwrap();
                            ima.sampler = ImageSampler::Descriptor(
                                bevy::render::texture::ImageSamplerDescriptor {
                                    address_mode_u: bevy::render::texture::ImageAddressMode::Repeat,
                                    address_mode_v: bevy::render::texture::ImageAddressMode::Repeat,
                                    address_mode_w: bevy::render::texture::ImageAddressMode::Repeat,
                                    ..Default::default()
                                },
                            );
                        }
                        // else if let Some(json) = json_assets.get_mut(file.id().typed()) {
                        //     info!("{json:?}");
                        // }
                    }
                    app_state.set(ApplicationState::AssetsLoaded);
                    true
                }
            }
        })
        .filter(|state| *state == false)
        .collect::<Vec<bool>>()
        .is_empty()
    {
        loading.1 = true;
    }
}
