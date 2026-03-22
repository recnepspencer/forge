use crate::authority::commit::phases::artifacts::prepare_publication_artifacts;
use crate::authority::commit::phases::finalize::{
    finalize_commit_publication, FinalizeCommitInput,
};
use crate::authority::commit::phases::history::resolve_commit_history;
use crate::authority::commit::phases::mutation::run_authoritative_mutation;
use crate::authority::commit::phases::prepare::{
    prepare_working_state_scope, record_preparation_counters,
};
use crate::authority::commit::phases::publication::{append_durable_commit, enforce_patch_budget};
use crate::authority::commit::phases::schema_continuity::{
    emit_schema_continuity_diagnostic, resolve_schema_continuity,
};
use crate::authority::commit::publication::assemble_patch;
#[cfg(test)]
use crate::authority::commit::{
    preparation::diagnostics::{emit_preparation_failure, failures::PreparationFailureClass},
    publication::{current_test_diff_preparation_fault, TestDiffPreparationFault},
};
use crate::publication::data::PublicationStatus;
use crate::schema::data::SchemaTransitionSummary;
use crate::transactions::data::{
    CommitExecution, CommitLog, CommitOutcome, CommitPatchBudgetSummary, CommitPhase,
    CommitPhaseTiming, CommitPublication, CommitResult, CommitSchemaSummary, CommitValidation,
    TransactionCommitError,
};
use crate::transactions::logic::RelationalTransaction;
use std::sync::Arc;
use std::time::Instant;

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
    pub fn commit(mut self) -> Result<CommitResult, TransactionCommitError> {
        let mut commit_log = CommitLog::new();
        let mut phase_timing = CommitPhaseTiming::default();
        let diagnostics_start = self
            .runtime
            .publication_access()
            .diagnostics()
            .artifacts()
            .len();
        let complexity_before = self
            .runtime
            .services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned")
            .clone();
        let mut invariant_executions = Vec::new();

        commit_log.begin_phase(CommitPhase::DraftPreparation);
        let phase_started = Instant::now();
        let prepared = prepare_working_state_scope(&mut self).map_err(|error| {
            attach_rejection(&mut commit_log, CommitPhase::DraftPreparation, error)
        })?;
        let merged_plan = prepared.merged_plan;
        let structural_summary = prepared.structural_summary;
        let public_structural_summary = structural_summary.public_summary();
        let mut working_state = prepared.working_state;
        record_preparation_counters(self.runtime, &working_state, &structural_summary);
        commit_log.record_structural_summary(&public_structural_summary);
        commit_log.complete_phase(CommitPhase::DraftPreparation);
        phase_timing.working_state_preparation_micros = elapsed_micros(phase_started);

        commit_log.begin_phase(CommitPhase::InvariantPreCheck);
        let phase_started = Instant::now();
        let pre_commit_invariants = self
            .runtime
            .invariant_authority()
            .enforce_commit_boundary(&merged_plan)
            .map_err(|error| {
                attach_rejection(&mut commit_log, CommitPhase::InvariantPreCheck, error)
            })?;
        commit_log.record_invariant_outcomes(&pre_commit_invariants);
        invariant_executions.push(pre_commit_invariants);
        commit_log.complete_phase(CommitPhase::InvariantPreCheck);
        phase_timing.invariant_pre_check_micros = elapsed_micros(phase_started);

        commit_log.begin_phase(CommitPhase::AuthoritativeMutation);
        let phase_started = Instant::now();
        let mutation = run_authoritative_mutation(&mut self, &mut working_state, &merged_plan)
            .map_err(|error| {
                attach_rejection(&mut commit_log, CommitPhase::AuthoritativeMutation, error)
            })?;
        let version_id = mutation.version_id;
        let mut effect = mutation.effect;
        commit_log.record_invariant_outcomes(&mutation.invariant_results);
        invariant_executions.push(mutation.invariant_results);
        commit_log.complete_phase(CommitPhase::AuthoritativeMutation);
        phase_timing.authoritative_mutation_micros = elapsed_micros(phase_started);

        commit_log.begin_phase(CommitPhase::HistoryResolution);
        let phase_started = Instant::now();
        let history = resolve_commit_history(&mut self, version_id).map_err(|error| {
            attach_rejection(&mut commit_log, CommitPhase::HistoryResolution, error)
        })?;
        let commit_id = history.commit_id;
        let branch_id = history.branch_id.clone();
        let commit_reference = history.commit_reference.clone();
        let history_summary = history.summary();
        let merge_base_commits = history.merge_base_commits;
        let merge_parent_branches = self.options.merge_parent_branches.clone();
        commit_log.record_history_resolution(&history_summary);
        commit_log.complete_phase(CommitPhase::HistoryResolution);
        phase_timing.history_resolution_micros = elapsed_micros(phase_started);

        let schema_continuity =
            resolve_schema_continuity(&mut *self.runtime, &branch_id, &self.options).map_err(|error| {
                attach_rejection(&mut commit_log, CommitPhase::ArtifactAssembly, error)
            })?;
        emit_schema_continuity_diagnostic(self.runtime, &branch_id, &schema_continuity);

        {
            commit_log.begin_phase(CommitPhase::InvariantPostCheck);
            let phase_started = Instant::now();
            let post_invariants = self
                .runtime
                .invariant_authority()
                .enforce_snapshot_publication_for_working_state(
                    &working_state,
                    version_id,
                    &merged_plan,
                )
                .map_err(TransactionCommitError::publication)
                .map_err(|error| {
                    attach_rejection(&mut commit_log, CommitPhase::InvariantPostCheck, error)
                })?;
            commit_log.record_invariant_outcomes(&post_invariants);
            invariant_executions.push(post_invariants);
            commit_log.complete_phase(CommitPhase::InvariantPostCheck);
            phase_timing.invariant_post_check_micros = elapsed_micros(phase_started);
        }

        commit_log.begin_phase(CommitPhase::ArtifactAssembly);
        let phase_started = Instant::now();
        let patch_records = std::mem::take(&mut effect.publication.patch_records);
        let patch = assemble_patch(self.runtime, commit_reference.commit_id, patch_records);
        #[cfg(test)]
        if let Some(fault) = current_test_diff_preparation_fault() {
            let (failure_class, label) = match fault {
                TestDiffPreparationFault::FragmentCanonicalizationFailure => (
                    PreparationFailureClass::FragmentCanonicalizationFailure,
                    "fragment_canonicalization_failure",
                ),
                TestDiffPreparationFault::PacketOverlapDetected => (
                    PreparationFailureClass::PacketOverlapDetected,
                    "packet_overlap_detected",
                ),
            };
            emit_preparation_failure(
                self.runtime,
                crate::diagnostics::data::DiagnosticsScope::PatchPublication,
                failure_class,
                serde_json::json!({
                    "failure_class": label,
                    "commit_id": commit_reference.commit_id.0,
                    "patch_record_count": patch.records.len(),
                }),
            );
        }
        let patch_budget_summary = CommitPatchBudgetSummary {
            patch_record_count: patch.records.len(),
            max_patch_records_per_commit: self
                .runtime
                .config
                .publication
                .policy
                .max_patch_records_per_commit,
        };
        commit_log.record_patch_budget(&patch_budget_summary);
        enforce_patch_budget(self.runtime, &patch).map_err(|error| {
            attach_rejection(&mut commit_log, CommitPhase::ArtifactAssembly, error)
        })?;
        let publication = prepare_publication_artifacts(
            self.runtime,
            &mut working_state,
            patch,
            &commit_reference,
            &branch_id,
            version_id,
            &merge_parent_branches,
            &merge_base_commits,
            &merged_plan,
            &schema_continuity,
            effect,
        )
        .map_err(|error| attach_rejection(&mut commit_log, CommitPhase::ArtifactAssembly, error))?;
        let change_summary = publication.change_summary.clone();
        let aspect_summary = publication.aspect_summary.clone();
        let aspect_evaluation_traces = publication.aspect_evaluation_traces.clone();
        let aspect_emission_traces = publication.aspect_emission_traces.clone();
        let publication_summary = publication.summary.clone();
        let publication_snapshot = publication.finalize.artifacts.bundle.snapshot.clone();
        if self
            .runtime
            .config
            .diagnostics
            .profile
            .detailed_traces_enabled
        {
            for trace in &aspect_evaluation_traces {
                self.runtime
                    .publication_authority()
                    .push_diagnostic_artifact(trace.diagnostic_artifact());
            }
            for trace in &aspect_emission_traces {
                self.runtime
                    .publication_authority()
                    .push_diagnostic_artifact(trace.diagnostic_artifact());
            }
        }
        commit_log.record_changed_records(&change_summary);
        commit_log.record_aspect_summary(&aspect_summary);
        commit_log.record_publication_artifacts(&publication_summary);
        commit_log.complete_phase(CommitPhase::ArtifactAssembly);
        phase_timing.artifact_assembly_micros = elapsed_micros(phase_started);

        commit_log.begin_phase(CommitPhase::DurableAppend);
        let phase_started = Instant::now();
        commit_log.record_durable_append_prepared(
            commit_id,
            &branch_id.0,
            publication
                .finalize
                .canonical_commit_envelope
                .patch
                .position,
        );
        append_durable_commit(
            self.runtime,
            &publication.finalize.canonical_commit_envelope,
            commit_id,
            &branch_id,
        )
        .map_err(|error| attach_rejection(&mut commit_log, CommitPhase::DurableAppend, error))?;
        commit_log.complete_phase(CommitPhase::DurableAppend);
        phase_timing.durable_append_micros = elapsed_micros(phase_started);

        commit_log.begin_phase(CommitPhase::Publication);
        let phase_started = Instant::now();
        let crate::authority::commit::phases::artifacts::PublicationPreparation {
            change_summary: _,
            aspect_summary: _,
            aspect_evaluation_traces: _,
            aspect_emission_traces: _,
            summary: _,
            finalize:
                crate::authority::commit::phases::artifacts::PublicationFinalizeArtifacts {
                    artifacts,
                    changed_records,
                    canonical_commit_envelope,
                    adjacency_deltas,
                },
        } = publication;
        let canonical_commit_envelope = Arc::new(canonical_commit_envelope);
        finalize_commit_publication(
            self.runtime,
            working_state,
            FinalizeCommitInput {
                changed_records: &changed_records,
                version_id,
                previous_branch_head_version: history.previous_branch_head_version,
                commit_id,
                commit_reference: &commit_reference,
                canonical_commit_envelope: canonical_commit_envelope.clone(),
                branch_id: &branch_id,
                merge_base_commits: &merge_base_commits,
                artifacts,
                merge_parent_branches: &merge_parent_branches,
                adjacency_deltas,
            },
        );
        commit_log.record_commit_published(commit_id, &commit_reference.branch_id.0);
        commit_log.complete_phase(CommitPhase::Publication);
        phase_timing.publication_micros = elapsed_micros(phase_started);

        let complexity_after = self
            .runtime
            .services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned")
            .clone();
        let diagnostics = self
            .runtime
            .publication_access()
            .diagnostics_since(diagnostics_start);
        let commit_summary = commit_log.summary().clone();
        let schema_summary = CommitSchemaSummary {
            transition: canonical_commit_envelope
                .schema_transition
                .as_ref()
                .map(SchemaTransitionSummary::from_artifact),
            descriptor_semantics_version: canonical_commit_envelope.descriptor_semantics_version,
        };
        let commit_outcome = CommitOutcome {
            transaction_id: self.transaction_id,
            commit: commit_reference,
            version_id,
            snapshot: publication_snapshot,
            changed_records,
            publication_status: PublicationStatus::Published,
            commit_log,
        };

        Ok(CommitResult {
            outcome: commit_outcome,
            summary: commit_summary,
            structural_summary: public_structural_summary,
            schema_summary,
            publication: CommitPublication {
                diagnostics,
                envelope: canonical_commit_envelope,
                aspect_evaluation_traces,
                aspect_emission_traces,
            },
            validation: CommitValidation {
                summary: CommitValidation::summarize(&invariant_executions),
                invariant_executions,
            },
            execution: CommitExecution {
                phase_timing,
                complexity_delta: complexity_delta(complexity_before, complexity_after),
            },
        })
    }
}

fn attach_rejection(
    commit_log: &mut CommitLog,
    phase: CommitPhase,
    error: TransactionCommitError,
) -> TransactionCommitError {
    match &error {
        TransactionCommitError::Conflict { error, .. } => {
            commit_log.record_rejection(phase, Some(error.code()), None, error.detail());
        }
        TransactionCommitError::Publication { error, .. } => {
            commit_log.record_rejection(phase, None, Some(error.stage), error.detail.clone());
        }
    }
    error.with_commit_log(commit_log.clone())
}

fn elapsed_micros(started: Instant) -> u64 {
    started.elapsed().as_micros() as u64
}

fn complexity_delta(
    before: crate::performance::data::RuntimeComplexityCounters,
    after: crate::performance::data::RuntimeComplexityCounters,
) -> crate::performance::data::RuntimeComplexityCounters {
    use crate::performance::data::RuntimeComplexityCounters;

    RuntimeComplexityCounters {
        full_state_clones: after
            .full_state_clones
            .saturating_sub(before.full_state_clones),
        partitions_cloned: after
            .partitions_cloned
            .saturating_sub(before.partitions_cloned),
        entity_slots_cloned: after
            .entity_slots_cloned
            .saturating_sub(before.entity_slots_cloned),
        relation_slots_cloned: after
            .relation_slots_cloned
            .saturating_sub(before.relation_slots_cloned),
        commit_topology_flags: after.commit_topology_flags,
        partitions_touched_by_commit: after
            .partitions_touched_by_commit
            .saturating_sub(before.partitions_touched_by_commit),
        entity_slots_touched_by_commit: after
            .entity_slots_touched_by_commit
            .saturating_sub(before.entity_slots_touched_by_commit),
        relation_slots_touched_by_commit: after
            .relation_slots_touched_by_commit
            .saturating_sub(before.relation_slots_touched_by_commit),
        bulk_entity_slots_reserved: after
            .bulk_entity_slots_reserved
            .saturating_sub(before.bulk_entity_slots_reserved),
        bulk_relation_slots_reserved: after
            .bulk_relation_slots_reserved
            .saturating_sub(before.bulk_relation_slots_reserved),
        relation_identity_candidates_scanned: after
            .relation_identity_candidates_scanned
            .saturating_sub(before.relation_identity_candidates_scanned),
        visibility_entity_slot_scans: after
            .visibility_entity_slot_scans
            .saturating_sub(before.visibility_entity_slot_scans),
        visibility_relation_slot_scans: after
            .visibility_relation_slot_scans
            .saturating_sub(before.visibility_relation_slot_scans),
        visible_entity_records_materialized: after
            .visible_entity_records_materialized
            .saturating_sub(before.visible_entity_records_materialized),
        visible_relation_records_materialized: after
            .visible_relation_records_materialized
            .saturating_sub(before.visible_relation_records_materialized),
        visibility_cache_hits: after
            .visibility_cache_hits
            .saturating_sub(before.visibility_cache_hits),
        visibility_cache_miss_reconstructions: after
            .visibility_cache_miss_reconstructions
            .saturating_sub(before.visibility_cache_miss_reconstructions),
        visibility_cache_recent_evictions: after
            .visibility_cache_recent_evictions
            .saturating_sub(before.visibility_cache_recent_evictions),
        visibility_cache_branch_head_promotions: after
            .visibility_cache_branch_head_promotions
            .saturating_sub(before.visibility_cache_branch_head_promotions),
        visibility_cache_replay_promotions: after
            .visibility_cache_replay_promotions
            .saturating_sub(before.visibility_cache_replay_promotions),
        visibility_cache_snapshot_promotions: after
            .visibility_cache_snapshot_promotions
            .saturating_sub(before.visibility_cache_snapshot_promotions),
        invariant_entity_slot_scans: after
            .invariant_entity_slot_scans
            .saturating_sub(before.invariant_entity_slot_scans),
        invariant_relation_slot_scans: after
            .invariant_relation_slot_scans
            .saturating_sub(before.invariant_relation_slot_scans),
        invariant_entity_records_materialized: after
            .invariant_entity_records_materialized
            .saturating_sub(before.invariant_entity_records_materialized),
        invariant_relation_records_materialized: after
            .invariant_relation_records_materialized
            .saturating_sub(before.invariant_relation_records_materialized),
        relation_integrity_contracts_evaluated: after
            .relation_integrity_contracts_evaluated
            .saturating_sub(before.relation_integrity_contracts_evaluated),
        relation_endpoint_kind_checks: after
            .relation_endpoint_kind_checks
            .saturating_sub(before.relation_endpoint_kind_checks),
        relation_cardinality_checks: after
            .relation_cardinality_checks
            .saturating_sub(before.relation_cardinality_checks),
        relation_uniqueness_checks: after
            .relation_uniqueness_checks
            .saturating_sub(before.relation_uniqueness_checks),
        relation_uniqueness_candidates_scanned: after
            .relation_uniqueness_candidates_scanned
            .saturating_sub(before.relation_uniqueness_candidates_scanned),
        relation_symmetry_checks: after
            .relation_symmetry_checks
            .saturating_sub(before.relation_symmetry_checks),
        relation_endpoint_deletion_checks: after
            .relation_endpoint_deletion_checks
            .saturating_sub(before.relation_endpoint_deletion_checks),
        preparation_packet_count: after
            .preparation_packet_count
            .saturating_sub(before.preparation_packet_count),
        preparation_packet_item_count: after
            .preparation_packet_item_count
            .saturating_sub(before.preparation_packet_item_count),
        preparation_packet_peak_width_total: after
            .preparation_packet_peak_width_total
            .saturating_sub(before.preparation_packet_peak_width_total),
        preparation_scope_unit_count: after
            .preparation_scope_unit_count
            .saturating_sub(before.preparation_scope_unit_count),
        preparation_parallel_legal_count: after
            .preparation_parallel_legal_count
            .saturating_sub(before.preparation_parallel_legal_count),
        preparation_parallel_profitable_count: after
            .preparation_parallel_profitable_count
            .saturating_sub(before.preparation_parallel_profitable_count),
        preparation_serial_strategy_count: after
            .preparation_serial_strategy_count
            .saturating_sub(before.preparation_serial_strategy_count),
        preparation_staged_parallel_strategy_count: after
            .preparation_staged_parallel_strategy_count
            .saturating_sub(before.preparation_staged_parallel_strategy_count),
        preparation_reducer_conflict_count: after
            .preparation_reducer_conflict_count
            .saturating_sub(before.preparation_reducer_conflict_count),
        post_commit_consumer_packet_count: after
            .post_commit_consumer_packet_count
            .saturating_sub(before.post_commit_consumer_packet_count),
        post_commit_consumer_item_count: after
            .post_commit_consumer_item_count
            .saturating_sub(before.post_commit_consumer_item_count),
        post_commit_consumer_peak_width_total: after
            .post_commit_consumer_peak_width_total
            .saturating_sub(before.post_commit_consumer_peak_width_total),
        post_commit_scope_unit_count: after
            .post_commit_scope_unit_count
            .saturating_sub(before.post_commit_scope_unit_count),
        post_commit_serial_strategy_count: after
            .post_commit_serial_strategy_count
            .saturating_sub(before.post_commit_serial_strategy_count),
        post_commit_parallel_strategy_count: after
            .post_commit_parallel_strategy_count
            .saturating_sub(before.post_commit_parallel_strategy_count),
        snapshot_pin_adjustments: after
            .snapshot_pin_adjustments
            .saturating_sub(before.snapshot_pin_adjustments),
        snapshot_pin_full_rebuilds: after
            .snapshot_pin_full_rebuilds
            .saturating_sub(before.snapshot_pin_full_rebuilds),
        retention_entity_slots_scanned: after
            .retention_entity_slots_scanned
            .saturating_sub(before.retention_entity_slots_scanned),
        retention_relation_slots_scanned: after
            .retention_relation_slots_scanned
            .saturating_sub(before.retention_relation_slots_scanned),
        inspection_structural_identity_lookups: after
            .inspection_structural_identity_lookups
            .saturating_sub(before.inspection_structural_identity_lookups),
        inspection_structural_identity_query_scans: after
            .inspection_structural_identity_query_scans
            .saturating_sub(before.inspection_structural_identity_query_scans),
        inspection_graph_summary_requests: after
            .inspection_graph_summary_requests
            .saturating_sub(before.inspection_graph_summary_requests),
        inspection_kind_summary_requests: after
            .inspection_kind_summary_requests
            .saturating_sub(before.inspection_kind_summary_requests),
        inspection_connectivity_summary_requests: after
            .inspection_connectivity_summary_requests
            .saturating_sub(before.inspection_connectivity_summary_requests),
        inspection_neighbor_requests: after
            .inspection_neighbor_requests
            .saturating_sub(before.inspection_neighbor_requests),
        inspection_historical_view_opens: after
            .inspection_historical_view_opens
            .saturating_sub(before.inspection_historical_view_opens),
        inspection_commit_reads: after
            .inspection_commit_reads
            .saturating_sub(before.inspection_commit_reads),
        live_entity_history_entries_trimmed: after
            .live_entity_history_entries_trimmed
            .saturating_sub(before.live_entity_history_entries_trimmed),
        live_relation_history_entries_trimmed: after
            .live_relation_history_entries_trimmed
            .saturating_sub(before.live_relation_history_entries_trimmed),
        forward_adjacency_updates: after
            .forward_adjacency_updates
            .saturating_sub(before.forward_adjacency_updates),
        reverse_adjacency_updates: after
            .reverse_adjacency_updates
            .saturating_sub(before.reverse_adjacency_updates),
        ..RuntimeComplexityCounters::default()
    }
}
