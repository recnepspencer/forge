use crate::identities::{BoundaryEpoch, BoundaryHandle, EquivalenceBasisId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalBranchLocalStateKind {
    Candidate,
    Staged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchLocalStateDefinition {
    kind: FoundationalBranchLocalStateKind,
    name: &'static str,
    intended_use: &'static str,
    must_not_mean: &'static str,
}

impl FoundationalBranchLocalStateDefinition {
    const fn new(
        kind: FoundationalBranchLocalStateKind,
        name: &'static str,
        intended_use: &'static str,
        must_not_mean: &'static str,
    ) -> Self {
        Self {
            kind,
            name,
            intended_use,
            must_not_mean,
        }
    }

    pub const fn kind(&self) -> FoundationalBranchLocalStateKind {
        self.kind
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn intended_use(&self) -> &'static str {
        self.intended_use
    }

    pub const fn must_not_mean(&self) -> &'static str {
        self.must_not_mean
    }
}

const CANDIDATE_STATE_DEFINITION: FoundationalBranchLocalStateDefinition =
    FoundationalBranchLocalStateDefinition::new(
        FoundationalBranchLocalStateKind::Candidate,
        "candidate",
        "branch-local work that has not yet been staged as a stronger branch-local snapshot",
        "merge meaning, committed authority, or receipt evidence",
    );
const STAGED_STATE_DEFINITION: FoundationalBranchLocalStateDefinition =
    FoundationalBranchLocalStateDefinition::new(
        FoundationalBranchLocalStateKind::Staged,
        "staged",
        "branch-local work that is ready for later merge planning while still remaining non-authoritative",
        "merge verdicts, committed authority, or receipt evidence",
    );

pub const fn foundational_branch_local_state_definitions(
) -> [FoundationalBranchLocalStateDefinition; 2] {
    [CANDIDATE_STATE_DEFINITION, STAGED_STATE_DEFINITION]
}

pub(crate) const fn candidate_state_definition() -> &'static FoundationalBranchLocalStateDefinition
{
    &CANDIDATE_STATE_DEFINITION
}

pub(crate) const fn staged_state_definition() -> &'static FoundationalBranchLocalStateDefinition {
    &STAGED_STATE_DEFINITION
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBranchIdConstructionDenial {
    EmptyName,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchId(String);

impl FoundationalBranchId {
    pub fn new(name: impl Into<String>) -> Result<Self, FoundationalBranchIdConstructionDenial> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(FoundationalBranchIdConstructionDenial::EmptyName);
        }

        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBranchForkBasis {
    forked_from_branch: FoundationalBranchId,
    fork_epoch: BoundaryEpoch,
}

impl FoundationalBranchForkBasis {
    pub fn new(forked_from_branch: FoundationalBranchId, fork_epoch: BoundaryEpoch) -> Self {
        Self {
            forked_from_branch,
            fork_epoch,
        }
    }

    pub fn forked_from_branch(&self) -> &FoundationalBranchId {
        &self.forked_from_branch
    }

    pub const fn fork_epoch(&self) -> BoundaryEpoch {
        self.fork_epoch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchObservationBasis {
    basis_id: EquivalenceBasisId,
    observed_epoch: BoundaryEpoch,
}

impl FoundationalBranchObservationBasis {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchForkObservationBasis {
    basis_id: EquivalenceBasisId,
    fork_epoch: BoundaryEpoch,
}

impl FoundationalBranchForkObservationBasis {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBranchComparisonBasis {
    basis_id: EquivalenceBasisId,
    compared_against_branch: FoundationalBranchId,
}

impl FoundationalBranchComparisonBasis {
    pub fn new(
        basis_id: EquivalenceBasisId,
        compared_against_branch: FoundationalBranchId,
    ) -> Self {
        Self {
            basis_id,
            compared_against_branch,
        }
    }

    pub const fn basis_id(&self) -> EquivalenceBasisId {
        self.basis_id
    }

    pub fn compared_against_branch(&self) -> &FoundationalBranchId {
        &self.compared_against_branch
    }
}
