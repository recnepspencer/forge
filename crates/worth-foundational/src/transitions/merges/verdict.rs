use super::builder::FoundationalMergeCandidate;
use super::scope_evidence::FoundationalAdmittedMergeScopeEvidence;
use super::scoped::FoundationalMergeScope;
use super::strategy::{
    FoundationalMergeBaseSelectionBasis, FoundationalMergeBasis, FoundationalStrategyBasis,
    FoundationalTransitionCorrespondenceBasis, FoundationalTransitionRemapBasis,
    FoundationalTransitionStrategyContractBasis, FoundationalTransitionStrategyDescriptorDigest,
    FoundationalTransitionStrategyIdentity,
};
use super::vocabulary::{
    FoundationalMergeConflictLocus, FoundationalMergeStructuralSummary,
    FoundationalMergeVerdictKind,
};
use crate::transitions::FoundationalBranchId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMergeVerdict<T> {
    candidate: FoundationalMergeCandidate<T>,
    kind: FoundationalMergeVerdictKind,
    scope_evidence: FoundationalAdmittedMergeScopeEvidence,
    conflict_loci: Vec<FoundationalMergeConflictLocus>,
    superseded_by_branch: Option<FoundationalBranchId>,
}

impl<T> FoundationalMergeVerdict<T> {
    pub(crate) fn new(
        candidate: FoundationalMergeCandidate<T>,
        kind: FoundationalMergeVerdictKind,
        scope_evidence: FoundationalAdmittedMergeScopeEvidence,
        conflict_loci: Vec<FoundationalMergeConflictLocus>,
        superseded_by_branch: Option<FoundationalBranchId>,
    ) -> Self {
        Self {
            candidate,
            kind,
            scope_evidence,
            conflict_loci,
            superseded_by_branch,
        }
    }

    pub const fn kind(&self) -> FoundationalMergeVerdictKind {
        self.kind
    }

    pub fn source_branch(&self) -> &FoundationalBranchId {
        self.candidate.source_branch()
    }

    pub fn target_branch(&self) -> &FoundationalBranchId {
        self.candidate.target_branch()
    }

    pub fn fork_basis(&self) -> &crate::transitions::FoundationalBranchCandidateForkBasis {
        self.candidate.staged_branch().fork_basis()
    }

    pub fn observation_basis(
        &self,
    ) -> crate::transitions::FoundationalBranchCandidateObservationBasis {
        self.candidate.staged_branch().observation_basis()
    }

    pub fn comparison_basis(
        &self,
    ) -> Option<&crate::transitions::FoundationalBranchCandidateComparisonBasis> {
        self.candidate.staged_branch().comparison_basis()
    }

    pub fn merge_basis(&self) -> &FoundationalMergeBasis {
        self.candidate.merge_basis()
    }

    pub const fn structural_summary(&self) -> FoundationalMergeStructuralSummary {
        self.candidate.structural_summary()
    }

    pub fn scope(&self) -> &FoundationalMergeScope {
        self.candidate.scope()
    }

    pub fn scope_evidence(&self) -> &FoundationalAdmittedMergeScopeEvidence {
        &self.scope_evidence
    }

    pub const fn merge_base_selection_basis(&self) -> FoundationalMergeBaseSelectionBasis {
        self.candidate.merge_base_selection_basis()
    }

    pub fn strategy_identity(&self) -> &FoundationalTransitionStrategyIdentity {
        self.candidate.strategy_identity()
    }

    pub const fn strategy_descriptor_digest(
        &self,
    ) -> FoundationalTransitionStrategyDescriptorDigest {
        self.candidate.strategy_descriptor_digest()
    }

    pub const fn strategy_contract_basis(&self) -> FoundationalTransitionStrategyContractBasis {
        self.candidate.strategy_contract_basis()
    }

    pub const fn strategy_basis(&self) -> FoundationalStrategyBasis {
        self.candidate.strategy_basis()
    }

    pub const fn correspondence_basis(&self) -> Option<FoundationalTransitionCorrespondenceBasis> {
        self.candidate.correspondence_basis()
    }

    pub const fn remap_basis(&self) -> Option<FoundationalTransitionRemapBasis> {
        self.candidate.remap_basis()
    }

    pub fn conflict_loci(&self) -> &[FoundationalMergeConflictLocus] {
        &self.conflict_loci
    }

    pub fn superseded_by_branch(&self) -> Option<&FoundationalBranchId> {
        self.superseded_by_branch.as_ref()
    }

    pub fn payload(&self) -> &T {
        self.candidate.payload()
    }
}
