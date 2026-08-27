use crate::identities::{BoundaryEpoch, BoundaryHandle, EquivalenceBasisId};

/// Candidate identity for non-authoritative branch-local work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchCandidateId(BoundaryHandle);

impl FoundationalBranchCandidateId {
    pub const fn new(handle: BoundaryHandle) -> Self {
        Self(handle)
    }

    pub const fn handle(&self) -> BoundaryHandle {
        self.0
    }
}

/// Milestone 5's epoch-shaped fork fact. It is not an exact branch reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBranchCandidateForkBasis {
    forked_from_branch: super::identity::FoundationalBranchId,
    fork_epoch: BoundaryEpoch,
}

impl FoundationalBranchCandidateForkBasis {
    pub fn new(
        forked_from_branch: super::identity::FoundationalBranchId,
        fork_epoch: BoundaryEpoch,
    ) -> Self {
        Self {
            forked_from_branch,
            fork_epoch,
        }
    }

    pub fn forked_from_branch(&self) -> &super::identity::FoundationalBranchId {
        &self.forked_from_branch
    }

    pub const fn fork_epoch(&self) -> BoundaryEpoch {
        self.fork_epoch
    }
}

/// Milestone 5's equivalence/epoch observation fact. It is not an exact
/// branch reference and cannot be converted into one without owner state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchCandidateObservationBasis {
    basis_id: EquivalenceBasisId,
    observed_epoch: BoundaryEpoch,
}

impl FoundationalBranchCandidateObservationBasis {
    pub const fn new(basis_id: EquivalenceBasisId, observed_epoch: BoundaryEpoch) -> Self {
        Self {
            basis_id,
            observed_epoch,
        }
    }

    pub const fn basis_id(&self) -> EquivalenceBasisId {
        self.basis_id
    }

    pub const fn observed_epoch(&self) -> BoundaryEpoch {
        self.observed_epoch
    }
}

/// Milestone 5's fork-observation fact, retained for candidate artifacts only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchCandidateForkObservationBasis {
    basis_id: EquivalenceBasisId,
    fork_epoch: BoundaryEpoch,
}

impl FoundationalBranchCandidateForkObservationBasis {
    pub const fn new(basis_id: EquivalenceBasisId, fork_epoch: BoundaryEpoch) -> Self {
        Self {
            basis_id,
            fork_epoch,
        }
    }

    pub const fn basis_id(&self) -> EquivalenceBasisId {
        self.basis_id
    }

    pub const fn fork_epoch(&self) -> BoundaryEpoch {
        self.fork_epoch
    }
}

/// Milestone 5's candidate comparison fact, not an operational comparison
/// against a complete exact reference observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBranchCandidateComparisonBasis {
    basis_id: EquivalenceBasisId,
    compared_against_branch: super::identity::FoundationalBranchId,
}

impl FoundationalBranchCandidateComparisonBasis {
    pub fn new(
        basis_id: EquivalenceBasisId,
        compared_against_branch: super::identity::FoundationalBranchId,
    ) -> Self {
        Self {
            basis_id,
            compared_against_branch,
        }
    }

    pub const fn basis_id(&self) -> EquivalenceBasisId {
        self.basis_id
    }

    pub fn compared_against_branch(&self) -> &super::identity::FoundationalBranchId {
        &self.compared_against_branch
    }
}
