use forge_store_tiering::S7ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobPlacementMovementColdOutcome {
    state: S7ColdPlacementState,
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
    pub const fn from_state(state: S7ColdPlacementState) -> Self {
        Self { state }
    }

    pub const fn state(self) -> S7ColdPlacementState {
        self.state
    }

    pub const fn permits_movement(self) -> bool {
        matches!(
            self.state,
            S7ColdPlacementState::HotAvailable | S7ColdPlacementState::ColdAvailable
        )
    }

    pub const fn read_outcome(self) -> BlobPlacementMovementColdReadOutcome {
        match self.state {
            S7ColdPlacementState::HotAvailable | S7ColdPlacementState::ColdAvailable => {
                BlobPlacementMovementColdReadOutcome::Allowed
            }
            S7ColdPlacementState::ColdFetchRequired | S7ColdPlacementState::ColdFetchInProgress => {
                BlobPlacementMovementColdReadOutcome::Retry
            }
            S7ColdPlacementState::ColdUnavailable => {
                BlobPlacementMovementColdReadOutcome::DeniedUnavailable
            }
            S7ColdPlacementState::ColdStale => BlobPlacementMovementColdReadOutcome::DeniedStale,
            S7ColdPlacementState::ColdScopeDenied => {
                BlobPlacementMovementColdReadOutcome::DeniedScope
            }
            S7ColdPlacementState::ColdRebindRequired => {
                BlobPlacementMovementColdReadOutcome::RebindRequired
            }
        }
    }

    pub const fn export_outcome(self) -> BlobPlacementMovementColdExportOutcome {
        match self.state {
            S7ColdPlacementState::HotAvailable | S7ColdPlacementState::ColdAvailable => {
                BlobPlacementMovementColdExportOutcome::Allowed
            }
            S7ColdPlacementState::ColdFetchRequired | S7ColdPlacementState::ColdFetchInProgress => {
                BlobPlacementMovementColdExportOutcome::Retry
            }
            S7ColdPlacementState::ColdUnavailable => {
                BlobPlacementMovementColdExportOutcome::DeniedUnavailable
            }
            S7ColdPlacementState::ColdStale => BlobPlacementMovementColdExportOutcome::DeniedStale,
            S7ColdPlacementState::ColdScopeDenied => {
                BlobPlacementMovementColdExportOutcome::DeniedScope
            }
            S7ColdPlacementState::ColdRebindRequired => {
                BlobPlacementMovementColdExportOutcome::RebindRequired
            }
        }
    }

    pub const fn capsule_outcome(self) -> BlobPlacementMovementColdCapsuleOutcome {
        match self.state {
            S7ColdPlacementState::HotAvailable | S7ColdPlacementState::ColdAvailable => {
                BlobPlacementMovementColdCapsuleOutcome::Allowed
            }
            S7ColdPlacementState::ColdFetchRequired | S7ColdPlacementState::ColdFetchInProgress => {
                BlobPlacementMovementColdCapsuleOutcome::Retry
            }
            S7ColdPlacementState::ColdUnavailable => {
                BlobPlacementMovementColdCapsuleOutcome::DeniedUnavailable
            }
            S7ColdPlacementState::ColdStale => BlobPlacementMovementColdCapsuleOutcome::DeniedStale,
            S7ColdPlacementState::ColdScopeDenied => {
                BlobPlacementMovementColdCapsuleOutcome::DeniedScope
            }
            S7ColdPlacementState::ColdRebindRequired => {
                BlobPlacementMovementColdCapsuleOutcome::RebindRequired
            }
        }
    }

    pub const fn materialization_outcome(self) -> BlobPlacementMovementColdMaterializationOutcome {
        match self.state {
            S7ColdPlacementState::HotAvailable | S7ColdPlacementState::ColdAvailable => {
                BlobPlacementMovementColdMaterializationOutcome::Allowed
            }
            S7ColdPlacementState::ColdFetchRequired | S7ColdPlacementState::ColdFetchInProgress => {
                BlobPlacementMovementColdMaterializationOutcome::Retry
            }
            S7ColdPlacementState::ColdUnavailable => {
                BlobPlacementMovementColdMaterializationOutcome::DeniedUnavailable
            }
            S7ColdPlacementState::ColdStale => {
                BlobPlacementMovementColdMaterializationOutcome::DeniedStale
            }
            S7ColdPlacementState::ColdScopeDenied => {
                BlobPlacementMovementColdMaterializationOutcome::DeniedScope
            }
            S7ColdPlacementState::ColdRebindRequired => {
                BlobPlacementMovementColdMaterializationOutcome::RebindRequired
            }
        }
    }
}

impl From<S7ColdPlacementState> for BlobPlacementMovementColdOutcome {
    fn from(state: S7ColdPlacementState) -> Self {
        Self::from_state(state)
    }
}
