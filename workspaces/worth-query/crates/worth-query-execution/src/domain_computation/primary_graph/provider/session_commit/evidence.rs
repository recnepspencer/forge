//! Immutable exact-commit evidence retained for equivalent receipt resolution.

use std::collections::BTreeMap;
use worth_relational::facade::history::{CommitId, CommitReference};

use super::super::WorthQueryPrimaryGraphCommittedApplication;

/// Immutable owner evidence indexed by the Relational commit that produced it.
#[derive(Default)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryCompletedCommitEvidenceStore {
    by_commit: BTreeMap<CommitId, WorthQueryPrimaryGraphCommittedApplication>,
    by_session:
        BTreeMap<crate::domain_computation::WorthQueryProviderSessionAffinityIdentity, CommitId>,
}

impl WorthQueryCompletedCommitEvidenceStore {
    pub(in crate::domain_computation::primary_graph::provider) fn record(
        &mut self,
        evidence: WorthQueryPrimaryGraphCommittedApplication,
    ) {
        let commit = evidence.commit_reference().commit_id;
        let affinity = evidence
            .commit_evidence()
            .provider_session_binding()
            .affinity_identity();
        assert!(
            self.by_session.insert(affinity, commit).is_none(),
            "one provider session may record completed evidence only once"
        );
        assert!(
            self.by_commit.insert(commit, evidence).is_none(),
            "one Relational commit may record provider evidence only once"
        );
    }

    pub(in crate::domain_computation::primary_graph) fn observe(
        &self,
        commit: &CommitReference,
    ) -> Option<WorthQueryPrimaryGraphCommittedApplication> {
        self.by_commit
            .get(&commit.commit_id)
            .filter(|evidence| evidence.commit_reference() == commit)
            .cloned()
    }

    pub(in crate::domain_computation::primary_graph) fn observe_session(
        &self,
        session: &crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding,
    ) -> Option<WorthQueryPrimaryGraphCommittedApplication> {
        let commit = self.by_session.get(&session.affinity_identity())?;
        self.by_commit
            .get(commit)
            .filter(|evidence| {
                evidence
                    .commit_evidence()
                    .provider_session_binding()
                    .same_session(session)
            })
            .cloned()
    }
}
