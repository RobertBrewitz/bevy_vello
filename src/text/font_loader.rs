use thiserror::{self, Error};
use super::font::VelloFont;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};

#[derive(Default)]
pub struct VelloFontLoader;

impl AssetLoader for VelloFontLoader {
    type Asset = VelloFont;

    type Settings = ();

    type Error = VectorLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let vello_font = VelloFont::new(bytes.to_vec());
            Ok(vello_font)
    }

    fn extensions(&self) -> &[&str] {
        &["ttf"]
    }
}


#[derive(Debug, Error)]
#[non_exhaustive]
pub enum VectorLoaderError {
    #[error("Could not load file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not parse utf-8: {0}")]
    FromStrUtf8(#[from] std::str::Utf8Error),
    #[cfg(feature = "svg")]
    #[error("Could not parse svg: {0}")]
    VelloSvg(#[from] vello_svg::Error),
    #[cfg(feature = "lottie")]
    #[error("Could not parse lottie: {0}")]
    Velato(#[from] velato::Error),
}
