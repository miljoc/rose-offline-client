use std::io::Cursor;

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{AddAsset, Image, Plugin},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    utils::BoxedFuture,
};

/// Asset loader for ICO (icon) files
pub struct IcoAssetLoader;

impl Default for IcoAssetLoader {
    fn default() -> Self {
        Self {}
    }
}

impl AssetLoader for IcoAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            // Use the image crate to decode the ICO
            let cursor = Cursor::new(bytes);
            let image_dynamic = match image::load(cursor, image::ImageFormat::Ico) {
                Ok(image) => image,
                Err(err) => return Err(bevy::asset::Error::msg(format!("Failed to decode ICO: {}", err))),
            };
            
            // Convert to RGBA
            let image_buffer = image_dynamic.into_rgba8();
            let width = image_buffer.width();
            let height = image_buffer.height();
            
            // Create a Bevy Image from the loaded ICO
            let image = Image::new(
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                image_buffer.into_raw(),
                TextureFormat::Rgba8UnormSrgb,
            );

            load_context.set_default_asset(LoadedAsset::new(image));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ico"]
    }
}

/// Plugin that adds ICO file support to Bevy
#[derive(Default)]
pub struct IcoAssetPlugin;

impl Plugin for IcoAssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset_loader(IcoAssetLoader::default());
    }
}