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
pub enum BlobPlacementIntent<'evidence> {
    Inline,
    External {
        recoverability: &'evidence StoreExternalPlacementRecoverabilityEvidence,
    },
    ExternalSidecarWithoutStoreAuthority {
        observation: &'evidence BlobBackendResidueObservation,
    },
    Cold {
        posture: &'evidence ColdTierIoPosture,
        state: ColdPlacementState,
    },
}

impl<'evidence> BlobPlacementIntent<'evidence> {
    pub const fn inline() -> Self {
        Self::Inline
    }

    pub const fn external(
        recoverability: &'evidence StoreExternalPlacementRecoverabilityEvidence,
    ) -> Self {
        Self::External { recoverability }
    }

    pub const fn external_sidecar_without_store_authority(
        observation: &'evidence BlobBackendResidueObservation,
    ) -> Self {
        Self::ExternalSidecarWithoutStoreAuthority { observation }
    }

    pub const fn cold(posture: &'evidence ColdTierIoPosture, state: ColdPlacementState) -> Self {
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

    pub const fn cold_posture(&self) -> Option<&'evidence ColdTierIoPosture> {
        match self {
            Self::Cold { posture, .. } => Some(*posture),
            _ => None,
        }
    }

    pub const fn cold_state(&self) -> Option<ColdPlacementState> {
        match self {
            Self::Cold { state, .. } => Some(*state),
            _ => None,
        }
    }

    pub const fn external_recoverability(
        &self,
    ) -> Option<&'evidence StoreExternalPlacementRecoverabilityEvidence> {
        match self {
            Self::External { recoverability } => Some(*recoverability),
            _ => None,
        }
    }

    pub const fn external_sidecar_denial(
        &self,
    ) -> Option<&'evidence BlobBackendResidueObservation> {
        match self {
            Self::ExternalSidecarWithoutStoreAuthority { observation } => Some(*observation),
            _ => None,
        }
    }
}
