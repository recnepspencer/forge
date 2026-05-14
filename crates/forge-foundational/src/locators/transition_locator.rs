use crate::transitions::{
    FoundationalBranchCandidateId, FoundationalBranchId, FoundationalCommitId,
    FoundationalCommitParentBasis, FoundationalCommittedDeltaLocus, FoundationalMergeConflictLocus,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalBranchCandidateLocator {
    branch_id: FoundationalBranchId,
    candidate_id: FoundationalBranchCandidateId,
}

impl FoundationalBranchCandidateLocator {
    pub fn new(
        branch_id: FoundationalBranchId,
        candidate_id: FoundationalBranchCandidateId,
    ) -> Self {
        Self {
            branch_id,
            candidate_id,
        }
    }

    pub fn branch_id(&self) -> &FoundationalBranchId {
        &self.branch_id
    }

    pub const fn candidate_id(&self) -> FoundationalBranchCandidateId {
        self.candidate_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMergeConflictLocator {
    source_branch: FoundationalBranchId,
    target_branch: FoundationalBranchId,
    conflict_locus: FoundationalMergeConflictLocus,
}

impl FoundationalMergeConflictLocator {
    pub fn new(
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
        conflict_locus: FoundationalMergeConflictLocus,
    ) -> Self {
        Self {
            source_branch,
            target_branch,
            conflict_locus,
        }
    }

    pub fn source_branch(&self) -> &FoundationalBranchId {
        &self.source_branch
    }

    pub fn target_branch(&self) -> &FoundationalBranchId {
        &self.target_branch
    }

    pub fn conflict_locus(&self) -> &FoundationalMergeConflictLocus {
        &self.conflict_locus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalCommitParentageLocator {
    commit_id: FoundationalCommitId,
    parent_basis: FoundationalCommitParentBasis,
}

impl FoundationalCommitParentageLocator {
    pub const fn new(
        commit_id: FoundationalCommitId,
        parent_basis: FoundationalCommitParentBasis,
    ) -> Self {
        Self {
            commit_id,
            parent_basis,
        }
    }

    pub const fn commit_id(&self) -> FoundationalCommitId {
        self.commit_id
    }

    pub const fn parent_basis(&self) -> FoundationalCommitParentBasis {
        self.parent_basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCommittedDeltaLocator {
    commit_id: FoundationalCommitId,
    delta_locus: FoundationalCommittedDeltaLocus,
}

impl FoundationalCommittedDeltaLocator {
    pub fn new(
        commit_id: FoundationalCommitId,
        delta_locus: FoundationalCommittedDeltaLocus,
    ) -> Self {
        Self {
            commit_id,
            delta_locus,
        }
    }

    pub const fn commit_id(&self) -> FoundationalCommitId {
        self.commit_id
    }

    pub fn delta_locus(&self) -> &FoundationalCommittedDeltaLocus {
        &self.delta_locus
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalTransitionLocator {
    BranchCandidate(FoundationalBranchCandidateLocator),
    MergeConflict(FoundationalMergeConflictLocator),
    CommitParentage(FoundationalCommitParentageLocator),
    CommittedDelta(FoundationalCommittedDeltaLocator),
}
