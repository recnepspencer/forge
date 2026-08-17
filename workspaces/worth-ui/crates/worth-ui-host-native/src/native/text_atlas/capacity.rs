//! Exact qualified atlas capacity dimensions and staging limits.

use super::recovery::UiNativeTextAtlasDenial;
use worth_ui_host_contract::UiGlyphRasterSource;

pub(crate) const MAX_ENTRIES: usize = 8_192;
pub(crate) const MAX_EXTENT: u32 = 512;
pub(crate) const MAX_STAGED_BYTES: u64 = 8 * 1_024 * 1_024;
pub(crate) const MAX_ATLAS_TEXEL_BYTES: u64 = 36 * 1_024 * 1_024;

pub(crate) fn physical_staging_bytes(
    width: u32,
    height: u32,
    source: UiGlyphRasterSource,
) -> Option<u64> {
    let row_bytes = u64::from(width).checked_mul(source_channels(source))?;
    let padded_row = row_bytes.checked_add(255)? / 256 * 256;
    padded_row.checked_mul(u64::from(height))
}

pub(crate) const fn source_channels(source: UiGlyphRasterSource) -> u64 {
    match source {
        UiGlyphRasterSource::ColorOutline | UiGlyphRasterSource::ColorBitmap => 4,
        UiGlyphRasterSource::AlphaOutline | UiGlyphRasterSource::LastResort => 1,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeTextAtlasQualifiedCapacity {
    alpha_pages: u32,
    alpha_width: u32,
    alpha_height: u32,
    color_pages: u32,
    color_width: u32,
    color_height: u32,
    entries: u32,
    maximum_glyph_width: u32,
    maximum_glyph_height: u32,
    texel_bytes: u64,
    staged_upload_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeTextAtlasCapacityPosture {
    qualified: UiNativeTextAtlasQualifiedCapacity,
    live_entries: u32,
    live_pins: u32,
    retained_texel_bytes: u64,
    staged_bytes: u64,
}

impl UiNativeTextAtlasQualifiedCapacity {
    pub const QUALIFIED: Self = Self {
        alpha_pages: 4,
        alpha_width: 1024,
        alpha_height: 1024,
        color_pages: 2,
        color_width: 2048,
        color_height: 2048,
        entries: 8192,
        maximum_glyph_width: 512,
        maximum_glyph_height: 512,
        texel_bytes: 37_748_736,
        staged_upload_bytes: 8_388_608,
    };

    pub const fn alpha_pages(self) -> u32 {
        self.alpha_pages
    }
    pub const fn alpha_width(self) -> u32 {
        self.alpha_width
    }
    pub const fn alpha_height(self) -> u32 {
        self.alpha_height
    }
    pub const fn color_pages(self) -> u32 {
        self.color_pages
    }
    pub const fn color_width(self) -> u32 {
        self.color_width
    }
    pub const fn color_height(self) -> u32 {
        self.color_height
    }
    pub const fn entries(self) -> u32 {
        self.entries
    }
    pub const fn maximum_glyph_width(self) -> u32 {
        self.maximum_glyph_width
    }
    pub const fn maximum_glyph_height(self) -> u32 {
        self.maximum_glyph_height
    }
    pub const fn texel_bytes(self) -> u64 {
        self.texel_bytes
    }
    pub const fn staged_upload_bytes(self) -> u64 {
        self.staged_upload_bytes
    }
}

impl UiNativeTextAtlasCapacityPosture {
    #[allow(dead_code, reason = "reserved for native atlas effect ownership")]
    pub(crate) const fn from_native_host(
        live_entries: u32,
        live_pins: u32,
        retained_texel_bytes: u64,
        staged_bytes: u64,
    ) -> Result<Self, UiNativeTextAtlasDenial> {
        let qualified = UiNativeTextAtlasQualifiedCapacity::QUALIFIED;
        if live_entries > qualified.entries {
            return Err(UiNativeTextAtlasDenial::EntryCapacityExceeded);
        }
        if retained_texel_bytes > qualified.texel_bytes {
            return Err(UiNativeTextAtlasDenial::TexelCapacityExceeded);
        }
        if staged_bytes > qualified.staged_upload_bytes {
            return Err(UiNativeTextAtlasDenial::StagingCapacityExceeded);
        }
        Ok(Self {
            qualified,
            live_entries,
            live_pins,
            retained_texel_bytes,
            staged_bytes,
        })
    }

    pub const fn qualified(self) -> UiNativeTextAtlasQualifiedCapacity {
        self.qualified
    }
    pub const fn live_entries(self) -> u32 {
        self.live_entries
    }
    pub const fn live_pins(self) -> u32 {
        self.live_pins
    }
    pub const fn retained_texel_bytes(self) -> u64 {
        self.retained_texel_bytes
    }
    pub const fn staged_bytes(self) -> u64 {
        self.staged_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualified_capacity_matches_the_profile_caps() {
        let capacity = UiNativeTextAtlasQualifiedCapacity::QUALIFIED;
        assert_eq!(capacity.alpha_pages(), 4);
        assert_eq!(capacity.color_pages(), 2);
        assert_eq!(capacity.entries(), 8192);
        assert_eq!(capacity.maximum_glyph_width(), 512);
        assert_eq!(capacity.texel_bytes(), 37_748_736);
        assert_eq!(capacity.staged_upload_bytes(), 8_388_608);
        assert!(UiNativeTextAtlasCapacityPosture::from_native_host(1, 1, 16, 0).is_ok());
        let shared = UiNativeTextAtlasCapacityPosture::from_native_host(1, 2, 16, 0)
            .expect("two layouts may independently pin one shared raster entry");
        assert_eq!(shared.live_entries(), 1);
        assert_eq!(shared.live_pins(), 2);
    }
}
