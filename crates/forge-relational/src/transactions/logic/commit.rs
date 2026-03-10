use serde_json::json;

use super::planning::{bulk_reservations_for_plan, touched_partitions_for_plan};
use super::RelationalTransaction;
use crate::diagnostics::data::RelationalDiagnosticArtifact;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::durability::data::DurableCommitEnvelope;
use crate::history::data::{CommitId, CommitReference, VersionNode};
use crate::identity::data::VersionId;
use crate::logic::runtime::InvariantExecutionPoint;
use crate::logic::runtime::apply::apply_plan_to_staged_state;
use crate::publication::data::diff::{
    PatchCompatibilityClass, PatchOrdering, PatchPublicationMode, PatchStreamPosition,
    RelationalPatchRecord,
};
use crate::publication::data::{PublicationError, PublicationStage, PublicationStatus};
use crate::publication::logic::publication_failure_diagnostic;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::transactions::data::{AuthoritativeApplyPlan, CommitOutcome, TransactionCommitError};
use crate::validation::logic::{
    first_blocking_invariant_error, first_publication_invariant_error,
};

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
    /// Any failure before publication restores the detached working state back into the runtime
    /// without making the commit visible.
    pub fn commit(mut self) -> Result<CommitOutcome, TransactionCommitError> {
        let mut staged = self.runtime.take_working_state();
        let merged_plan = self
            .build_merged_plan_for_state(&staged)
            .map_err(TransactionCommitError::Conflict)?;
        {
            let mut counters = self
                .runtime
                .instrumentation
                .complexity_counters
                .borrow_mut();
            counters.partitions_touched_by_commit = touched_partitions_for_plan(&merged_plan);
            let (bulk_entity_slots_reserved, bulk_relation_slots_reserved) =
                bulk_reservations_for_plan(&staged, &merged_plan);
            counters.bulk_entity_slots_reserved = bulk_entity_slots_reserved;
            counters.bulk_relation_slots_reserved = bulk_relation_slots_reserved;
        }
        let commit_boundary_results = self.runtime.run_invariants_for_state(
            &staged,
            self.runtime.current_version_id(),
            InvariantExecutionPoint::CommitBoundary,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&commit_boundary_results) {
            staged.apply_to_runtime(&mut self.runtime.partitions);
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

        let version_id = VersionId(self.runtime.history.next_version_id);
        let apply_plan = AuthoritativeApplyPlan {
            transaction_id: self.transaction_id,
            version_id,
            merged_intents: merged_plan.merged_intents.clone(),
        };
        let (changed_records, patch_records, diagnostics_entries) = apply_plan_to_staged_state(
            &mut staged,
            &apply_plan,
            self.runtime.config.publication.patch_surface_policy,
            &self.runtime.config.schema_registry,
            self.runtime.config.cascade_delete_policy,
        );
        {
            let mut counters = self
                .runtime
                .instrumentation
                .complexity_counters
                .borrow_mut();
            counters.entity_slots_touched_by_commit = staged
                .mutation_journal
                .values()
                .map(|journal| journal.entity_slots.len())
                .sum();
            counters.relation_slots_touched_by_commit = staged
                .mutation_journal
                .values()
                .map(|journal| journal.relation_slots.len())
                .sum();
        }

        let structural_results = self.runtime.run_invariants_for_state(
            &staged,
            version_id,
            InvariantExecutionPoint::MutationSensitive,
            false,
            Some(&merged_plan),
        );
        if let Some(error) = first_blocking_invariant_error(&structural_results) {
            staged.apply_to_runtime(&mut self.runtime.partitions);
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

        let commit_id = CommitId(self.runtime.history.next_commit_id);
        let branch_id = self
            .options
            .target_branch
            .clone()
            .unwrap_or_else(|| self.runtime.config.main_branch.clone());
        let (parents, merge_base_commits) = match self.resolve_parent_commits(&branch_id) {
            Ok(result) => result,
            Err(conflict) => {
                staged.apply_to_runtime(&mut self.runtime.partitions);
                self.runtime.push_bounded_diagnostic(
                    DiagnosticsScope::History,
                    DiagnosticsArtifactKind::Failure,
                    vec![RelationalDiagnosticsEntry {
                        code: conflict.code,
                        message: conflict.detail.clone(),
                        fields: json!({
                            "branch_id": branch_id.0,
                            "merge_parent_branches": self.options.merge_parent_branches.iter().map(|branch| branch.0.clone()).collect::<Vec<_>>(),
                        }),
                    }],
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
        let patch = RelationalPatchRecord {
            ordering: PatchOrdering::CanonicalCommitOrder,
            publication_mode: PatchPublicationMode::CommitNative,
            position: PatchStreamPosition(commit_id.0),
            compatibility: match self.runtime.config.publication.patch_surface_policy {
                crate::config::data::PatchSurfacePolicy::StructuredPatchSurface => {
                    PatchCompatibilityClass::StructuredCompatible
                }
                crate::config::data::PatchSurfacePolicy::DensePatchSurface => {
                    PatchCompatibilityClass::DenseCompatible
                }
            },
            records: patch_records,
        }
        .canonicalized();
        if patch.records.len() > self.runtime.config.publication.max_patch_records_per_commit {
            staged.apply_to_runtime(&mut self.runtime.partitions);
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::PatchPublication,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::DiagnosticsPublicationFailure,
                    message: "patch record budget exceeded".to_string(),
                    fields: json!({
                        "patch_records": patch.records.len(),
                        "max_patch_records_per_commit": self.runtime.config.publication.max_patch_records_per_commit,
                    }),
                }],
            );
            return Err(TransactionCommitError::Publication(PublicationError {
                stage: PublicationStage::BundleAssembly,
                detail: "patch record budget exceeded".to_string(),
            }));
        }
        let diagnostics_summary = RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Transaction,
            kind: DiagnosticsArtifactKind::MinimalSummary,
            determinism: crate::diagnostics::data::DeterminismExpectation::Required,
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
            staged.apply_to_runtime(&mut self.runtime.partitions);
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
            commit_reference.clone(),
            version_id,
            patch.clone(),
            diagnostics_summary.clone(),
        );
        let lineage_event_ids = self.runtime.ensure_lineage_for_commit(
            &mut staged,
            &commit_reference,
            &merged_plan.merged_intents,
            &changed_records,
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
        if let Err(error) = self.runtime.append_durable_commit(DurableCommitEnvelope {
            envelope: canonical_commit_envelope.clone(),
        }) {
            staged.apply_to_runtime(&mut self.runtime.partitions);
            self.runtime.push_bounded_diagnostic(
                DiagnosticsScope::History,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::DurableAppendFailed,
                    message: error.detail.clone(),
                    fields: json!({
                        "commit_id": commit_id.0,
                        "branch_id": branch_id.0,
                    }),
                }],
            );
            return Err(TransactionCommitError::Publication(PublicationError {
                stage: PublicationStage::Visibility,
                detail: error.detail,
            }));
        }

        staged.apply_to_runtime(&mut self.runtime.partitions);
        self.runtime
            .refresh_unique_field_index_for_records(&changed_records, version_id);
        self.runtime.pin_snapshot_state(&artifacts.snapshot_state);
        self.runtime
            .snapshots
            .version_visibility_cache
            .insert(version_id, artifacts.snapshot_state.clone());
        self.runtime
            .snapshots
            .active
            .insert(artifacts.snapshot.snapshot_id, artifacts.snapshot_state);
        self.runtime
            .trim_live_history_for_records(&changed_records, version_id);
        self.runtime.history.next_commit_id += 1;
        self.runtime.history.next_version_id += 1;
        self.runtime
            .history
            .branch_heads
            .insert(branch_id.clone(), Some(commit_reference.clone()));
        self.runtime
            .history
            .commit_graph
            .insert(commit_id, VersionNode { commit: commit_reference.clone() });
        self.runtime
            .history
            .commit_envelopes
            .insert(commit_id, canonical_commit_envelope.clone());
        self.runtime.compact_durable_log_if_needed();
        self.runtime.publication.latest_bundle = Some(artifacts.bundle.clone());
        self.runtime
            .push_diagnostic_artifact(artifacts.diagnostics_summary);
        let _ = self.runtime.run_retention_pass();
        let publication_code = if commit_reference.parents.len() > 1 {
            DiagnosticCode::MergeCommitPublished
        } else {
            DiagnosticCode::CommitPublished
        };
        let mut publication_entries = Vec::new();
        if commit_reference.parents.len() > 1 {
            publication_entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::MergeBaseResolved,
                message: "merge bases resolved deterministically".to_string(),
                fields: json!({
                    "commit_id": commit_id.0,
                    "merge_base_commit_ids": merge_base_commits.iter().map(|base| base.0).collect::<Vec<_>>(),
                }),
            });
        }
        publication_entries.push(RelationalDiagnosticsEntry {
            code: publication_code,
            message: if commit_reference.parents.len() > 1 {
                "merge commit published coherently".to_string()
            } else {
                "commit published coherently".to_string()
            },
            fields: json!({
                "commit_id": commit_id.0,
                "snapshot_id": artifacts.snapshot.snapshot_id.0,
                "branch_id": branch_id.0,
                "parent_commit_ids": commit_reference.parents.iter().map(|parent| parent.0).collect::<Vec<_>>(),
                "merge_parent_branches": self.options.merge_parent_branches.iter().map(|branch| branch.0.clone()).collect::<Vec<_>>(),
                "merge_base_commit_ids": merge_base_commits.iter().map(|base| base.0).collect::<Vec<_>>(),
            }),
        });
        self.runtime.push_bounded_diagnostic(
            DiagnosticsScope::PatchPublication,
            DiagnosticsArtifactKind::MinimalSummary,
            publication_entries,
        );

        Ok(CommitOutcome {
            transaction_id: self.transaction_id,
            commit: commit_reference,
            version_id,
            snapshot: artifacts.snapshot,
            changed_records,
            publication_status: PublicationStatus::Published,
        })
    }
}
