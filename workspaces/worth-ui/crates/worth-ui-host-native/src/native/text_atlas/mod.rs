//! Native-owned bounded atlas transaction and resource lifecycle.

mod admission;
mod alpha;
mod candidate_store;
mod capacity;
mod census;
mod cleanup;
mod color;
mod demand;
mod demand_admission;
mod entry;
#[cfg(test)]
mod eviction;
#[cfg(test)]
mod gate_d_model_evidence;
mod in_flight;
mod key;
mod ownership;
mod pinning;
mod placement;
mod planning;
mod raster_upload;
mod recovery;
mod settlement;
mod settling;
#[cfg(test)]
mod test_device_tests;
mod transaction;
#[cfg(test)]
mod transaction_plan_snapshot;
mod upload;
mod upload_staging;

#[cfg(test)]
mod boundary_tests;
#[cfg(test)]
mod content_extent_tests;
#[cfg(test)]
mod correlation_tests;
#[cfg(test)]
pub(crate) mod eviction_tests;
#[cfg(test)]
mod model_key;
#[cfg(test)]
mod model_oracle;
#[cfg(test)]
mod model_placement;
#[cfg(test)]
mod model_records;
#[cfg(test)]
mod model_tests;
#[cfg(test)]
mod ownership_tests;
#[cfg(test)]
mod pinning_capacity_tests;
#[cfg(test)]
mod placement_model_tests;
#[cfg(test)]
mod recovery_identity_tests;

pub use capacity::{UiNativeTextAtlasCapacityPosture, UiNativeTextAtlasQualifiedCapacity};
pub(crate) use census::UiNativeTextAtlasPhysicalPosture;
pub use census::{UiNativeTextAtlasCensus, UiNativeTextAtlasResourceClass};
#[cfg(test)]
pub(crate) use content_extent_tests::retained_content_extent_is_the_uploaded_shape_not_the_padded_allocation;
#[cfg(test)]
pub(crate) use correlation_tests::physical_transaction_correlation_rebinds_to_the_current_signal_attempt;
pub(crate) use demand::UiNativeTextAtlasDemand;
#[cfg(test)]
pub(crate) use gate_d_model_evidence::{
    assert_gate_d_model_boundaries, assert_independent_committed_transaction,
};
pub(crate) use in_flight::UiNativeTextAtlasInFlight;
pub(crate) use key::canonical_raster_key_bytes;
pub use key::{UiAtlasEntryIdentity, UiNativeValidatedRasterKey};
pub use ownership::UiNativeTextPinObservation;
pub(crate) use ownership::{UiNativeTextAtlas, UiNativeTextAtlasEntryView};
pub use pinning::{UiNativeTextAtlasPin, UiNativeTextAtlasPinSnapshot};
pub(crate) use raster_upload::UiNativeTextAtlasUpload;
pub use recovery::{
    UiNativeTextAtlasDenial, UiNativeTextAtlasGeneration, UiNativeTextAtlasLineageIdentity,
    UiNativeTextAtlasRecovery, UiNativeTextAtlasRecoverySnapshot,
};
pub(crate) use settlement::UiNativeTextAtlasCommitOutcome;
#[cfg(test)]
pub(crate) use test_device_tests::qualified_test_device;
pub(crate) use transaction::{
    UiNativeTextAtlasExternalOutcome, UiNativeTextAtlasPinRequest, UiNativeTextAtlasPinTransition,
    UiNativeTextAtlasTransactionPlan,
};
pub(crate) use upload::UiNativeGpuAtlasKind;
pub(crate) use upload::{
    UiNativeTextAtlasGpuBatchUpload, UiNativeTextAtlasGpuPages, UiNativeTextAtlasGpuUploadRequest,
    UiNativeTextAtlasPhysicalPoll,
};
