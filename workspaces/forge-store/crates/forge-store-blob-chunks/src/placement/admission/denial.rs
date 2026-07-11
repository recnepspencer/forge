use forge_store_physical_backend::{
    BackendCapabilityAdmissionDenial, BlobBackendResidueObservation,
};
use forge_store_tiering::S7ColdPlacementState;

use super::BlobPlacementCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobPlacementAdmissionDenial {
    BackendCapability {
        source: BackendCapabilityAdmissionDenial,
        counters: BlobPlacementCounterSnapshot,
    },
    PlacementReadinessBasisMismatch {
        counters: BlobPlacementCounterSnapshot,
    },
    ExternalPlacementMissingRecoverability {
        counters: BlobPlacementCounterSnapshot,
    },
    ExternalPlacementRecoverabilityBasisMismatch {
        counters: BlobPlacementCounterSnapshot,
    },
    ExternalSidecarWithoutStoreAuthority {
        observation: BlobBackendResidueObservation,
        counters: BlobPlacementCounterSnapshot,
    },
    ColdChunkUnavailable {
        state: S7ColdPlacementState,
        counters: BlobPlacementCounterSnapshot,
    },
}
