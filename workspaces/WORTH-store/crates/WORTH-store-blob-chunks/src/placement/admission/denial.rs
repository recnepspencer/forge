use worth_store_io_scheduler::S6LaterReadinessReadmissionState;
use worth_store_physical_backend::{
    BackendCapabilityAdmissionDenial, BlobBackendResidueObservation,
};
use worth_store_tiering::S7ColdPlacementState;

use super::BlobPlacementCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobPlacementAdmissionDenial {
    BackendCapability {
        source: BackendCapabilityAdmissionDenial,
        counters: BlobPlacementCounterSnapshot,
    },
    StaleS6Readiness {
        readmission: S6LaterReadinessReadmissionState,
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
