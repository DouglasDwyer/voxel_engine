use crate::*;
use wasset::*;

/// Identifies an embedded asset.
pub type AssetId = WassetId;

/// Holds the raw data for an embedded game asset.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Asset {
    /// A 2D image consisting of a format and data.
    Image {
        /// The raw bytes of the image.
        data: Vec<u8>,
        /// The format of the image.
        format: ImageFormat
    },
    /// A text-based file.
    Text {
        /// The text within this document.
        value: String
    }
}

/// Describes the format of an image.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ImageFormat {
    /// The image is encoded as a PNG.
    Png
}

/// Allows for loading and using embedded assets.
#[system_trait(host)]
pub trait AssetManager: 'static {
    /// Gets the raw data for an asset.
    fn try_get_raw(&self, id: AssetId) -> Result<Asset, EngineError>;

    /// Attempts to get a handle that be used to draw `Image` assets as `egui` images.
    fn try_get_ui_texture(&self, id: AssetId) -> Result<UiTextureIndex, EngineError>;
}

impl dyn AssetManager {
    /// Deserializes the given TOML-table asset as `T`. Panics if the conversion fails.
    #[cfg(feature = "toml")]
    pub fn get_from_toml<T: 'static + serde::de::DeserializeOwned>(&self, id: AssetId) -> T {
        match self.try_get_raw(id).expect("Failed to get asset.") {
            Asset::Text { value } => toml::from_str(&value).expect("Failed to deserialize TOML map."),
            x => panic!("Expected text asset; got {x:?}")
        }
    }

    /// Shorthand for `try_get_ui_texture(id).unwrap()`.
    pub fn get_ui_texture(&self, id: AssetId) -> UiTextureIndex {
        self.try_get_ui_texture(id).expect("Failed to load image asset.")
    }
}

/// The allocated index of a UI texture. Only valid for a single frame;
/// `AssetManager::get_ui_texture` should be called to get a new index
/// for every usage.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UiTextureIndex {
    /// The `epaint::TextureId` of the texture.
    pub id: u64,
    /// The width of the texture.
    pub width: f32,
    /// The height of the texture.
    pub height: f32
}

#[cfg(feature = "egui")]
impl<'a> From<UiTextureIndex> for egui_wings::ImageSource<'a> {
    fn from(value: UiTextureIndex) -> Self {
        Self::Texture(egui_wings::load::SizedTexture {
            id: egui_wings::TextureId::User(value.id),
            size: egui_wings::Vec2 { x: value.width, y: value.height }
        })
    }
}