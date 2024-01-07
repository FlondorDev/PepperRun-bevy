use bevy::{prelude::*, render::texture::ImageSampler};

use crate::components::AssetsLoading;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
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
    loading.0.push(server.load("Cobble.png"));
    loading.0.push(server.load("Player.png"));
}

fn check_assets_ready(
    server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut assets: ResMut<Assets<Image>>,
) {
    use bevy::asset::LoadState;
    if loading
        .0
        .iter()
        .map(|h| {
            match server.load_state(h.id()) {
                LoadState::Failed => {
                    // one of our assets had an error
                    false
                }
                LoadState::Loaded => {
                    // all assets are now ready
                    let ima = assets.get_mut(h.id()).unwrap();
                    ima.sampler =
                        ImageSampler::Descriptor(bevy::render::texture::ImageSamplerDescriptor {
                            address_mode_u: bevy::render::texture::ImageAddressMode::Repeat,
                            address_mode_v: bevy::render::texture::ImageAddressMode::Repeat,
                            address_mode_w: bevy::render::texture::ImageAddressMode::Repeat,
                            ..Default::default()
                        });
                    true
                }
                _ => {
                    false
                    // NotLoaded/Loading: not fully ready yet
                }
            }
        })
        .filter(|state| *state == false)
        .collect::<Vec<bool>>()
        .len()
        == 0
    {
        loading.1 = true;
    }
}
