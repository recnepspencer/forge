use forge_store_contracts::StableDigest;

use crate::BlobPlacementClass;

use super::{
    basis::BlobPlacementMovementBasis, cold_outcome::BlobPlacementMovementColdOutcome,
    read_hold::BlobPlacementMovementReadHold,
};
use crate::placement::movement::counters::BlobPlacementMovementCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobPlacementMovementPhysicalExecutionIntent {
    pub(crate) basis_digest: StableDigest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBlobPlacementMovementPlan {
    pub(crate) basis: BlobPlacementMovementBasis,
    pub(crate) source_class: BlobPlacementClass,
    pub(crate) target_class: BlobPlacementClass,
    pub(crate) read_hold: BlobPlacementMovementReadHold,
    pub(crate) cold_outcome: BlobPlacementMovementColdOutcome,
    pub(crate) counters: BlobPlacementMovementCounterSnapshot,
}

impl AdmittedBlobPlacementMovementPlan {
    pub const fn counters(&self) -> BlobPlacementMovementCounterSnapshot {
        self.counters
    }

    pub const fn source_class(&self) -> BlobPlacementClass {
        self.source_class
    }

    pub const fn target_class(&self) -> BlobPlacementClass {
        self.target_class
    }

    pub const fn read_hold(&self) -> BlobPlacementMovementReadHold {
        self.read_hold
    }

    pub const fn cold_outcome(&self) -> BlobPlacementMovementColdOutcome {
        self.cold_outcome
    }

    pub(crate) const fn basis(&self) -> &BlobPlacementMovementBasis {
        &self.basis
    }

    pub(crate) fn physical_execution_basis_digest(&self) -> StableDigest {
        self.basis
            .physical_execution_basis_digest(self.source_class, self.target_class)
    }
}

impl BlobPlacementMovementPhysicalExecutionIntent {
    pub(crate) const fn basis_digest(&self) -> &StableDigest {
        &self.basis_digest
    }
}
