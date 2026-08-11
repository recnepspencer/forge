//! Immutable exact-commit evidence retained for equivalent receipt resolution.

use std::collections::BTreeMap;
use worth_relational::facade::history::{CommitId, CommitReference};

use super::super::WorthQueryPrimaryGraphCommittedApplication;

/// Immutable owner evidence indexed by the Relational commit that produced it.
#[derive(Default)]
pub(in crate::domain_computation::primary_graph) struct WorthQueryCompletedCommitEvidenceStore {
    by_commit: BTreeMap<CommitId, WorthQueryPrimaryGraphCommittedApplication>,
}

impl WorthQueryCompletedCommitEvidenceStore {
    pub(super) fn record(&mut self, evidence: WorthQueryPrimaryGraphCommittedApplication) {
        let commit = evidence.commit_reference().commit_id;
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
}
