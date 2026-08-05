use worth_relational::facade::history::{BranchId, CommitId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryPrimaryGraphCommittedApplication {
    application_outcome_identity:
        Option<super::super::application_attempt::WorthQueryApplicationCommitOutcomeIdentity>,
    runtime_instance_id: u64,
    branch: BranchId,
    commit_id: CommitId,
    changed_record_count: usize,
    emitted_effect_count: usize,
    mutation_work: Option<super::WorthQueryPrimaryMutationWorkEvidence>,
}

impl WorthQueryPrimaryGraphCommittedApplication {
    pub(in crate::domain_computation::primary_graph) fn new(
        application_outcome_identity: super::super::application_attempt::WorthQueryApplicationCommitOutcomeIdentity,
        runtime_instance_id: u64,
        branch: BranchId,
        commit_id: CommitId,
        changed_record_count: usize,
        emitted_effect_count: usize,
    ) -> Self {
        Self {
            application_outcome_identity: Some(application_outcome_identity),
            runtime_instance_id,
            branch,
            commit_id,
            changed_record_count,
            emitted_effect_count,
            mutation_work: None,
        }
    }

    pub(in crate::domain_computation::primary_graph) const fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub(in crate::domain_computation::primary_graph) const fn application_outcome_identity(
        &self,
    ) -> Option<super::super::application_attempt::WorthQueryApplicationCommitOutcomeIdentity> {
        self.application_outcome_identity
    }

    pub(in crate::domain_computation::primary_graph) const fn branch(&self) -> &BranchId {
        &self.branch
    }

    pub(in crate::domain_computation::primary_graph) const fn commit_id(&self) -> CommitId {
        self.commit_id
    }

    pub(in crate::domain_computation::primary_graph) const fn changed_record_count(&self) -> usize {
        self.changed_record_count
    }

    pub(in crate::domain_computation::primary_graph) const fn emitted_effect_count(&self) -> usize {
        self.emitted_effect_count
    }

    pub(in crate::domain_computation::primary_graph) const fn with_mutation_work(
        mut self,
        mutation_work: super::WorthQueryPrimaryMutationWorkEvidence,
    ) -> Self {
        self.mutation_work = Some(mutation_work);
        self
    }

    pub(in crate::domain_computation::primary_graph) const fn mutation_work(
        &self,
    ) -> Option<super::WorthQueryPrimaryMutationWorkEvidence> {
        self.mutation_work
    }
}
