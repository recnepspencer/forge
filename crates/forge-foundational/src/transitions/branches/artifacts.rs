use super::vocabulary::{
    candidate_state_definition, staged_state_definition, FoundationalBranchCandidateId,
    FoundationalBranchComparisonBasis, FoundationalBranchForkBasis,
    FoundationalBranchForkObservationBasis, FoundationalBranchId,
    FoundationalBranchLocalStateDefinition, FoundationalBranchLocalStateKind,
    FoundationalBranchObservationBasis,
};
use crate::transitions::receipts::{
    FoundationalBranchCloseoutCause, FoundationalBranchDiscardReceipt,
    FoundationalCommitReceiptIssuanceDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBranchCandidateArtifact<T> {
    branch_id: FoundationalBranchId,
    candidate_id: FoundationalBranchCandidateId,
    fork_basis: FoundationalBranchForkBasis,
    observation_basis: FoundationalBranchObservationBasis,
    fork_observation_basis: Option<FoundationalBranchForkObservationBasis>,
    comparison_basis: Option<FoundationalBranchComparisonBasis>,
    payload: T,
}

impl<T> FoundationalBranchCandidateArtifact<T> {
    pub(crate) fn new(
        branch_id: FoundationalBranchId,
        candidate_id: FoundationalBranchCandidateId,
        fork_basis: FoundationalBranchForkBasis,
        observation_basis: FoundationalBranchObservationBasis,
        fork_observation_basis: Option<FoundationalBranchForkObservationBasis>,
        comparison_basis: Option<FoundationalBranchComparisonBasis>,
        payload: T,
    ) -> Self {
        Self {
            branch_id,
            candidate_id,
            fork_basis,
            observation_basis,
            fork_observation_basis,
            comparison_basis,
            payload,
        }
    }

    pub const fn branch_local_state_kind(&self) -> FoundationalBranchLocalStateKind {
        FoundationalBranchLocalStateKind::Candidate
    }

    pub fn branch_local_state_definition(&self) -> &'static FoundationalBranchLocalStateDefinition {
        candidate_state_definition()
    }

    pub fn branch_id(&self) -> &FoundationalBranchId {
        &self.branch_id
    }

    pub const fn candidate_id(&self) -> FoundationalBranchCandidateId {
        self.candidate_id
    }

    pub fn fork_basis(&self) -> &FoundationalBranchForkBasis {
        &self.fork_basis
    }

    pub const fn observation_basis(&self) -> FoundationalBranchObservationBasis {
        self.observation_basis
    }

    pub const fn fork_observation_basis(&self) -> Option<FoundationalBranchForkObservationBasis> {
        self.fork_observation_basis
    }

    pub fn comparison_basis(&self) -> Option<&FoundationalBranchComparisonBasis> {
        self.comparison_basis.as_ref()
    }

    pub const fn payload(&self) -> &T {
        &self.payload
    }

    pub fn discard_with_zero_residue_proof(
        self,
    ) -> Result<FoundationalBranchDiscardReceipt, FoundationalCommitReceiptIssuanceDenial> {
        FoundationalBranchDiscardReceipt::new(
            self.branch_id,
            self.fork_basis,
            FoundationalBranchCloseoutCause::ExplicitDiscard,
        )
    }

    pub fn staged(self) -> FoundationalStagedBranchArtifact<T> {
        FoundationalStagedBranchArtifact::new(
            self.branch_id,
            self.candidate_id,
            self.fork_basis,
            self.observation_basis,
            self.fork_observation_basis,
            self.comparison_basis,
            self.payload,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalStagedBranchArtifact<T> {
    branch_id: FoundationalBranchId,
    candidate_id: FoundationalBranchCandidateId,
    fork_basis: FoundationalBranchForkBasis,
    observation_basis: FoundationalBranchObservationBasis,
    fork_observation_basis: Option<FoundationalBranchForkObservationBasis>,
    comparison_basis: Option<FoundationalBranchComparisonBasis>,
    payload: T,
}

impl<T> FoundationalStagedBranchArtifact<T> {
    pub(crate) fn new(
        branch_id: FoundationalBranchId,
        candidate_id: FoundationalBranchCandidateId,
        fork_basis: FoundationalBranchForkBasis,
        observation_basis: FoundationalBranchObservationBasis,
        fork_observation_basis: Option<FoundationalBranchForkObservationBasis>,
        comparison_basis: Option<FoundationalBranchComparisonBasis>,
        payload: T,
    ) -> Self {
        Self {
            branch_id,
            candidate_id,
            fork_basis,
            observation_basis,
            fork_observation_basis,
            comparison_basis,
            payload,
        }
    }

    pub const fn branch_local_state_kind(&self) -> FoundationalBranchLocalStateKind {
        FoundationalBranchLocalStateKind::Staged
    }

    pub fn branch_local_state_definition(&self) -> &'static FoundationalBranchLocalStateDefinition {
        staged_state_definition()
    }

    pub fn branch_id(&self) -> &FoundationalBranchId {
        &self.branch_id
    }

    pub const fn candidate_id(&self) -> FoundationalBranchCandidateId {
        self.candidate_id
    }

    pub fn fork_basis(&self) -> &FoundationalBranchForkBasis {
        &self.fork_basis
    }

    pub const fn observation_basis(&self) -> FoundationalBranchObservationBasis {
        self.observation_basis
    }

    pub const fn fork_observation_basis(&self) -> Option<FoundationalBranchForkObservationBasis> {
        self.fork_observation_basis
    }

    pub fn comparison_basis(&self) -> Option<&FoundationalBranchComparisonBasis> {
        self.comparison_basis.as_ref()
    }

    pub const fn payload(&self) -> &T {
        &self.payload
    }
}
