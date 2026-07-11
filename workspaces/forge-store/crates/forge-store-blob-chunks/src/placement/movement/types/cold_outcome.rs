use forge_store_tiering::{cold_posture_permits_movement, ColdPlacementState};

use crate::placement::movement::classification::cold_lane_decision_table::{
    classify_cold_lane_outcome, map_classified_cold_lane_outcome, ClassifiedColdLaneOutcome,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobPlacementMovementColdOutcome {
    state: ColdPlacementState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementMovementColdReadOutcome {
    Allowed,
    Retry,
    DeniedUnavailable,
    DeniedStale,
    DeniedScope,
    RebindRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementMovementColdExportOutcome {
    Allowed,
    Retry,
    DeniedUnavailable,
    DeniedStale,
    DeniedScope,
    RebindRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementMovementColdCapsuleOutcome {
    Allowed,
    Retry,
    DeniedUnavailable,
    DeniedStale,
    DeniedScope,
    RebindRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobPlacementMovementColdMaterializationOutcome {
    Allowed,
    Retry,
    DeniedUnavailable,
    DeniedStale,
    DeniedScope,
    RebindRequired,
}

impl BlobPlacementMovementColdOutcome {
    pub const fn from_state(state: ColdPlacementState) -> Self {
        Self { state }
    }

    pub const fn state(self) -> ColdPlacementState {
        self.state
    }

    pub const fn permits_movement(self) -> bool {
        cold_posture_permits_movement(self.state)
    }

    const fn classified_outcome(self) -> ClassifiedColdLaneOutcome {
        classify_cold_lane_outcome(self.state)
    }

    pub const fn read_outcome(self) -> BlobPlacementMovementColdReadOutcome {
        map_classified_cold_lane_outcome!(
            self.classified_outcome(),
            BlobPlacementMovementColdReadOutcome::Allowed,
            BlobPlacementMovementColdReadOutcome::Retry,
            BlobPlacementMovementColdReadOutcome::DeniedUnavailable,
            BlobPlacementMovementColdReadOutcome::DeniedStale,
            BlobPlacementMovementColdReadOutcome::DeniedScope,
            BlobPlacementMovementColdReadOutcome::RebindRequired
        )
    }

    pub const fn export_outcome(self) -> BlobPlacementMovementColdExportOutcome {
        map_classified_cold_lane_outcome!(
            self.classified_outcome(),
            BlobPlacementMovementColdExportOutcome::Allowed,
            BlobPlacementMovementColdExportOutcome::Retry,
            BlobPlacementMovementColdExportOutcome::DeniedUnavailable,
            BlobPlacementMovementColdExportOutcome::DeniedStale,
            BlobPlacementMovementColdExportOutcome::DeniedScope,
            BlobPlacementMovementColdExportOutcome::RebindRequired
        )
    }

    pub const fn capsule_outcome(self) -> BlobPlacementMovementColdCapsuleOutcome {
        map_classified_cold_lane_outcome!(
            self.classified_outcome(),
            BlobPlacementMovementColdCapsuleOutcome::Allowed,
            BlobPlacementMovementColdCapsuleOutcome::Retry,
            BlobPlacementMovementColdCapsuleOutcome::DeniedUnavailable,
            BlobPlacementMovementColdCapsuleOutcome::DeniedStale,
            BlobPlacementMovementColdCapsuleOutcome::DeniedScope,
            BlobPlacementMovementColdCapsuleOutcome::RebindRequired
        )
    }

    pub const fn materialization_outcome(self) -> BlobPlacementMovementColdMaterializationOutcome {
        map_classified_cold_lane_outcome!(
            self.classified_outcome(),
            BlobPlacementMovementColdMaterializationOutcome::Allowed,
            BlobPlacementMovementColdMaterializationOutcome::Retry,
            BlobPlacementMovementColdMaterializationOutcome::DeniedUnavailable,
            BlobPlacementMovementColdMaterializationOutcome::DeniedStale,
            BlobPlacementMovementColdMaterializationOutcome::DeniedScope,
            BlobPlacementMovementColdMaterializationOutcome::RebindRequired
        )
    }
}

impl From<ColdPlacementState> for BlobPlacementMovementColdOutcome {
    fn from(state: ColdPlacementState) -> Self {
        Self::from_state(state)
    }
}
