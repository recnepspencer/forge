use forge_store_physical_backend::{
    BlobBackendResidueObservation, StoreExternalPlacementRecoverabilityEvidence,
};
use forge_store_tiering::{ColdPlacementState, TierPlacementIoAdmission};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementClass {
    Inline,
    External,
    Cold,
}

#[derive(Debug, Clone)]
pub struct BlobPlacementIntent {
    class: BlobPlacementClass,
    readiness: TierPlacementIoAdmission,
    cold_state: Option<ColdPlacementState>,
    external_recoverability: Option<StoreExternalPlacementRecoverabilityEvidence>,
    external_sidecar_denial: Option<BlobBackendResidueObservation>,
}

impl BlobPlacementIntent {
    pub fn inline(readiness: TierPlacementIoAdmission) -> Self {
        Self {
            class: BlobPlacementClass::Inline,
            readiness,
            cold_state: None,
            external_recoverability: None,
            external_sidecar_denial: None,
        }
    }

    pub fn external(
        readiness: TierPlacementIoAdmission,
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
        readiness: TierPlacementIoAdmission,
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

    pub fn cold(readiness: TierPlacementIoAdmission, state: ColdPlacementState) -> Self {
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

    pub const fn readiness(&self) -> &TierPlacementIoAdmission {
        &self.readiness
    }

    pub const fn cold_state(&self) -> Option<ColdPlacementState> {
        self.cold_state
    }

    pub fn external_recoverability(&self) -> Option<&StoreExternalPlacementRecoverabilityEvidence> {
        self.external_recoverability.as_ref()
    }

    pub fn external_sidecar_denial(&self) -> Option<&BlobBackendResidueObservation> {
        self.external_sidecar_denial.as_ref()
    }
}
