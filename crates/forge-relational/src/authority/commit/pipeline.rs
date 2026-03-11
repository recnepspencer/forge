use serde_json::json;

use crate::authority::commit::plan_building::bulk_reservations_for_plan;
use crate::authority::commit::publication::{
    assemble_patch, diagnostics_summary_artifact, finalize_published_commit,
};
use crate::authority::commit::touched_scope::touched_partitions_for_plan_set;
use crate::transactions::logic::RelationalTransaction;
use crate::authority::mutation::apply_plan_to_draft;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
};
use crate::durability::data::DurableCommitEnvelope;
use crate::history::data::{CommitId, CommitReference};
use crate::identity::data::VersionId;
use crate::logic::runtime::InvariantExecutionPoint;
use crate::publication::data::{PublicationError, PublicationStage, PublicationStatus};
use crate::publication::logic::publication_failure_diagnostic;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::transactions::data::{
    AuthoritativeApplyPlan, CommitOutcome, RecordRef, TransactionCommitError,
};
use crate::validation::logic::{first_blocking_invariant_error, first_publication_invariant_error};

impl<'a> RelationalTransaction<'a> {
    /// Executes the serialized truth-commit pipeline.
    ///
    /// The phases are intentionally explicit:
    /// 1. build a deterministic merged plan over the current immutable committed state
    /// 2. run commit-boundary invariants before any authoritative mutation
    /// 3. apply the authoritative plan into detached working state
    /// 4. run mutation-sensitive and publication invariants
    /// 5. assemble the canonical publication bundle and durable envelope
    /// 6. publish history/version visibility atomically into the runtime on success
    ///
    /// Any failure before publication discards the touched-partition overlay without making the
    /// commit visible.
    pub fn commit(mut self) -> Result<CommitOutcome, TransactionCommitError> {
        let planning_state = crate::logic::runtime::WorkingState::new(
            self.runtime.partitions.clone(),
            self.runtime.config.adjacency_policy.clone(),
        );
        let merged_plan = self
            .build_merged_plan_for_state(&planning_state)
            .map_err(TransactionCommitError::Conflict)?;
        let touched_partitions =
            touched_partitions_for_plan_set(&self.runtime.current_state(), &merged_plan);
        let mut draft = self
            .runtime
            .touched_partition_overlay(touched_partitions.iter().copied());
        {
            let mut counters = self
                .runtime
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned");
            counters.partitions_touched_by_commit = draft.touched_partitions().len();
            let (bulk_entity_slots_reserved, bulk_relation_slots_reserved) =
                bulk_reservations_for_plan(&planning_state, &merged_plan);
            counters.bulk_entity_slots_reserved = bulk_entity_slots_reserved;
            counters.bulk_relation_slots_reserved = bulk_relation_slots_reserved;
        }
        let commit_boundary_results = {
            let committed_state = self.runtime.current_state();
            self.runtime.run_invariants_for_state(
                &committed_state,
                self.runtime.current_version_id(),
                InvariantExecutionPoint::CommitBoundary,
                false,
                Some(&merged_plan),
            )
        };
        if let Some(error) = first_blocking_invariant_error(&commit_boundary_results) {
            self.runtime
                .diagnostic(DiagnosticsScope::Invariant)
                .failure()
                .emit_entry(
                    DiagnosticCode::InvariantViolation,
                    error.detail.clone(),
                    json!({ "execution_point": "commit_boundary" }),
                );
            return Err(TransactionCommitError::Conflict(error));
        }

        let version_id = VersionId(self.runtime.history.next_version_id);
        let apply_plan = AuthoritativeApplyPlan {
            transaction_id: self.transaction_id,
            version_id,
            merged_intents: merged_plan.merged_intents.clone(),
        };
        let mutation_config = self.runtime.mutation_config();
        let effect = apply_plan_to_draft(
            &mut draft,
            &apply_plan,
            &mutation_config,
            &self.runtime.config.schema_registry,
            &mut self.runtime.symbols,
        )
        .map_err(TransactionCommitError::Conflict)?;
        {
            let mut counters = self
                .runtime
                .instrumentation
                .complexity_counters
                .lock()
                .expect("complexity counter lock poisoned");
            counters.entity_slots_touched_by_commit = draft
                .mutation_journal()
                .values()
                .map(|journal| journal.entity_slots.len())
                .sum();
            counters.relation_slots_touched_by_commit = draft
                .mutation_journal()
                .values()
                .map(|journal| journal.relation_slots.len())
                .sum();
        }

        let overlay_state = self.runtime.overlay_state_view(&draft);
        let structural_results = self.runtime.run_invariants_for_state(
            &overlay_state,
            version_id,
            InvariantExecutionPoint::MutationSensitive,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&structural_results) {
            self.runtime
                .diagnostic(DiagnosticsScope::Invariant)
                .failure()
                .emit_entry(
                    DiagnosticCode::InvariantViolation,
                    error.detail.clone(),
                    json!({ "execution_point": "mutation_sensitive" }),
                );
            return Err(TransactionCommitError::Conflict(error));
        }

        let commit_id = CommitId(self.runtime.history.next_commit_id);
        let branch_id = self
            .options
            .target_branch
            .clone()
            .unwrap_or_else(|| self.runtime.config.main_branch.clone());
        let previous_branch_head = self.runtime.branch_head(&branch_id).cloned();
        let (parents, merge_base_commits) = match self.resolve_parent_commits(&branch_id) {
            Ok(result) => result,
            Err(conflict) => {
                self.runtime
                    .diagnostic(DiagnosticsScope::History)
                    .failure()
                    .emit_entry(
                        conflict.code,
                        conflict.detail.clone(),
                        json!({
                            "branch_id": branch_id.0,
                            "merge_parent_branches": self.options.merge_parent_branches.iter().map(|branch| branch.0.clone()).collect::<Vec<_>>(),
                        }),
                    );
                return Err(TransactionCommitError::Conflict(conflict));
            }
        };
        let commit_reference = CommitReference {
            commit_id,
            version_id,
            branch_id: branch_id.clone(),
            parents,
        };
        let patch = assemble_patch(&self.runtime.config, commit_id, &effect);
        let max_patch_records_per_commit =
            self.runtime.config.publication.max_patch_records_per_commit;
        if patch.records.len() > max_patch_records_per_commit {
            self.runtime
                .diagnostic(DiagnosticsScope::PatchPublication)
                .failure()
                .emit_entry(
                    DiagnosticCode::DiagnosticsPublicationFailure,
                    "patch record budget exceeded",
                    json!({
                        "patch_records": patch.records.len(),
                        "max_patch_records_per_commit": max_patch_records_per_commit,
                    }),
                );
            return Err(TransactionCommitError::Publication(PublicationError {
                stage: PublicationStage::BundleAssembly,
                detail: "patch record budget exceeded".to_string(),
            }));
        }
        let diagnostics_summary = diagnostics_summary_artifact(&self.runtime.config, &effect);

        let snapshot_results = self.runtime.run_invariants_for_state(
            &overlay_state,
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
            commit_reference.clone(),
            version_id,
            patch.clone(),
            diagnostics_summary.clone(),
        );
        let published_snapshot = artifacts.snapshot.clone();
        let lineage_event_ids = self.runtime.ensure_lineage_for_commit(
            &mut draft,
            &commit_reference,
            &merged_plan.merged_intents,
            &effect.changed_records,
        );
        let canonical_commit_envelope = CanonicalCommitEnvelope {
            commit: commit_reference.clone(),
            branch_context: branch_id.clone(),
            merge_parent_branches: self.options.merge_parent_branches.clone(),
            merge_base_commits: merge_base_commits.clone(),
            schema_version: self.runtime.primary_schema_version(),
            schema_registry: self.runtime.config.schema_registry.clone(),
            merged_plan: merged_plan.clone(),
            patch: patch.clone(),
            diagnostics_summary: diagnostics_summary.clone(),
            lineage_event_ids,
            index_generation_ids: Vec::new(),
        };
        let mut changed_records = effect.changed_records;
        canonicalize_changed_records(&mut changed_records);

        if let Err(error) = self.runtime.append_durable_commit(DurableCommitEnvelope {
            envelope: canonical_commit_envelope.clone(),
        }) {
            self.runtime
                .diagnostic(DiagnosticsScope::History)
                .failure()
                .emit_entry(
                    DiagnosticCode::DurableAppendFailed,
                    error.detail.clone(),
                    json!({
                        "commit_id": commit_id.0,
                        "branch_id": branch_id.0,
                    }),
                );
            return Err(TransactionCommitError::Publication(PublicationError {
                stage: PublicationStage::Visibility,
                detail: error.detail,
            }));
        }

        finalize_published_commit(
            self.runtime,
            draft.commit(),
            &changed_records,
            version_id,
            previous_branch_head.as_ref().map(|head| head.version_id),
            commit_id,
            &commit_reference,
            canonical_commit_envelope,
            patch.position,
            branch_id,
            merge_base_commits,
            artifacts,
            self.options.merge_parent_branches.clone(),
        );

        Ok(CommitOutcome {
            transaction_id: self.transaction_id,
            commit: commit_reference,
            version_id,
            snapshot: published_snapshot,
            changed_records,
            publication_status: PublicationStatus::Published,
        })
    }
}

fn canonicalize_changed_records(records: &mut Vec<RecordRef>) {
    records.sort_by(|left, right| {
        canonical_record_sort_key(left).cmp(&canonical_record_sort_key(right))
    });
    records.dedup();
}

fn canonical_record_sort_key(
    record: &RecordRef,
) -> (u8, crate::identity::data::PartitionId, u64, u32) {
    match record {
        RecordRef::Entity(entity_id) => (
            0,
            entity_id.partition_id,
            entity_id.local_slot.0,
            entity_id.generation.0,
        ),
        RecordRef::Relation(relation_id) => (
            1,
            relation_id.partition_id,
            relation_id.local_slot.0,
            relation_id.generation.0,
        ),
    }
}
