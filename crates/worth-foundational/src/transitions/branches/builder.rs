use super::artifacts::FoundationalBranchCandidateArtifact;
use super::vocabulary::{
    FoundationalBranchCandidateId, FoundationalBranchComparisonBasis, FoundationalBranchForkBasis,
    FoundationalBranchForkObservationBasis, FoundationalBranchId,
    FoundationalBranchObservationBasis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBranchLocalConstructionDenial {
    MissingBranchId,
    MissingCandidateId,
    MissingForkBasis,
    MissingObservationBasis,
}

#[derive(Debug, Clone)]
pub struct FoundationalBranchCandidateBuilder {
    branch_id: Option<FoundationalBranchId>,
    candidate_id: Option<FoundationalBranchCandidateId>,
    fork_basis: Option<FoundationalBranchForkBasis>,
    observation_basis: Option<FoundationalBranchObservationBasis>,
    fork_observation_basis: Option<FoundationalBranchForkObservationBasis>,
    comparison_basis: Option<FoundationalBranchComparisonBasis>,
}

impl FoundationalBranchCandidateBuilder {
    pub const fn new() -> Self {
        Self {
            branch_id: None,
            candidate_id: None,
            fork_basis: None,
            observation_basis: None,
            fork_observation_basis: None,
            comparison_basis: None,
        }
    }

    pub fn on_branch(mut self, branch_id: FoundationalBranchId) -> Self {
        self.branch_id = Some(branch_id);
        self
    }

    pub fn with_candidate_id(mut self, candidate_id: FoundationalBranchCandidateId) -> Self {
        self.candidate_id = Some(candidate_id);
        self
    }

    pub fn from_fork_basis(mut self, fork_basis: FoundationalBranchForkBasis) -> Self {
        self.fork_basis = Some(fork_basis);
        self
    }

    pub fn under_observation_basis(
        mut self,
        observation_basis: FoundationalBranchObservationBasis,
    ) -> Self {
        self.observation_basis = Some(observation_basis);
        self
    }

    pub fn under_fork_observation_basis(
        mut self,
        fork_observation_basis: FoundationalBranchForkObservationBasis,
    ) -> Self {
        self.fork_observation_basis = Some(fork_observation_basis);
        self
    }

    pub fn against_comparison_basis(
        mut self,
        comparison_basis: FoundationalBranchComparisonBasis,
    ) -> Self {
        self.comparison_basis = Some(comparison_basis);
        self
    }

    pub fn stage<T>(
        self,
        payload: T,
    ) -> Result<FoundationalBranchCandidateArtifact<T>, FoundationalBranchLocalConstructionDenial>
    {
        Ok(FoundationalBranchCandidateArtifact::new(
            self.branch_id
                .ok_or(FoundationalBranchLocalConstructionDenial::MissingBranchId)?,
            self.candidate_id
                .ok_or(FoundationalBranchLocalConstructionDenial::MissingCandidateId)?,
            self.fork_basis
                .ok_or(FoundationalBranchLocalConstructionDenial::MissingForkBasis)?,
            self.observation_basis
                .ok_or(FoundationalBranchLocalConstructionDenial::MissingObservationBasis)?,
            self.fork_observation_basis,
            self.comparison_basis,
            payload,
        ))
    }
}

pub fn foundational_branch_candidate() -> FoundationalBranchCandidateBuilder {
    FoundationalBranchCandidateBuilder::new()
}
