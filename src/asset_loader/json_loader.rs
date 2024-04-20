use bevy::asset::io::Reader;
use bevy::asset::{AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize, Deref, DerefMut)]
pub struct JsonAsset(pub Value);

#[derive(Default)]
pub struct JsonAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum JsonAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [JSON](serde_json) Error
    #[error("Could not parse JSON: {0}")]
    RonSpannedError(#[from] serde_json::Error),
}

impl AssetLoader for JsonAssetLoader {
    type Asset = JsonAsset;
    type Settings = ();
    type Error = JsonAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = serde_json::from_slice::<Value>(&bytes).unwrap();
            Ok(JsonAsset(custom_asset))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
