use worth_relational::facade::history::{BranchId, CommitReference};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryPrimaryGraphCommittedApplication {
    application_outcome_identity:
        Option<super::super::application_attempt::WorthQueryApplicationCommitOutcomeIdentity>,
    runtime_instance_id: u64,
    changed_record_count: usize,
    emitted_effect_count: usize,
    commit_evidence: super::session_commit::WorthQueryPrimaryGraphCommitEvidence,
}

impl WorthQueryPrimaryGraphCommittedApplication {
    pub(in crate::domain_computation::primary_graph::provider) fn from_publication(
        seal: super::session_commit::WorthQueryCommittedApplicationPublicationSeal,
    ) -> Self {
        let (
            application_outcome_identity,
            runtime_instance_id,
            changed_record_count,
            emitted_effect_count,
            commit_evidence,
        ) = seal.into_parts();
        Self {
            application_outcome_identity: Some(application_outcome_identity),
            runtime_instance_id,
            changed_record_count,
            emitted_effect_count,
            commit_evidence,
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
        &self.commit_evidence.commit_reference().branch_id
    }

    pub(in crate::domain_computation::primary_graph) const fn commit_reference(
        &self,
    ) -> &CommitReference {
        self.commit_evidence.commit_reference()
    }

    pub(in crate::domain_computation::primary_graph) const fn changed_record_count(&self) -> usize {
        self.changed_record_count
    }

    pub(in crate::domain_computation::primary_graph) const fn emitted_effect_count(&self) -> usize {
        self.emitted_effect_count
    }

    pub(in crate::domain_computation::primary_graph) fn mutation_work(
        &self,
    ) -> Option<&super::WorthQueryPrimaryMutationWorkEvidence> {
        Some(self.commit_evidence.mutation_work())
    }

    pub(in crate::domain_computation::primary_graph) fn commit_evidence(
        &self,
    ) -> &super::session_commit::WorthQueryPrimaryGraphCommitEvidence {
        &self.commit_evidence
    }
}
