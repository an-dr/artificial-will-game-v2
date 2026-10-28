//! Sprite-sheet documents and third-party asset-pack manifests.
//!
//! A sheet is one PNG plus a JSON document naming its animations. Each
//! animation lists its frames as source rectangles (`rects`), or as the
//! `row` + `frames` shorthand for a uniform grid. Bones today only plays
//! frames laid out left to right from the image origin, so
//! [`SpriteSheet::cut_strip`] re-packs one animation into such a strip.
//! That step disappears once bones draws source rects natively — see
//! `TECHDEBT.md` at the repository root.

mod pack;
mod strip;

use std::collections::BTreeMap;
use std::fmt;

use bones_messages::game_core::{Sprite, SpritePresentation};
use serde::Deserialize;

pub use pack::{AssetPack, EmbeddedPack};

/// A source rectangle `[x, y, w, h]` in sheet pixels.
pub type Rect = [u32; 4];

#[derive(Debug)]
pub enum SheetError {
    Json(serde_json::Error),
    Png(String),
    UnknownAnimation(String),
    UnknownAsset(String),
    /// An animation has neither `rects` nor `row` + `frames`.
    NoFrames(String),
    /// A frame rectangle reaches outside the sheet image.
    RectOutOfBounds {
        animation: String,
        rect: Rect,
    },
}

impl fmt::Display for SheetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(f, "invalid sheet JSON: {error}"),
            Self::Png(error) => write!(f, "invalid sheet PNG: {error}"),
            Self::UnknownAnimation(name) => write!(f, "unknown animation `{name}`"),
            Self::UnknownAsset(id) => write!(f, "unknown asset `{id}`"),
            Self::NoFrames(name) => write!(f, "animation `{name}` declares no frames"),
            Self::RectOutOfBounds { animation, rect } => {
                write!(
                    f,
                    "animation `{animation}` frame {rect:?} is outside the sheet"
                )
            }
        }
    }
}

impl std::error::Error for SheetError {}

impl From<serde_json::Error> for SheetError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Animation {
    pub fps: f32,
    #[serde(rename = "loop", default)]
    pub looping: bool,
    #[serde(default)]
    row: Option<u32>,
    #[serde(default)]
    frames: Option<u32>,
    #[serde(default)]
    rects: Vec<Rect>,
}

impl Animation {
    pub fn frame_duration(&self) -> f32 {
        1.0 / self.fps
    }

    /// Seconds a non-looping play takes from first frame to last.
    pub fn duration(&self, frame_count: u32) -> f32 {
        frame_count as f32 * self.frame_duration()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SpriteSheet {
    /// Image path, relative to the document that declares the sheet.
    pub image: String,
    pub frame_size: [u32; 2],
    /// The point frames are anchored on (a character's feet), in frame pixels.
    #[serde(default)]
    pub origin_feet: Option<[u32; 2]>,
    pub animations: BTreeMap<String, Animation>,
}

impl SpriteSheet {
    pub fn parse(json: &str) -> Result<Self, SheetError> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn animation(&self, name: &str) -> Result<&Animation, SheetError> {
        self.animations
            .get(name)
            .ok_or_else(|| SheetError::UnknownAnimation(name.to_string()))
    }

    /// The animation's frame rectangles, expanding the `row` shorthand.
    pub fn rects(&self, name: &str) -> Result<Vec<Rect>, SheetError> {
        let animation = self.animation(name)?;
        if !animation.rects.is_empty() {
            return Ok(animation.rects.clone());
        }
        let (Some(row), Some(frames)) = (animation.row, animation.frames) else {
            return Err(SheetError::NoFrames(name.to_string()));
        };
        let [w, h] = self.frame_size;
        Ok((0..frames)
            .map(|index| [index * w, row * h, w, h])
            .collect())
    }

    /// Re-packs one animation into a single-row PNG strip, the only layout
    /// bones can play. `shift` moves every frame's content by that many
    /// source pixels — used to put `origin_feet` where the engine anchors.
    pub fn cut_strip(
        &self,
        png_bytes: &[u8],
        name: &str,
        shift: (i32, i32),
    ) -> Result<Vec<u8>, SheetError> {
        let rects = self.rects(name)?;
        strip::cut(png_bytes, name, &rects, self.frame_size, shift)
    }

    /// The shift that puts `origin_feet` on `target` (frame pixels).
    pub fn shift_feet_to(&self, target: (i32, i32)) -> (i32, i32) {
        match self.origin_feet {
            Some([x, y]) => (target.0 - x as i32, target.1 - y as i32),
            None => (0, 0),
        }
    }

    /// The presentation of a strip produced by [`Self::cut_strip`], loaded
    /// under `sprite_id` and drawn at `draw_size` world pixels.
    pub fn presentation(
        &self,
        name: &str,
        sprite_id: u32,
        draw_size: [u32; 2],
    ) -> Result<SpritePresentation, SheetError> {
        let animation = self.animation(name)?;
        let frame_count = self.rects(name)?.len() as u32;
        let [w, h] = self.frame_size;
        Ok(SpritePresentation {
            sprite: Sprite {
                sprite_id,
                frame_w: w,
                frame_h: h,
                frame_count,
                frame_duration: animation.frame_duration(),
            },
            frames_per_row: frame_count,
            draw_w: draw_size[0],
            draw_h: draw_size[1],
            looping: animation.looping,
            advance_while_stopped: true,
            flip_h: false,
            flip_v: false,
        })
    }
}

#[cfg(test)]
mod tests;
