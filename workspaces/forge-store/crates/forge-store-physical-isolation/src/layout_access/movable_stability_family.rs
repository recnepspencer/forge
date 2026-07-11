use crate::{
    FutureBlobMigrationNonClaimReport, TierMovementStabilityCounterSnapshot,
    TierMovementStabilityDenial, UnsupportedTierMovementRequest,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPlacementLayoutRule {
    _private: (),
}

impl AdmittedPlacementLayoutRule {
    pub(crate) const fn internal_phase20() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase20-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase20() -> Self {
        Self::internal_phase20()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MovableStabilityLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlacementLayoutFamilyAdmission;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlacementLayoutReport {
    placement_map_state: PlacementResidencyMapState,
    non_claims: FutureBlobMigrationNonClaimReport,
    counters: TierMovementStabilityCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementResidencyMapState {
    UnmaterializedInPhase20,
}

impl MovableStabilityLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        &self,
        _rule: &AdmittedPlacementLayoutRule,
    ) -> Result<PlacementLayoutFamilyAdmission, TierMovementStabilityDenial> {
        Ok(PlacementLayoutFamilyAdmission)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedPlacementLayoutFamily {
    _admission: PlacementLayoutFamilyAdmission,
}

impl AdmittedPlacementLayoutFamily {
    pub const fn new(admission: PlacementLayoutFamilyAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn placement_residency_map(&self) -> PlacementLayoutReport {
        PlacementLayoutReport {
            placement_map_state: PlacementResidencyMapState::UnmaterializedInPhase20,
            non_claims: FutureBlobMigrationNonClaimReport::s5_stability_only(),
            counters: TierMovementStabilityCounterSnapshot::default()
                .with_stability_admission()
                .with_chunk_placeholder(),
        }
    }

    pub const fn reject_projection_as_data_authority(
        &self,
        request: UnsupportedTierMovementRequest,
    ) -> Result<(), TierMovementStabilityDenial> {
        let _ = request;
        Err(TierMovementStabilityDenial::UnsupportedTierMovement)
    }
}

impl PlacementLayoutReport {
    pub const fn placement_map_state(self) -> PlacementResidencyMapState {
        self.placement_map_state
    }

    pub const fn non_claims(self) -> FutureBlobMigrationNonClaimReport {
        self.non_claims
    }

    pub const fn counters(self) -> TierMovementStabilityCounterSnapshot {
        self.counters
    }
}
