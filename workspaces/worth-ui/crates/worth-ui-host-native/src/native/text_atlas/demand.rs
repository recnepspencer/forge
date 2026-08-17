//! Exact admitted glyph-demand provenance carried into native planning.

use super::capacity::physical_staging_bytes;
use worth_ui_host_contract::{
    UiGlyphRasterDemandBatchView, UiGlyphRasterDemandIdentity, UiGlyphRasterDemandRecord,
    UiGlyphRasterKey, UiGlyphRasterLane, UiQualifiedTextLayoutIdentity, UiTextScaleGeneration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasDemand {
    identity: UiGlyphRasterDemandIdentity,
    key: UiGlyphRasterKey,
    width: u32,
    height: u32,
    staged_bytes: u64,
    physical_staged_bytes: u64,
    source_identity: UiGlyphRasterDemandIdentity,
    source_layout: UiQualifiedTextLayoutIdentity,
    source_lane: UiGlyphRasterLane,
    source_dpi_milli: u32,
    source_text_scale: UiTextScaleGeneration,
    source_record: Option<UiGlyphRasterDemandRecord>,
}

impl UiNativeTextAtlasDemand {
    pub(super) fn from_host_contract(
        identity: UiGlyphRasterDemandIdentity,
        demand: UiGlyphRasterDemandBatchView<'_>,
        record: UiGlyphRasterDemandRecord,
    ) -> Self {
        Self {
            identity,
            key: record.key(),
            width: record.extent().width(),
            height: record.extent().height(),
            staged_bytes: record.staged_bytes(),
            physical_staged_bytes: physical_staging_bytes(
                record.extent().width(),
                record.extent().height(),
                record.key().source(),
            )
            .unwrap_or(u64::MAX),
            source_identity: demand.identity(),
            source_layout: demand.layout_identity(),
            source_lane: demand.lane(),
            source_dpi_milli: demand.dpi_milli(),
            source_text_scale: demand.text_scale_generation(),
            source_record: Some(record),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_native_geometry(
        identity: UiGlyphRasterDemandIdentity,
        key: UiGlyphRasterKey,
        width: u32,
        height: u32,
        staged_bytes: u64,
    ) -> Self {
        Self {
            identity,
            key,
            width,
            height,
            staged_bytes,
            physical_staged_bytes: physical_staging_bytes(width, height, key.source())
                .unwrap_or(u64::MAX),
            source_identity: identity,
            source_layout: UiQualifiedTextLayoutIdentity::from_text_mechanics([0; 32]),
            source_lane: UiGlyphRasterLane::Ordinary,
            source_dpi_milli: key.dpi_milli(),
            source_text_scale: UiTextScaleGeneration::new(1).expect("nonzero test scale"),
            source_record: None,
        }
    }

    pub(crate) const fn identity(self) -> UiGlyphRasterDemandIdentity {
        self.identity
    }
    pub(crate) const fn key(self) -> UiGlyphRasterKey {
        self.key
    }
    pub(crate) const fn width(self) -> u32 {
        self.width
    }
    pub(crate) const fn height(self) -> u32 {
        self.height
    }
    pub(crate) const fn staged_bytes(self) -> u64 {
        self.staged_bytes
    }
    pub(crate) const fn physical_staged_bytes(self) -> u64 {
        self.physical_staged_bytes
    }
    pub(crate) const fn source_identity(self) -> UiGlyphRasterDemandIdentity {
        self.source_identity
    }
    pub(crate) const fn source_layout(self) -> UiQualifiedTextLayoutIdentity {
        self.source_layout
    }
    pub(crate) const fn source_lane(self) -> UiGlyphRasterLane {
        self.source_lane
    }
    pub(crate) const fn source_dpi_milli(self) -> u32 {
        self.source_dpi_milli
    }
    pub(crate) const fn source_text_scale(self) -> UiTextScaleGeneration {
        self.source_text_scale
    }
    pub(crate) const fn source_record(self) -> Option<UiGlyphRasterDemandRecord> {
        self.source_record
    }
}
