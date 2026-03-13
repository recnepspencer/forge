use crate::capabilities::{
    HistorySource, LineageRead, ReplayRead, RuntimeConfigSource, SchemaSource,
};
use crate::history::data::{BranchId, CommitId};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{
    RelationalReplayOutcome, RelationalReplayRequest, ReplayExecutionMode, ReplayFailureClass,
    ReplayMismatch, ReplayMismatchClass, ReplayObservableSurface,
};

use super::diagnostics::record_replay_diagnostic;
use super::planning::{
    load_replay_envelope, promised_replay_surfaces, replay_chain, replay_recovery_plan_for_chain,
};

pub struct ReplayAuthority<'runtime> {
    runtime: &'runtime mut RelationalRuntime,
}

impl<'runtime> ReplayAuthority<'runtime> {
    pub(crate) fn new(runtime: &'runtime mut RelationalRuntime) -> Self {
        Self { runtime }
    }

    pub fn replay_commit(&mut self, request: RelationalReplayRequest) -> RelationalReplayOutcome {
        let Some(envelope) = load_replay_envelope(self.runtime, request.commit_id) else {
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
        if envelope.schema_registry != *self.runtime.schema_registry() {
            return RelationalReplayOutcome::fail(
                request,
                Some(&envelope),
                None,
                ReplayFailureClass::SchemaMismatch,
            );
        }

        let chain = match replay_chain(self.runtime, request.commit_id) {
            Ok(chain) => chain,
            Err(failure) => {
                return RelationalReplayOutcome::fail(request, Some(&envelope), None, failure);
            }
        };

        let replay_plan =
            replay_recovery_plan_for_chain(self.runtime, self.runtime.runtime_config(), &chain);
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
            .replay_access()
            .canonical_commit_envelope(request.commit_id)
            .cloned()
            .expect("replayed target envelope");
        let compared_surfaces = promised_replay_surfaces(&envelope);
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
            let original_surface = self
                .runtime
                .replay_snapshot_surface_at_version(envelope.commit.version_id);
            let replayed_surface = replay_runtime
                .replay_snapshot_surface_at_version(replayed_envelope.commit.version_id);
            if original_surface != replayed_surface {
                mismatches.push(ReplayMismatch {
                    class: ReplayMismatchClass::SnapshotDrift,
                    surface: ReplayObservableSurface::Snapshot,
                    detail: "snapshot-visible state differed".to_string(),
                    expected: Some(format!("{:?}", original_surface)),
                    observed: Some(format!("{:?}", replayed_surface)),
                });
            }
        }
        if compared_surfaces.contains(&ReplayObservableSurface::BranchHead)
            && replay_runtime.branch_head_ref(&request.branch_id) != Some(&envelope.commit)
        {
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::BranchHeadDrift,
                surface: ReplayObservableSurface::BranchHead,
                detail: "branch head movement differed".to_string(),
                expected: Some(format!("{:?}", Some(&envelope.commit))),
                observed: Some(format!(
                    "{:?}",
                    replay_runtime.branch_head_ref(&request.branch_id)
                )),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::Lineage)
            && replay_runtime.lineage_events() != self.runtime.lineage_events()
        {
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::LineageDrift,
                surface: ReplayObservableSurface::Lineage,
                detail: "lineage events differed".to_string(),
                expected: Some(format!("{:?}", self.runtime.lineage_events())),
                observed: Some(format!("{:?}", replay_runtime.lineage_events())),
            });
        }
        if compared_surfaces.contains(&ReplayObservableSurface::DerivedIndexes)
            && replay_runtime.index_generations_at_version(envelope.commit.version_id)
                != self
                    .runtime
                    .index_generations_at_version(envelope.commit.version_id)
        {
            mismatches.push(ReplayMismatch {
                class: ReplayMismatchClass::DerivedIndexDrift,
                surface: ReplayObservableSurface::DerivedIndexes,
                detail: "derived index generations differed".to_string(),
                expected: Some(format!(
                    "{:?}",
                    self.runtime
                        .index_generations_at_version(envelope.commit.version_id)
                )),
                observed: Some(format!(
                    "{:?}",
                    replay_runtime.index_generations_at_version(envelope.commit.version_id)
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
        record_replay_diagnostic(self.runtime, &outcome.requested, &outcome);
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
}

impl RelationalRuntime {
    pub fn replay_authority(&mut self) -> ReplayAuthority<'_> {
        ReplayAuthority::new(self)
    }
}
