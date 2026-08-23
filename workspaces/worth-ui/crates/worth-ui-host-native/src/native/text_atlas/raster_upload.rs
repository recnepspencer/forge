//! Raster callback output admitted against one native atlas plan.

use super::capacity::source_channels;
use worth_ui_host_contract::{UiGlyphRasterBearing, UiGlyphRasterKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasUpload {
    key: UiGlyphRasterKey,
    bearing: UiGlyphRasterBearing,
    width: u32,
    height: u32,
    stride: u32,
    bytes: Box<[u8]>,
    digest: [u8; 32],
}

impl UiNativeTextAtlasUpload {
    #[doc(hidden)]
    pub fn from_text_mechanics(
        key: UiGlyphRasterKey,
        width: u32,
        height: u32,
        stride: u32,
        bytes: impl Into<Box<[u8]>>,
        digest: [u8; 32],
    ) -> Self {
        Self {
            key,
            bearing: UiGlyphRasterBearing::from_sixty_fourths(0, 0),
            width,
            height,
            stride,
            bytes: bytes.into(),
            digest,
        }
    }

    pub(crate) fn with_bearing_from_text_mechanics(
        key: UiGlyphRasterKey,
        bearing: UiGlyphRasterBearing,
        width: u32,
        height: u32,
        stride: u32,
        bytes: impl Into<Box<[u8]>>,
        digest: [u8; 32],
    ) -> Self {
        Self {
            key,
            bearing,
            width,
            height,
            stride,
            bytes: bytes.into(),
            digest,
        }
    }

    pub(crate) const fn key(&self) -> UiGlyphRasterKey {
        self.key
    }
    pub(crate) const fn bearing(&self) -> UiGlyphRasterBearing {
        self.bearing
    }
    pub(crate) const fn width(&self) -> u32 {
        self.width
    }
    pub(crate) const fn height(&self) -> u32 {
        self.height
    }
    pub(crate) const fn stride(&self) -> u32 {
        self.stride
    }
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub(crate) const fn digest(&self) -> [u8; 32] {
        self.digest
    }
}

pub(crate) fn upload_shape_is_valid(upload: &UiNativeTextAtlasUpload) -> bool {
    let channels = source_channels(upload.key().source());
    let Some(expected) = u64::from(upload.width())
        .checked_mul(u64::from(upload.height()))
        .and_then(|pixels| pixels.checked_mul(channels))
    else {
        return false;
    };
    u64::try_from(upload.bytes().len()).ok() == Some(expected)
        && upload.stride()
            == upload
                .width()
                .saturating_mul(u32::try_from(channels).unwrap_or(0))
}
