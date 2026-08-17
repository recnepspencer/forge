//! Move-only atlas transaction records and inert cross-domain upload views.
use super::candidate_store::CandidateAtlasStore;
use super::capacity::{physical_staging_bytes, source_channels};
use super::ownership::AtlasCore;
use super::recovery::UiNativeTextAtlasGeneration;
#[cfg(test)]
use super::transaction_plan_snapshot::UiNativeTextAtlasTransactionPlanSnapshot;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasPinRequest {
    layout: UiQualifiedTextLayoutIdentity,
    key: UiGlyphRasterKey,
}

impl UiNativeTextAtlasPinRequest {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        layout: UiQualifiedTextLayoutIdentity,
        key: UiGlyphRasterKey,
    ) -> Self {
        Self { layout, key }
    }

    pub(crate) const fn layout(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }

    pub(crate) const fn key(self) -> UiGlyphRasterKey {
        self.key
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasPinTransition {
    additions: Box<[UiNativeTextAtlasPinRequest]>,
    releases: Box<[UiNativeTextAtlasPinRequest]>,
}

impl UiNativeTextAtlasPinTransition {
    #[doc(hidden)]
    pub fn from_text_mechanics(
        additions: impl IntoIterator<Item = UiNativeTextAtlasPinRequest>,
        releases: impl IntoIterator<Item = UiNativeTextAtlasPinRequest>,
    ) -> Self {
        Self {
            additions: additions.into_iter().collect(),
            releases: releases.into_iter().collect(),
        }
    }

    pub(crate) fn additions(&self) -> &[UiNativeTextAtlasPinRequest] {
        &self.additions
    }

    pub(crate) fn releases(&self) -> &[UiNativeTextAtlasPinRequest] {
        &self.releases
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeTextAtlasUpload {
    key: UiGlyphRasterKey,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeTextAtlasExternalOutcome {
    Submitted,
    Rejected,
    EffectsIndeterminate,
}

pub(crate) struct UiNativeTextAtlasTransactionPlan {
    pub(crate) core: Rc<RefCell<AtlasCore>>,
    pub(crate) reservation: u64,
    pub(crate) demand_identity: UiGlyphRasterDemandIdentity,
    pub(crate) peak_entries: u32,
    pub(crate) peak_texel_bytes: u64,
    pub(crate) predecessor_generation: UiNativeTextAtlasGeneration,
    pub(crate) candidate_generation: UiNativeTextAtlasGeneration,
    pub(crate) misses: Box<[UiNativeTextAtlasDemand]>,
    pub(crate) hits: Box<[UiGlyphRasterKey]>,
    pub(crate) evictions: Box<[UiGlyphRasterKey]>,
    pub(crate) candidate_alpha: CandidateAtlasStore,
    pub(crate) candidate_color: CandidateAtlasStore,
    pub(crate) pin_additions: Box<[super::ownership::PinIdentity]>,
    pub(crate) pin_releases: Box<[super::ownership::PinIdentity]>,
    pub(crate) pin_change_keys: Box<[UiGlyphRasterKey]>,
    pub(crate) next_entry: u64,
    pub(crate) staged_bytes: u64,
    pub(crate) physical_staged_bytes: u64,
    pub(crate) committed: bool,
}

impl UiNativeTextAtlasTransactionPlan {
    pub(crate) const fn transaction_identity(&self) -> u64 {
        self.reservation
    }

    pub(crate) fn miss_demands(&self) -> &[UiNativeTextAtlasDemand] {
        &self.misses
    }

    pub(crate) fn placement_for(&self, key: UiGlyphRasterKey) -> Option<(u32, [u32; 2])> {
        let core = self.core.borrow();
        self.candidate_alpha
            .entry(&core.alpha, key)
            .or_else(|| self.candidate_color.entry(&core.color, key))
            .map(|entry| (entry.page, [entry.rect.x, entry.rect.y]))
    }

    pub(crate) fn candidate_page_counts(&self) -> (usize, usize) {
        (
            self.candidate_alpha.page_count(),
            self.candidate_color.page_count(),
        )
    }

    #[cfg(test)]
    pub(crate) fn hit_keys(&self) -> &[UiGlyphRasterKey] {
        &self.hits
    }

    #[cfg(test)]
    pub(crate) fn evicted_keys(&self) -> &[UiGlyphRasterKey] {
        &self.evictions
    }

    #[cfg(test)]
    pub(crate) fn candidate_overlay_entries(&self) -> usize {
        self.candidate_alpha.added_len() + self.candidate_color.added_len()
    }

    #[cfg(test)]
    pub(crate) const fn predecessor_generation(&self) -> UiNativeTextAtlasGeneration {
        self.predecessor_generation
    }

    #[cfg(test)]
    pub(crate) const fn candidate_generation(&self) -> UiNativeTextAtlasGeneration {
        self.candidate_generation
    }

    #[cfg(test)]
    pub(crate) const fn staged_bytes(&self) -> u64 {
        self.staged_bytes
    }

    pub(crate) const fn physical_staged_bytes(&self) -> u64 {
        self.physical_staged_bytes
    }

    #[cfg(test)]
    pub(crate) fn snapshot(&self) -> UiNativeTextAtlasTransactionPlanSnapshot {
        UiNativeTextAtlasTransactionPlanSnapshot {
            demand_identity: self.demand_identity,
            peak_entries: self.peak_entries,
            peak_texel_bytes: self.peak_texel_bytes,
            predecessor_generation: self.predecessor_generation,
            candidate_generation: self.candidate_generation,
            misses: u32::try_from(self.misses.len()).unwrap_or(u32::MAX),
            hits: u32::try_from(self.hits.len()).unwrap_or(u32::MAX),
            evictions: u32::try_from(self.evictions.len()).unwrap_or(u32::MAX),
            staged_bytes: self.staged_bytes,
            physical_staged_bytes: self.physical_staged_bytes,
        }
    }

    pub(crate) fn admits_uploads(&self, uploads: &[UiNativeTextAtlasUpload]) -> bool {
        if uploads.len() != self.misses.len() {
            return false;
        }
        let miss_keys = self
            .misses
            .iter()
            .map(|demand| demand.key)
            .collect::<HashSet<_>>();
        if miss_keys.len() != self.misses.len() {
            return false;
        }
        let mut upload_keys = HashSet::with_capacity(uploads.len());
        uploads.iter().all(|upload| {
            upload_keys.insert(upload.key)
                && miss_keys.contains(&upload.key)
                && self.admits_upload(upload)
                && upload_shape_is_valid(upload)
        }) && upload_keys == miss_keys
    }

    fn admits_upload(&self, upload: &UiNativeTextAtlasUpload) -> bool {
        self.misses.iter().any(|demand| {
            demand.key == upload.key
                && upload.width <= demand.width
                && upload.height <= demand.height
                && u64::try_from(upload.bytes.len()).unwrap_or(u64::MAX) <= demand.staged_bytes
                && upload.key.source() == demand.key.source()
        })
    }
}

impl Drop for UiNativeTextAtlasTransactionPlan {
    fn drop(&mut self) {
        if self.committed {
            return;
        }
        if let Ok(mut core) = self.core.try_borrow_mut() {
            if core.reservation == Some(self.reservation) {
                core.reservation = None;
            }
        }
    }
}

pub(crate) fn upload_shape_is_valid(upload: &UiNativeTextAtlasUpload) -> bool {
    let Some(expected) = u64::from(upload.width())
        .checked_mul(u64::from(upload.height()))
        .and_then(|pixels| pixels.checked_mul(source_channels(upload.key().source())))
    else {
        return false;
    };
    u64::try_from(upload.bytes().len()).ok() == Some(expected)
        && upload.stride()
            == upload
                .width()
                .saturating_mul(u32::try_from(source_channels(upload.key().source())).unwrap_or(0))
}
