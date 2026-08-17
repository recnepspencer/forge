//! Object-safe borrowed protocol for the runtime/native raster transaction.
//!
//! The protocol carries only borrowed demand, miss, pin, and raster-batch
//! facts.  Atlas plans, reservations, GPU owners, and recovery authorities
//! remain native-owned.

use super::{
    UiAlphaRasterBatchView, UiColorRasterBatchView, UiGlyphRasterDemandIdentity,
    UiGlyphRasterDemandRecord, UiGlyphRasterKey, UiGlyphRasterLane, UiQualifiedTextLayoutIdentity,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiGlyphRasterPinRequest {
    layout: UiQualifiedTextLayoutIdentity,
    key: UiGlyphRasterKey,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedTextPinReleaseRequest {
    surface: crate::UiHostSurfaceRegistrationRequest,
    attempt: crate::UiMountedPresentationAttemptIdentity,
}

#[doc(hidden)]
impl UiGlyphRasterPinRequest {
    pub const fn from_text_mechanics(
        layout: UiQualifiedTextLayoutIdentity,
        key: UiGlyphRasterKey,
    ) -> Self {
        Self { layout, key }
    }

    pub const fn layout_identity(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }

    pub const fn key(self) -> UiGlyphRasterKey {
        self.key
    }
}

#[doc(hidden)]
impl UiMountedTextPinReleaseRequest {
    pub const fn from_runtime(
        surface: crate::UiHostSurfaceRegistrationRequest,
        attempt: crate::UiMountedPresentationAttemptIdentity,
    ) -> Self {
        Self { surface, attempt }
    }

    pub const fn surface(self) -> crate::UiHostSurfaceRegistrationRequest {
        self.surface
    }

    pub const fn attempt(self) -> crate::UiMountedPresentationAttemptIdentity {
        self.attempt
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UiGlyphRasterPinTransitionView<'pins> {
    additions: &'pins [UiGlyphRasterPinRequest],
    releases: &'pins [UiGlyphRasterPinRequest],
}

#[doc(hidden)]
impl<'pins> UiGlyphRasterPinTransitionView<'pins> {
    pub const fn from_text_mechanics(
        additions: &'pins [UiGlyphRasterPinRequest],
        releases: &'pins [UiGlyphRasterPinRequest],
    ) -> Self {
        Self {
            additions,
            releases,
        }
    }

    pub const fn additions(self) -> &'pins [UiGlyphRasterPinRequest] {
        self.additions
    }

    pub const fn releases(self) -> &'pins [UiGlyphRasterPinRequest] {
        self.releases
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UiGlyphRasterMissSelectionView<'miss> {
    demand: UiGlyphRasterDemandIdentity,
    layout: UiQualifiedTextLayoutIdentity,
    lane: UiGlyphRasterLane,
    records: &'miss [UiGlyphRasterDemandRecord],
}

#[doc(hidden)]
impl<'miss> UiGlyphRasterMissSelectionView<'miss> {
    pub const fn from_text_mechanics(
        demand: UiGlyphRasterDemandIdentity,
        layout: UiQualifiedTextLayoutIdentity,
        lane: UiGlyphRasterLane,
        records: &'miss [UiGlyphRasterDemandRecord],
    ) -> Self {
        Self {
            demand,
            layout,
            lane,
            records,
        }
    }

    pub const fn demand_identity(self) -> UiGlyphRasterDemandIdentity {
        self.demand
    }

    pub const fn layout_identity(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }

    pub const fn lane(self) -> UiGlyphRasterLane {
        self.lane
    }

    pub const fn records(self) -> &'miss [UiGlyphRasterDemandRecord] {
        self.records
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterBatchSubmissionDenial {
    WrongDemand,
    WrongMiss,
    WrongLayout,
    WrongBatch,
    WrongSource,
    Duplicate,
    Malformed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterCallbackDenial {
    Rejected,
    DemandMismatch,
    RasterizationDenied,
    BatchRejected(UiGlyphRasterBatchSubmissionDenial),
}

/// Sink called while a native move-only plan is still live.  Implementations
/// must consume the borrowed batch during the call; no batch lifetime escapes.
pub trait UiGlyphRasterBatchSink {
    fn submit_alpha(
        &mut self,
        batch: UiAlphaRasterBatchView<'_, '_>,
    ) -> Result<(), UiGlyphRasterBatchSubmissionDenial>;

    fn submit_color(
        &mut self,
        batch: UiColorRasterBatchView<'_, '_>,
    ) -> Result<(), UiGlyphRasterBatchSubmissionDenial>;
}

/// Text-owned callback invoked only for the exact native-admitted miss set.
/// The trait is deliberately object-safe so runtime/native orchestration does
/// not need a generic callback, `Any`, serialization, or a downcast lane.
pub trait UiGlyphRasterMissRasterizer {
    fn rasterize(
        &mut self,
        misses: UiGlyphRasterMissSelectionView<'_>,
        sink: &mut dyn UiGlyphRasterBatchSink,
    ) -> Result<(), UiGlyphRasterCallbackDenial>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGlyphRasterTransactionDenial {
    Unsupported,
    MalformedDemand,
    StaleDemand,
    CapacityExceeded,
    PinnedCapacityExceeded,
    ReservationConflict,
    GenerationExhausted,
    StalePlan,
    StalePin,
    ReconstructionRequired,
    RasterGeometryMismatch,
    RasterBatchMismatch,
    UploadRejected,
    CallbackRejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterEffectsIndeterminate {
    demand: UiGlyphRasterDemandIdentity,
    generation: u64,
}

/// Inert observation returned after native work has been queued but before
/// the owning host has observed physical completion.
///
/// This record deliberately carries no atlas/page/upload handle.  The live
/// completion owner remains in the native host; callers must present this
/// exact observation back to that owner through the governed adapter seam.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterTransactionPending {
    demand: UiGlyphRasterDemandIdentity,
    generation: u64,
    transaction: u64,
    host_session: u64,
}

#[doc(hidden)]
impl UiGlyphRasterTransactionPending {
    pub const fn from_text_mechanics(
        demand: UiGlyphRasterDemandIdentity,
        generation: u64,
        transaction: u64,
        host_session: u64,
    ) -> Self {
        Self {
            demand,
            generation,
            transaction,
            host_session,
        }
    }

    pub const fn demand_identity(self) -> UiGlyphRasterDemandIdentity {
        self.demand
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn transaction(self) -> u64 {
        self.transaction
    }

    pub const fn host_session(self) -> u64 {
        self.host_session
    }
}

#[doc(hidden)]
impl UiGlyphRasterEffectsIndeterminate {
    pub const fn from_text_mechanics(demand: UiGlyphRasterDemandIdentity, generation: u64) -> Self {
        Self { demand, generation }
    }

    pub const fn demand_identity(self) -> UiGlyphRasterDemandIdentity {
        self.demand
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGlyphRasterTransactionReceipt {
    generation: u64,
    misses: u32,
    hits: u32,
    evictions: u32,
    committed_pins: u32,
    staged_bytes: u64,
    physical_staged_bytes: u64,
    peak_entries: u32,
    peak_texel_bytes: u64,
}

#[doc(hidden)]
impl UiGlyphRasterTransactionReceipt {
    pub const fn from_text_mechanics(
        generation: u64,
        misses: u32,
        hits: u32,
        evictions: u32,
        committed_pins: u32,
        staged_bytes: u64,
        physical_staged_bytes: u64,
        peak_entries: u32,
        peak_texel_bytes: u64,
    ) -> Self {
        Self {
            generation,
            misses,
            hits,
            evictions,
            committed_pins,
            staged_bytes,
            physical_staged_bytes,
            peak_entries,
            peak_texel_bytes,
        }
    }

    pub const fn generation(self) -> u64 {
        self.generation
    }

    pub const fn misses(self) -> u32 {
        self.misses
    }

    pub const fn hits(self) -> u32 {
        self.hits
    }

    pub const fn evictions(self) -> u32 {
        self.evictions
    }

    pub const fn committed_pins(self) -> u32 {
        self.committed_pins
    }

    pub const fn staged_bytes(self) -> u64 {
        self.staged_bytes
    }

    pub const fn physical_staged_bytes(self) -> u64 {
        self.physical_staged_bytes
    }

    pub const fn peak_entries(self) -> u32 {
        self.peak_entries
    }

    pub const fn peak_texel_bytes(self) -> u64 {
        self.peak_texel_bytes
    }
}

#[derive(Debug, PartialEq)]
pub enum UiGlyphRasterTransactionOutcome {
    RejectedBeforeEffects(UiGlyphRasterTransactionDenial),
    RejectedAfterRasterization(UiGlyphRasterTransactionDenial),
    Pending(UiGlyphRasterTransactionPending),
    Committed(UiGlyphRasterTransactionReceipt),
    EffectsIndeterminate(UiGlyphRasterEffectsIndeterminate),
}

impl UiGlyphRasterTransactionOutcome {
    pub const fn is_committed(&self) -> bool {
        matches!(self, Self::Committed(_))
    }

    pub const fn is_pending(&self) -> bool {
        matches!(self, Self::Pending(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ObjectSafeProbe;

    impl UiGlyphRasterMissRasterizer for ObjectSafeProbe {
        fn rasterize(
            &mut self,
            _misses: UiGlyphRasterMissSelectionView<'_>,
            _sink: &mut dyn UiGlyphRasterBatchSink,
        ) -> Result<(), UiGlyphRasterCallbackDenial> {
            Ok(())
        }
    }

    #[test]
    fn callback_protocol_is_object_safe_and_outcome_is_typed() {
        let mut callback: Box<dyn UiGlyphRasterMissRasterizer> = Box::new(ObjectSafeProbe);
        let _ = &mut callback;
        assert!(!UiGlyphRasterTransactionOutcome::EffectsIndeterminate(
            UiGlyphRasterEffectsIndeterminate::from_text_mechanics(
                UiGlyphRasterDemandIdentity::from_text_mechanics([0; 32]),
                1,
            )
        )
        .is_committed());
        assert_eq!(
            UiGlyphRasterTransactionDenial::CallbackRejected,
            UiGlyphRasterTransactionDenial::CallbackRejected
        );
    }
}
