use worth_store_physical_backend::{
    BlobBackendResidueObservation, StoreExternalPlacementRecoverabilityEvidence,
};
use worth_store_tiering::{S7ColdPlacementState, S7PlacementIoReadinessSeed};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementClass {
    Inline,
    External,
    Cold,
}

#[derive(Debug, Clone)]
pub struct BlobPlacementIntent {
    class: BlobPlacementClass,
    readiness: S7PlacementIoReadinessSeed,
    cold_state: Option<S7ColdPlacementState>,
    external_recoverability: Option<StoreExternalPlacementRecoverabilityEvidence>,
    external_sidecar_denial: Option<BlobBackendResidueObservation>,
}

impl BlobPlacementIntent {
    pub fn inline(readiness: S7PlacementIoReadinessSeed) -> Self {
        Self {
            class: BlobPlacementClass::Inline,
            readiness,
            cold_state: None,
            external_recoverability: None,
            external_sidecar_denial: None,
        }
    }

    pub fn external(
        readiness: S7PlacementIoReadinessSeed,
        recoverability: StoreExternalPlacementRecoverabilityEvidence,
    ) -> Self {
        Self {
            class: BlobPlacementClass::External,
            readiness,
            cold_state: None,
            external_recoverability: Some(recoverability),
            external_sidecar_denial: None,
        }
    }

    pub fn external_sidecar_without_store_authority(
        readiness: S7PlacementIoReadinessSeed,
        denial: BlobBackendResidueObservation,
    ) -> Self {
        Self {
            class: BlobPlacementClass::External,
            readiness,
            cold_state: None,
            external_recoverability: None,
            external_sidecar_denial: Some(denial),
        }
    }

    pub fn cold(readiness: S7PlacementIoReadinessSeed, state: S7ColdPlacementState) -> Self {
        Self {
            class: BlobPlacementClass::Cold,
            readiness,
            cold_state: Some(state),
            external_recoverability: None,
            external_sidecar_denial: None,
        }
    }

    pub const fn class(&self) -> BlobPlacementClass {
        self.class
    }

    pub const fn readiness(&self) -> &S7PlacementIoReadinessSeed {
        &self.readiness
    }

    pub const fn cold_state(&self) -> Option<S7ColdPlacementState> {
        self.cold_state
    }

    pub fn external_recoverability(&self) -> Option<&StoreExternalPlacementRecoverabilityEvidence> {
        self.external_recoverability.as_ref()
    }

    pub fn external_sidecar_denial(&self) -> Option<&BlobBackendResidueObservation> {
        self.external_sidecar_denial.as_ref()
    }
}
