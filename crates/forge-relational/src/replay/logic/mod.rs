use std::collections::BTreeSet;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitId};
use crate::replay::data::{
    CanonicalCommitEnvelope, RelationalReplayOutcome, RelationalReplayRequest, ReplayExecutionMode,
    ReplayFailureClass, ReplayMismatch, ReplayObservableSurface,
};
use crate::transactions::data::TransactionOptions;
use serde_json::json;

use crate::logic::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub fn canonical_commit_envelope(
        &self,
        commit_id: CommitId,
    ) -> Option<&CanonicalCommitEnvelope> {
        self.commit_envelopes.get(&commit_id)
    }

    pub fn replay_commit(&mut self, request: RelationalReplayRequest) -> RelationalReplayOutcome {
        let Some(envelope) = self.commit_envelopes.get(&request.commit_id).cloned() else {
            return RelationalReplayOutcome {
                requested: request,
                commit: None,
                reconstructed_parent_chain: Vec::new(),
                snapshot_version: None,
                compared_surfaces: Vec::new(),
                mismatches: Vec::new(),
                failure: Some(ReplayFailureClass::MissingCommit),
            };
        };
        if envelope.branch_context != request.branch_id {
            return RelationalReplayOutcome {
                requested: request,
                commit: Some(envelope.commit.clone()),
                reconstructed_parent_chain: envelope.commit.parents.clone(),
                snapshot_version: Some(envelope.commit.version_id),
                compared_surfaces: Vec::new(),
                mismatches: Vec::new(),
                failure: Some(ReplayFailureClass::BranchMismatch),
            };
        }
        if envelope.schema_registry != self.config.schema_registry {
            return RelationalReplayOutcome {
                requested: request,
                commit: Some(envelope.commit.clone()),
                reconstructed_parent_chain: envelope.commit.parents.clone(),
                snapshot_version: Some(envelope.commit.version_id),
                compared_surfaces: Vec::new(),
                mismatches: Vec::new(),
                failure: Some(ReplayFailureClass::SchemaMismatch),
            };
        }

        let chain = match self.replay_chain(request.commit_id) {
            Ok(chain) => chain,
            Err(failure) => {
                return RelationalReplayOutcome {
                    requested: request,
                    commit: Some(envelope.commit.clone()),
                    reconstructed_parent_chain: envelope.commit.parents.clone(),
                    snapshot_version: Some(envelope.commit.version_id),
                    compared_surfaces: Vec::new(),
                    mismatches: Vec::new(),
                    failure: Some(failure),
                };
            }
        };

        let mut replay_runtime = RelationalRuntime::new(self.config.clone());
        for commit_id in &chain {
            let staged_envelope = self
                .commit_envelopes
                .get(commit_id)
                .expect("replay chain envelope must exist");
            if !replay_runtime
                .branch_heads
                .contains_key(&staged_envelope.branch_context)
            {
                let parent_branch = staged_envelope
                    .commit
                    .parents
                    .first()
                    .and_then(|parent| self.commit_envelopes.get(parent))
                    .map(|parent| parent.branch_context.clone())
                    .unwrap_or_else(|| replay_runtime.config.main_branch.clone());
                let _ = replay_runtime
                    .create_branch(staged_envelope.branch_context.clone(), &parent_branch);
            }
            let mut txn = replay_runtime.begin_transaction(TransactionOptions {
                target_branch: Some(staged_envelope.branch_context.clone()),
                merge_parent_branches: staged_envelope.merge_parent_branches.clone(),
                ..TransactionOptions::default()
            });
            txn.push_batch(crate::transactions::data::WorkerIntentBatch {
                name: format!("replay-commit-{}", staged_envelope.commit.commit_id.0),
                partition_key: None,
                worker_local_only: true,
                intents: staged_envelope.merged_plan.merged_intents.clone(),
            });
            if txn.commit().is_err() {
                return RelationalReplayOutcome {
                    requested: request,
                    commit: Some(envelope.commit.clone()),
                    reconstructed_parent_chain: chain.clone(),
                    snapshot_version: Some(envelope.commit.version_id),
                    compared_surfaces: Vec::new(),
                    mismatches: Vec::new(),
                    failure: Some(ReplayFailureClass::ObservableMismatch),
                };
            }
        }

        let replayed_envelope = replay_runtime
            .canonical_commit_envelope(request.commit_id)
            .cloned()
            .expect("replayed target envelope");
        let compared_surfaces = self.promised_replay_surfaces(&envelope);
        let mut mismatches = Vec::new();

        if compared_surfaces.contains(&ReplayObservableSurface::Patch)
            && replayed_envelope.patch.canonicalized() != envelope.patch.canonicalized()
        {
            mismatches.push(ReplayMismatch {
                surface: ReplayObservableSurface::Patch,
                detail: "canonical patch artifact differed".to_string(),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::Diagnostics)
            && replayed_envelope.diagnostics_summary != envelope.diagnostics_summary
        {
            mismatches.push(ReplayMismatch {
                surface: ReplayObservableSurface::Diagnostics,
                detail: "diagnostics summary differed".to_string(),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::History)
            && (replayed_envelope.commit.parents != envelope.commit.parents
                || replayed_envelope.merge_parent_branches != envelope.merge_parent_branches
                || replayed_envelope.merge_base_commits != envelope.merge_base_commits)
        {
            mismatches.push(ReplayMismatch {
                surface: ReplayObservableSurface::History,
                detail: "history parent ordering differed".to_string(),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::Snapshot) {
            let original_read = self.read_version(envelope.commit.version_id);
            let replayed_read = replay_runtime.read_version(replayed_envelope.commit.version_id);
            if original_read != replayed_read {
                mismatches.push(ReplayMismatch {
                    surface: ReplayObservableSurface::Snapshot,
                    detail: "snapshot-visible state differed".to_string(),
                });
            }
        }
        if compared_surfaces.contains(&ReplayObservableSurface::BranchHead)
            && replay_runtime.branch_head(&request.branch_id) != Some(&envelope.commit)
        {
            mismatches.push(ReplayMismatch {
                surface: ReplayObservableSurface::BranchHead,
                detail: "branch head movement differed".to_string(),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::Lineage)
            && replay_runtime.lineage_events != self.lineage_events
        {
            mismatches.push(ReplayMismatch {
                surface: ReplayObservableSurface::Lineage,
                detail: "lineage events differed".to_string(),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::DerivedIndexes)
            && replay_runtime.index_generations_for_version(envelope.commit.version_id)
                != self.index_generations_for_version(envelope.commit.version_id)
        {
            mismatches.push(ReplayMismatch {
                surface: ReplayObservableSurface::DerivedIndexes,
                detail: "derived index generations differed".to_string(),
            });
        }

        let outcome = RelationalReplayOutcome {
            requested: request,
            commit: Some(envelope.commit.clone()),
            reconstructed_parent_chain: chain.clone(),
            snapshot_version: Some(envelope.commit.version_id),
            compared_surfaces: compared_surfaces.clone(),
            mismatches: mismatches.clone(),
            failure: (!mismatches.is_empty()).then_some(ReplayFailureClass::ObservableMismatch),
        };
        self.record_replay_diagnostic(&outcome.requested, &outcome);
        outcome
    }

    pub fn replay_range(
        &mut self,
        branch_id: BranchId,
        commits: &[CommitId],
    ) -> Vec<RelationalReplayOutcome> {
        commits
            .iter()
            .copied()
            .map(|commit_id| {
                self.replay_commit(RelationalReplayRequest {
                    commit_id,
                    branch_id: branch_id.clone(),
                    execution_mode: ReplayExecutionMode::SerialDeterministic,
                })
            })
            .collect()
    }

    pub fn compare_replay_outcome(&self, outcome: &RelationalReplayOutcome) -> bool {
        outcome.failure.is_none() && outcome.mismatches.is_empty()
    }

    fn promised_replay_surfaces(
        &self,
        envelope: &CanonicalCommitEnvelope,
    ) -> Vec<ReplayObservableSurface> {
        let mut surfaces = vec![
            ReplayObservableSurface::Snapshot,
            ReplayObservableSurface::Patch,
            ReplayObservableSurface::Diagnostics,
            ReplayObservableSurface::History,
            ReplayObservableSurface::BranchHead,
        ];
        if !envelope.lineage_event_ids.is_empty() {
            surfaces.push(ReplayObservableSurface::Lineage);
        }
        if !envelope.index_generation_ids.is_empty() {
            surfaces.push(ReplayObservableSurface::DerivedIndexes);
        }
        surfaces
    }

    fn replay_chain(&self, commit_id: CommitId) -> Result<Vec<CommitId>, ReplayFailureClass> {
        let mut ordered = Vec::new();
        let mut visiting = BTreeSet::new();
        self.visit_replay_chain(commit_id, &mut visiting, &mut ordered)?;
        Ok(ordered)
    }

    fn visit_replay_chain(
        &self,
        commit_id: CommitId,
        visiting: &mut BTreeSet<CommitId>,
        ordered: &mut Vec<CommitId>,
    ) -> Result<(), ReplayFailureClass> {
        if ordered.contains(&commit_id) {
            return Ok(());
        }
        let Some(envelope) = self.commit_envelopes.get(&commit_id) else {
            return Err(ReplayFailureClass::MissingParentChain);
        };
        if !visiting.insert(commit_id) {
            return Err(ReplayFailureClass::MissingParentChain);
        }
        for parent in &envelope.commit.parents {
            self.visit_replay_chain(*parent, visiting, ordered)?;
        }
        visiting.remove(&commit_id);
        ordered.push(commit_id);
        Ok(())
    }

    pub(super) fn record_replay_diagnostic(
        &mut self,
        request: &RelationalReplayRequest,
        outcome: &RelationalReplayOutcome,
    ) {
        let code = match outcome.failure.as_ref() {
            Some(
                ReplayFailureClass::SchemaMismatch | ReplayFailureClass::UnsupportedReplaySchema,
            ) => DiagnosticCode::ReplaySchemaVersionMismatch,
            Some(_) => DiagnosticCode::InvariantViolation,
            None => DiagnosticCode::CommitPublished,
        };
        self.push_bounded_diagnostic(
            DiagnosticsScope::Replay,
            if outcome.failure.is_some() {
                DiagnosticsArtifactKind::Failure
            } else {
                DiagnosticsArtifactKind::Comparison
            },
            vec![RelationalDiagnosticsEntry {
                code,
                message: "replay comparison completed".to_string(),
                fields: json!({
                    "commit_id": request.commit_id.0,
                    "branch_id": request.branch_id.0,
                    "mismatch_count": outcome.mismatches.len(),
                    "failure": outcome.failure.as_ref().map(|value| format!("{value:?}")),
                }),
            }],
        );
    }
}
