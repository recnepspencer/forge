use std::collections::BTreeSet;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsScope,
};
use crate::durability::data::{DurableCommitEnvelope, RecoveryPlan};
use crate::history::data::{BranchId, CommitId};
use crate::replay::data::{
    CanonicalCommitEnvelope, RelationalReplayOutcome, RelationalReplayRequest, ReplayExecutionMode,
    ReplayFailureClass, ReplayMismatch, ReplayMismatchClass, ReplayObservableSurface,
};
use serde_json::json;

use crate::logic::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub fn canonical_commit_envelope(
        &self,
        commit_id: CommitId,
    ) -> Option<&CanonicalCommitEnvelope> {
        self.history.commit_envelopes.get(&commit_id)
    }

    pub fn replay_commit(&mut self, request: RelationalReplayRequest) -> RelationalReplayOutcome {
        let Some(envelope) = self
            .history
            .commit_envelopes
            .get(&request.commit_id)
            .cloned()
        else {
            return RelationalReplayOutcome::fail(
                request,
                None,
                None,
                ReplayFailureClass::MissingCommit,
            );
        };
        if envelope.branch_context != request.branch_id {
            return RelationalReplayOutcome::fail(
                request,
                Some(&envelope),
                None,
                ReplayFailureClass::BranchMismatch,
            );
        }
        if envelope.schema_registry != self.config.schema_registry {
            return RelationalReplayOutcome::fail(
                request,
                Some(&envelope),
                None,
                ReplayFailureClass::SchemaMismatch,
            );
        }

        let chain = match self.replay_chain(request.commit_id) {
            Ok(chain) => chain,
            Err(failure) => {
                return RelationalReplayOutcome::fail(request, Some(&envelope), None, failure);
            }
        };

        let replay_plan = self.replay_recovery_plan_for_chain(&chain);
        let replay_runtime = match RelationalRuntime::rebuild_runtime_from_plan(replay_plan) {
            Ok(runtime) => runtime,
            Err(_) => {
                return RelationalReplayOutcome::fail(
                    request,
                    Some(&envelope),
                    Some(&chain),
                    ReplayFailureClass::ObservableMismatch,
                );
            }
        };

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
                class: ReplayMismatchClass::PatchDrift,
                surface: ReplayObservableSurface::Patch,
                detail: "canonical patch artifact differed".to_string(),
                expected: Some(format!("{:?}", envelope.patch)),
                observed: Some(format!("{:?}", replayed_envelope.patch)),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::Diagnostics)
            && replayed_envelope.diagnostics_summary != envelope.diagnostics_summary
        {
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::DiagnosticsDrift,
                surface: ReplayObservableSurface::Diagnostics,
                detail: "diagnostics summary differed".to_string(),
                expected: Some(format!("{:?}", envelope.diagnostics_summary)),
                observed: Some(format!("{:?}", replayed_envelope.diagnostics_summary)),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::History)
            && (replayed_envelope.commit.parents != envelope.commit.parents
                || replayed_envelope.merge_parent_branches != envelope.merge_parent_branches
                || replayed_envelope.merge_base_commits != envelope.merge_base_commits)
        {
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::HistoryDrift,
                surface: ReplayObservableSurface::History,
                detail: "history parent ordering differed".to_string(),
                expected: Some(format!(
                    "{:?}|{:?}|{:?}",
                    envelope.commit.parents,
                    envelope.merge_parent_branches,
                    envelope.merge_base_commits
                )),
                observed: Some(format!(
                    "{:?}|{:?}|{:?}",
                    replayed_envelope.commit.parents,
                    replayed_envelope.merge_parent_branches,
                    replayed_envelope.merge_base_commits
                )),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::Snapshot) {
            let original_read = self.read_version(envelope.commit.version_id);
            let replayed_read = replay_runtime.read_version(replayed_envelope.commit.version_id);
            if original_read != replayed_read {
                mismatches.push(ReplayMismatch {
                    class: ReplayMismatchClass::SnapshotDrift,
                    surface: ReplayObservableSurface::Snapshot,
                    detail: "snapshot-visible state differed".to_string(),
                    expected: Some(format!("{:?}", original_read)),
                    observed: Some(format!("{:?}", replayed_read)),
                });
            }
        }
        if compared_surfaces.contains(&ReplayObservableSurface::BranchHead)
            && replay_runtime.branch_head(&request.branch_id) != Some(&envelope.commit)
        {
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::BranchHeadDrift,
                surface: ReplayObservableSurface::BranchHead,
                detail: "branch head movement differed".to_string(),
                expected: Some(format!("{:?}", Some(&envelope.commit))),
                observed: Some(format!(
                    "{:?}",
                    replay_runtime.branch_head(&request.branch_id)
                )),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::Lineage)
            && replay_runtime.lineage.events != self.lineage.events
        {
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::LineageDrift,
                surface: ReplayObservableSurface::Lineage,
                detail: "lineage events differed".to_string(),
                expected: Some(format!("{:?}", self.lineage.events)),
                observed: Some(format!("{:?}", replay_runtime.lineage.events)),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::DerivedIndexes)
            && replay_runtime.index_generations_for_version(envelope.commit.version_id)
                != self.index_generations_for_version(envelope.commit.version_id)
        {
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::DerivedIndexDrift,
                surface: ReplayObservableSurface::DerivedIndexes,
                detail: "derived index generations differed".to_string(),
                expected: Some(format!(
                    "{:?}",
                    self.index_generations_for_version(envelope.commit.version_id)
                )),
                observed: Some(format!(
                    "{:?}",
                    replay_runtime.index_generations_for_version(envelope.commit.version_id)
                )),
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

    fn replay_recovery_plan_for_chain(&self, chain: &[CommitId]) -> RecoveryPlan {
        let checkpoint = self
            .durability
            .checkpoints
            .iter()
            .rev()
            .find(|checkpoint| {
                checkpoint
                    .coverage
                    .up_to_commit
                    .as_ref()
                    .map(|commit| chain.contains(&commit.commit_id))
                    .unwrap_or(false)
            })
            .cloned();
        let tail_start = checkpoint
            .as_ref()
            .and_then(|checkpoint| checkpoint.coverage.up_to_commit.as_ref())
            .map(|commit| commit.commit_id);
        let tail_log = chain
            .iter()
            .copied()
            .filter(|commit_id| tail_start.is_none_or(|start| *commit_id > start))
            .filter_map(|commit_id| {
                self.history
                    .commit_envelopes
                    .get(&commit_id)
                    .cloned()
                    .map(|envelope| DurableCommitEnvelope { envelope })
            })
            .collect();
        RecoveryPlan {
            config: self.config.clone(),
            store: self.durability.store.clone(),
            checkpoint_manifest: checkpoint.as_ref().and_then(|_| None),
            checkpoint,
            tail_log,
            cursor: crate::durability::data::RecoveryCursor {
                checkpoint_id: None,
                segment_ids: Vec::new(),
            },
            integrity_report: crate::durability::data::RecoveryIntegrityReport {
                selected_checkpoint_id: None,
                skipped_corrupt_checkpoints: Vec::new(),
                verified_segment_ids: Vec::new(),
                corrupt_segment_id: None,
            },
            compatibility: crate::durability::data::RecoveryCompatibilityCheck {
                schema_match: true,
                profile_match: true,
                runtime_name_match: true,
            },
        }
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
        let Some(envelope) = self.history.commit_envelopes.get(&commit_id) else {
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
        let builder = self.diagnostic(DiagnosticsScope::Replay);
        let builder = if outcome.failure.is_some() {
            builder.failure()
        } else {
            builder.comparison()
        };
        builder.emit_entry(
            code,
            "replay comparison completed",
            json!({
                "commit_id": request.commit_id.0,
                "branch_id": request.branch_id.0,
                "mismatch_count": outcome.mismatches.len(),
                "failure": outcome.failure.as_ref().map(|value| format!("{value:?}")),
            }),
        );
    }
}
