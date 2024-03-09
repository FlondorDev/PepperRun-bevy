use crate::components::{ApplicationState, AssetsLoading};
use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
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
}

fn check_assets_ready(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    loaded_folder_assets: ResMut<Assets<LoadedFolder>>,
    mut image_assets: ResMut<Assets<Image>>,
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
                        let ima = image_assets.get_mut(file.id().typed()).unwrap();
                        ima.sampler = ImageSampler::Descriptor(
                            bevy::render::texture::ImageSamplerDescriptor {
                                address_mode_u: bevy::render::texture::ImageAddressMode::Repeat,
                                address_mode_v: bevy::render::texture::ImageAddressMode::Repeat,
                                address_mode_w: bevy::render::texture::ImageAddressMode::Repeat,
                                ..Default::default()
                            },
                        );
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
