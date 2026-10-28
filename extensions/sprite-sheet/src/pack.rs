use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{SheetError, SpriteSheet};

/// Our manifest around a third-party asset pack. The pack's own files stay
/// byte-identical under `upstream/`; this document records where they came
/// from, their license, and the assets the game uses, by stable id. Paths
/// are relative to the manifest.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AssetPack {
    pub name: String,
    pub author: String,
    pub source: String,
    /// Path to the license text, or the license URL when the pack ships none.
    pub license: String,
    /// Static images, by asset id.
    #[serde(default)]
    pub images: BTreeMap<String, String>,
    /// Animated sheets, by asset id, in the [`SpriteSheet`] format.
    #[serde(default)]
    pub sheets: BTreeMap<String, SpriteSheet>,
}

impl AssetPack {
    pub fn parse(json: &str) -> Result<Self, SheetError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn image(&self, id: &str) -> Result<&str, SheetError> {
        self.images
            .get(id)
            .map(String::as_str)
            .ok_or_else(|| SheetError::UnknownAsset(id.to_string()))
    }

    pub fn sheet(&self, id: &str) -> Result<&SpriteSheet, SheetError> {
        self.sheets
            .get(id)
            .ok_or_else(|| SheetError::UnknownAsset(id.to_string()))
    }

    /// Every file the manifest references, for checking that the embedding
    /// extension ships each one.
    pub fn files(&self) -> impl Iterator<Item = &str> {
        self.images
            .values()
            .map(String::as_str)
            .chain(self.sheets.values().map(|sheet| sheet.image.as_str()))
    }
}

/// A pack manifest compiled into an extension together with the files it
/// references, so lookups go through the manifest's ids rather than paths
/// scattered across game code.
pub struct EmbeddedPack {
    pub manifest: &'static str,
    /// `(manifest-relative path, bytes)` for every file the game ships.
    pub files: &'static [(&'static str, &'static [u8])],
}

impl EmbeddedPack {
    pub fn pack(&self) -> Result<AssetPack, SheetError> {
        AssetPack::parse(self.manifest)
    }

    pub fn file(&self, path: &str) -> Result<&'static [u8], SheetError> {
        self.files
            .iter()
            .find(|(embedded, _)| *embedded == path)
            .map(|(_, bytes)| *bytes)
            .ok_or_else(|| SheetError::UnknownAsset(path.to_string()))
    }

    pub fn image_bytes(&self, id: &str) -> Result<&'static [u8], SheetError> {
        self.file(self.pack()?.image(id)?)
    }

    /// Manifest files with no embedded bytes; empty when the pack is whole.
    pub fn missing_files(&self) -> Result<Vec<String>, SheetError> {
        let pack = self.pack()?;
        Ok(pack
            .files()
            .filter(|path| self.file(path).is_err())
            .map(str::to_string)
            .collect())
    }
}
