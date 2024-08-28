//! Implements macros for the [`voxel_engine`](https://crates.io/crates/voxel_engine) crate.

extern crate proc_macro;

use proc_macro::*;
use toml::*;
use voxel_engine_types::asset::*;
use wasset::*;

/// Includes all of the assets contained in the specified folder (and its subfolders).
/// A set of modules is generated containing the IDs of each asset.
#[proc_macro]
pub fn include_assets(path: TokenStream) -> TokenStream {
    wasset::include_assets::<VoxelAssetEncoder>(
        path,
        &quote::quote! { ::voxel_engine::asset::AssetId },
    )
}

/// Serializes voxel-engine related assets to the [`Asset`] type.
struct VoxelAssetEncoder;

impl AssetEncoder for VoxelAssetEncoder {
    type Target = Asset;

    fn encode(
        extension: &str,
        _: &Table,
        data: Vec<u8>,
    ) -> Result<Option<Self::Target>, WassetError> {
        match extension {
            "jpg" | "jpeg" => Ok(Some(Asset::Image {
                data,
                format: ImageFormat::Jpeg,
            })),
            "png" => Ok(Some(Asset::Image {
                data,
                format: ImageFormat::Png,
            })),
            "toml" | "txt" => Ok(Some(Asset::Text {
                value: String::from_utf8(data).map_err(WassetError::from_serialize)?,
            })),
            _ => Ok(None),
        }
    }
}

/// Patch `wings` dependency without including `wings_host`.
#[no_mangle]
extern "C" fn __wings_invoke_proxy_function() {}

/// Patch `wings` dependency without including `wings_host`.
#[no_mangle]
extern "C" fn __wings_proxy_index() {}
