use serde_json::json;

use crate::data::diagnostics::RelationalDiagnosticArtifact;
use crate::data::diagnostics::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::data::diff::{
    PatchOrdering, PatchPublicationMode, PatchStreamPosition, RelationalPatchRecord,
};
use crate::data::history::CommitId;
use crate::data::identity::{EntityId, RelationId, VersionId};
use crate::data::publication::{PublicationError, PublicationStage, PublicationStatus};
use crate::data::transaction::{
    AuthoritativeApplyPlan, CommitConflict, CommitOutcome, MergedCommitPlan, RecordRef,
    RollbackOutcome, SavepointId, TransactionCommitError, TransactionIntent, TransactionOptions,
    WorkerIntentBatch,
};

use super::apply::apply_plan_to_staged_state;
use super::invariants::{first_blocking_invariant_error, first_publication_invariant_error};
use super::merge::{canonical_intent_key, detect_conflicting_updates, validate_intent};
use super::publication::publication_failure_diagnostic;
use super::{InvariantExecutionPoint, RelationalRuntime};

#[derive(Debug)]
pub struct RelationalTransaction<'a> {
    pub(super) runtime: &'a mut RelationalRuntime,
    pub(super) transaction_id: crate::data::transaction::TransactionId,
    pub(super) options: TransactionOptions,
    pub(super) batches: Vec<WorkerIntentBatch>,
    pub(super) savepoints: Vec<(SavepointId, usize)>,
    pub(super) last_merged_plan: Option<MergedCommitPlan>,
}

impl<'a> RelationalTransaction<'a> {
    pub fn transaction_id(&self) -> crate::data::transaction::TransactionId {
        self.transaction_id
    }

    pub fn push_batch(&mut self, batch: WorkerIntentBatch) {
        self.batches.push(batch);
    }

    pub fn create_savepoint(&mut self) -> SavepointId {
        assert!(
            self.options.allow_nested_savepoints,
            "nested savepoints are disabled for this transaction"
        );
        let savepoint_id = SavepointId(self.runtime.next_savepoint_id);
        self.runtime.next_savepoint_id += 1;
        self.savepoints.push((savepoint_id, self.batches.len()));
        savepoint_id
    }

    pub fn rollback_to_savepoint(
        &mut self,
        savepoint_id: SavepointId,
    ) -> Result<RollbackOutcome, CommitConflict> {
        let Some(index) = self
            .savepoints
            .iter()
            .position(|(candidate, _)| *candidate == savepoint_id)
        else {
            return Err(CommitConflict {
                code: DiagnosticCode::InvalidSavepoint,
                detail: format!("savepoint {:?} does not exist", savepoint_id),
            });
        };
        let (_, batch_len) = self.savepoints[index];
        let drained = self.batches.split_off(batch_len);
        self.savepoints.truncate(index);
        let restored_records = drained
            .into_iter()
            .flat_map(|batch| batch.intents.into_iter())
            .map(|intent| match intent {
                TransactionIntent::CreateEntity(_) => RecordRef::Entity(EntityId::new(u64::MAX, 0)),
                TransactionIntent::UpdateEntity { entity_id, .. } => RecordRef::Entity(entity_id),
                TransactionIntent::DeleteEntity { entity_id } => RecordRef::Entity(entity_id),
                TransactionIntent::CreateRelation(_) => {
                    RecordRef::Relation(RelationId::new(u64::MAX, 0))
                }
                TransactionIntent::DeleteRelation { relation_id } => {
                    RecordRef::Relation(relation_id)
                }
            })
            .collect();
        self.runtime.push_bounded_diagnostic(
            DiagnosticsScope::Transaction,
            DiagnosticsArtifactKind::Rollback,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::SavepointRolledBack,
                message: "rolled back to savepoint".to_string(),
                fields: json!({ "savepoint_id": savepoint_id.0 }),
            }],
        );
        Ok(RollbackOutcome {
            transaction_id: self.transaction_id,
            restored_records,
        })
    }

    pub fn merged_plan(&mut self) -> Result<&MergedCommitPlan, CommitConflict> {
        if self.last_merged_plan.is_none() {
            let plan = self.build_merged_plan()?;
            self.last_merged_plan = Some(plan);
        }
        Ok(self.last_merged_plan.as_ref().expect("merged plan"))
    }

    pub fn commit(self) -> Result<CommitOutcome, TransactionCommitError> {
        let merged_plan = self
            .build_merged_plan()
            .map_err(TransactionCommitError::Conflict)?;
        let current_state = self.runtime.current_state();
        let commit_boundary_results = self.runtime.run_invariants_for_state(
            &current_state,
            self.runtime.current_version_id(),
            InvariantExecutionPoint::CommitBoundary,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&commit_boundary_results) {
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::InvariantViolation,
                    message: error.detail.clone(),
                    fields: json!({ "execution_point": "commit_boundary" }),
                }],
            );
            return Err(TransactionCommitError::Conflict(error));
        }

        let version_id = VersionId(self.runtime.next_version_id);
        let apply_plan = AuthoritativeApplyPlan {
            transaction_id: self.transaction_id,
            version_id,
            merged_intents: merged_plan.merged_intents.clone(),
        };
        let mut staged = current_state.clone();
        let (changed_records, patch_records, diagnostics_entries) =
            apply_plan_to_staged_state(&mut staged, &apply_plan);

        let structural_results = self.runtime.run_invariants_for_state(
            &staged,
            version_id,
            InvariantExecutionPoint::MutationSensitive,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&structural_results) {
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::InvariantViolation,
                    message: error.detail.clone(),
                    fields: json!({ "execution_point": "mutation_sensitive" }),
                }],
            );
            return Err(TransactionCommitError::Conflict(error));
        }

        let commit_id = CommitId(self.runtime.next_commit_id);
        let patch = RelationalPatchRecord {
            ordering: PatchOrdering::CanonicalCommitOrder,
            publication_mode: PatchPublicationMode::CommitNative,
            position: PatchStreamPosition(commit_id.0),
            records: patch_records,
        };
        let diagnostics_summary = RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Transaction,
            kind: DiagnosticsArtifactKind::MinimalSummary,
            determinism: crate::data::diagnostics::DeterminismExpectation::Required,
            entries: diagnostics_entries
                .into_iter()
                .take(self.runtime.config.diagnostics.max_entries_per_artifact)
                .collect(),
        };

        let snapshot_results = self.runtime.run_invariants_for_state(
            &staged,
            version_id,
            InvariantExecutionPoint::SnapshotPublication,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_publication_invariant_error(&snapshot_results) {
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::Invariant,
                DiagnosticsArtifactKind::Failure,
                vec![publication_failure_diagnostic(error.detail.clone())],
            );
            return Err(TransactionCommitError::Publication(PublicationError {
                stage: PublicationStage::InvariantCheck,
                detail: error.detail,
            }));
        }

        let artifacts = self.runtime.assemble_publication_bundle(
            &staged,
            commit_id,
            version_id,
            patch.clone(),
            diagnostics_summary.clone(),
        );

        self.runtime.entity_arena = staged.entity_arena;
        self.runtime.relation_arena = staged.relation_arena;
        self.runtime.adjacency = staged.adjacency;
        self.runtime.next_commit_id += 1;
        self.runtime.next_version_id += 1;
        self.runtime.latest_publication_bundle = Some(artifacts.bundle.clone());
        self.runtime
            .push_diagnostic_artifact(artifacts.diagnostics_summary);
        self.runtime.push_bounded_diagnostic(
            DiagnosticsScope::PatchPublication,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "commit published coherently".to_string(),
                fields: json!({
                    "commit_id": commit_id.0,
                    "snapshot_id": artifacts.snapshot.snapshot_id.0
                }),
            }],
        );

        Ok(CommitOutcome {
            transaction_id: self.transaction_id,
            version_id,
            snapshot: artifacts.snapshot,
            changed_records,
            publication_status: PublicationStatus::Published,
        })
    }

    fn build_merged_plan(&self) -> Result<MergedCommitPlan, CommitConflict> {
        let mut intents = self
            .batches
            .iter()
            .flat_map(|batch| batch.intents.iter().cloned())
            .collect::<Vec<_>>();
        for intent in &intents {
            validate_intent(
                &self.runtime.current_state(),
                &self.runtime.config.schema_registry,
                intent,
            )?;
        }
        intents.sort_by_key(canonical_intent_key);
        detect_conflicting_updates(&intents)?;
        Ok(MergedCommitPlan {
            transaction_id: self.transaction_id,
            merged_intents: intents,
        })
    }
}
