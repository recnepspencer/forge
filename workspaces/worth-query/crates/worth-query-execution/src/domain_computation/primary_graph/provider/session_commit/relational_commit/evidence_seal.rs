//! Exact-commit evidence minted only from the committed session stage.

use super::commit_execution::WorthQueryCommittedApplicationSession;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryPrimaryGraphCommitEvidence {
    provider_session_binding:
        crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding,
    commit: worth_relational::facade::history::CommitReference,
    mutation_work: super::super::super::mutation_work::WorthQueryPrimaryMutationWorkEvidence,
    retained_preimage:
        Option<crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage>,
    committed_dispatch_outbox:
        super::super::outbox_commit::WorthQueryCommittedDispatchOutboxResolution,
}

pub(in crate::domain_computation::primary_graph) struct WorthQueryMutationWorkCommitSeal {
    counters: super::super::super::mutation_work::WorthQueryPrimaryMutationWorkCounters,
    changed_records: Vec<worth_relational::facade::transactions::RecordRef>,
    preimage: super::super::preimage_retention::WorthQueryPreImageRetentionWork,
}

pub(super) fn seal(
    committed: &WorthQueryCommittedApplicationSession,
) -> WorthQueryPrimaryGraphCommitEvidence {
    let mutation_work =
        super::super::super::mutation_work::WorthQueryPrimaryMutationWorkEvidence::from_commit_seal(
            WorthQueryMutationWorkCommitSeal {
                counters: committed.work(),
                changed_records: committed.committed().changed_records.clone(),
                preimage: committed.preimage_retention_work(),
            },
        );
    let committed_dispatch_outbox =
        super::super::outbox_commit::WorthQueryCommittedDispatchOutboxResolution::from_commit(
            committed.attempt().dispatch_outbox.as_ref(),
            committed.committed(),
        );
    WorthQueryPrimaryGraphCommitEvidence {
        provider_session_binding: committed.attempt().provider_session_binding.clone(),
        commit: committed.committed().envelope().commit.clone(),
        mutation_work,
        retained_preimage: committed.retained_preimage().cloned(),
        committed_dispatch_outbox,
    }
}

impl WorthQueryMutationWorkCommitSeal {
    pub(in crate::domain_computation::primary_graph) fn into_parts(
        self,
    ) -> (
        super::super::super::mutation_work::WorthQueryPrimaryMutationWorkCounters,
        Vec<worth_relational::facade::transactions::RecordRef>,
        super::super::preimage_retention::WorthQueryPreImageRetentionWork,
    ) {
        (self.counters, self.changed_records, self.preimage)
    }
}

impl WorthQueryPrimaryGraphCommitEvidence {
    pub(in crate::domain_computation::primary_graph) const fn provider_session_binding(
        &self,
    ) -> &crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding
    {
        &self.provider_session_binding
    }

    pub(in crate::domain_computation::primary_graph) const fn commit_reference(
        &self,
    ) -> &worth_relational::facade::history::CommitReference {
        &self.commit
    }

    pub(in crate::domain_computation::primary_graph) const fn mutation_work(
        &self,
    ) -> &super::super::super::mutation_work::WorthQueryPrimaryMutationWorkEvidence {
        &self.mutation_work
    }

    pub(in crate::domain_computation::primary_graph) const fn retained_preimage(
        &self,
    ) -> Option<&crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage> {
        self.retained_preimage.as_ref()
    }

    pub(in crate::domain_computation::primary_graph) const fn committed_dispatch_outbox(
        &self,
    ) -> &super::super::outbox_commit::WorthQueryCommittedDispatchOutboxResolution {
        &self.committed_dispatch_outbox
    }
}
