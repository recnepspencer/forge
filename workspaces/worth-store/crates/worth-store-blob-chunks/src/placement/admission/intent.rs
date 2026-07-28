use worth_store_physical_backend::{
    BlobBackendResidueObservation, StoreExternalPlacementRecoverabilityEvidence,
};
use worth_store_tiering::{ColdPlacementState, ColdTierIoPosture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementClass {
    Inline,
    External,
    Cold,
}

#[derive(Debug, Clone)]
pub enum BlobPlacementIntent {
    Inline,
    External {
        recoverability: StoreExternalPlacementRecoverabilityEvidence,
    },
    ExternalSidecarWithoutStoreAuthority {
        observation: BlobBackendResidueObservation,
    },
    Cold {
        posture: ColdTierIoPosture,
        state: ColdPlacementState,
    },
}

impl BlobPlacementIntent {
    pub const fn inline() -> Self {
        Self::Inline
    }

    pub fn external(recoverability: StoreExternalPlacementRecoverabilityEvidence) -> Self {
        Self::External { recoverability }
    }

    pub fn external_sidecar_without_store_authority(
        observation: BlobBackendResidueObservation,
    ) -> Self {
        Self::ExternalSidecarWithoutStoreAuthority { observation }
    }

    pub fn cold(posture: ColdTierIoPosture, state: ColdPlacementState) -> Self {
        Self::Cold { posture, state }
    }

    pub const fn class(&self) -> BlobPlacementClass {
        match self {
            Self::Inline => BlobPlacementClass::Inline,
            Self::External { .. } | Self::ExternalSidecarWithoutStoreAuthority { .. } => {
                BlobPlacementClass::External
            }
            Self::Cold { .. } => BlobPlacementClass::Cold,
        }
    }

    pub const fn cold_posture(&self) -> Option<&ColdTierIoPosture> {
        match self {
            Self::Cold { posture, .. } => Some(posture),
            _ => None,
        }
    }

    pub const fn cold_state(&self) -> Option<ColdPlacementState> {
        match self {
            Self::Cold { state, .. } => Some(*state),
            _ => None,
        }
    }

    pub fn external_recoverability(&self) -> Option<&StoreExternalPlacementRecoverabilityEvidence> {
        match self {
            Self::External { recoverability } => Some(recoverability),
            _ => None,
        }
    }

    pub fn external_sidecar_denial(&self) -> Option<&BlobBackendResidueObservation> {
        match self {
            Self::ExternalSidecarWithoutStoreAuthority { observation } => Some(observation),
            _ => None,
        }
    }
}
