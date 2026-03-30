use crate::authority::commit::phases::artifacts::prepare_publication_artifacts;
use crate::authority::commit::phases::finalize::{
    finalize_commit_publication, FinalizeCommitInput,
};
use crate::authority::commit::phases::history::{
    resolve_commit_history, resolve_commit_history_for_merge,
};
use crate::authority::commit::phases::mutation::run_authoritative_mutation_for_runtime;
use crate::authority::commit::phases::prepare::{
    prepare_authoritative_working_state_scope, prepare_working_state_scope,
    record_preparation_counters,
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
    MergeCommitMutationPlan, MergedCommitPlan, ProvenanceCompleteBulkMutationBatch,
    TransactionCommitError, TransactionId, TransactionOptions,
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
        let admitted_bulk_batch = self
            .admit_provenance_complete_bulk_mutation_batch()
            .map_err(TransactionCommitError::conflict)?;
        let mut draft_preparation_log = CommitLog::new();
        draft_preparation_log.begin_phase(CommitPhase::DraftPreparation);
        let prepared = prepare_working_state_scope(&mut self).map_err(|error| {
            attach_rejection(
                &mut draft_preparation_log,
                CommitPhase::DraftPreparation,
                error,
            )
        })?;
        execute_authoritative_commit(
            self.runtime,
            AuthoritativeCommitContext::from_mutation(
                self.transaction_id,
                self.options,
                prepared,
                admitted_bulk_batch,
            ),
        )
    }
}

#[derive(Debug)]
pub(crate) enum CommitAuthorityInput {
    Mutation(MergedCommitPlan),
    Merge(MergeCommitMutationPlan),
}

#[derive(Debug)]
pub(crate) struct AuthoritativeCommitContext {
    transaction_id: TransactionId,
    options: TransactionOptions,
    authority_input: CommitAuthorityInput,
    prepared_scope: Option<PreparedAuthorityScope>,
    merge_execution_accounting: Option<MergeExecutionAccounting>,
    bulk_mutation_batch: Option<ProvenanceCompleteBulkMutationBatch>,
}

#[derive(Debug)]
struct PreparedAuthorityScope {
    structural_summary: crate::authority::commit::structural_summary::CommitStructuralSummary,
    working_state: crate::storage::overlay::WorkingState,
}

#[derive(Debug, Clone, Copy)]
struct MergeExecutionAccounting {
    admitted_records: usize,
    emitted_mutation_intents: usize,
}

impl AuthoritativeCommitContext {
    pub(crate) fn from_mutation(
        transaction_id: TransactionId,
        options: TransactionOptions,
        prepared: crate::authority::commit::phases::prepare::PreparedWorkingStateScope,
        bulk_mutation_batch: Option<ProvenanceCompleteBulkMutationBatch>,
    ) -> Self {
        Self {
            transaction_id,
            options,
            authority_input: CommitAuthorityInput::Mutation(prepared.merged_plan),
            prepared_scope: Some(PreparedAuthorityScope {
                structural_summary: prepared.structural_summary,
                working_state: prepared.working_state,
            }),
            merge_execution_accounting: None,
            bulk_mutation_batch,
        }
    }

    pub(crate) fn from_merge(
        options: TransactionOptions,
        merge_plan: MergeCommitMutationPlan,
    ) -> Result<Self, TransactionCommitError> {
        if options.target_branch.as_ref() != Some(&merge_plan.target_branch) {
            return Err(invalid_merge_context(
                "merge commit context target branch does not match merge proof",
            ));
        }
        if options.merge_parent_branches != merge_plan.merge_parent_branches.as_ref() {
            return Err(invalid_merge_context(
                "merge commit context merge parent branches do not match merge proof",
            ));
        }
        if merge_plan.requested_merge_parent_count != merge_plan.merge_parent_branches.len() {
            return Err(invalid_merge_context(
                "merge commit context requested merge parent count does not match merge proof",
            ));
        }
        let expected_parents = [
            merge_plan.merge_execution_summary.target_head_commit_id,
            merge_plan.merge_execution_summary.source_head_commit_id,
        ];
        if merge_plan.parent_commits.as_slice() != expected_parents {
            return Err(invalid_merge_context(
                "merge commit context ordered parent commits do not match merge proof",
            ));
        }

        Ok(Self {
            transaction_id: merge_plan.transaction_id,
            options,
            merge_execution_accounting: Some(MergeExecutionAccounting {
                admitted_records: merge_plan.structural_summary.executed_record_count,
                emitted_mutation_intents: merge_plan.structural_summary.emitted_mutation_intent_count,
            }),
            authority_input: CommitAuthorityInput::Merge(merge_plan),
            prepared_scope: None,
            bulk_mutation_batch: None,
        })
    }
}

fn invalid_merge_context(detail: &str) -> TransactionCommitError {
    TransactionCommitError::conflict(crate::transactions::data::CommitConflict::new(
        crate::transactions::data::ConflictClass::InvalidMergeParent {
            detail: detail.to_string(),
        },
    ))
}

pub(crate) fn execute_authoritative_commit(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    context: AuthoritativeCommitContext,
) -> Result<CommitResult, TransactionCommitError> {
    let AuthoritativeCommitContext {
        transaction_id,
        options,
        authority_input,
        prepared_scope,
        merge_execution_accounting,
        bulk_mutation_batch,
    } = context;
    let mut commit_log = CommitLog::new();
    let mut phase_timing = CommitPhaseTiming::default();
    let diagnostics_start = runtime
        .publication_access()
        .diagnostics()
        .artifacts()
        .len();
    let complexity_before = runtime
        .services
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned")
        .clone();
    if let Some(admitted_bulk_batch) = bulk_mutation_batch.as_ref() {
        let planned = admitted_bulk_batch.planned();
        runtime.performance_access().count_bulk_mutation_plan(
            &planned.locality,
            planned.naming.normalized_client_keys.len(),
            planned.lineage.transitions.len(),
            planned.provenance.worker_batch_names.len(),
        );
    }
    let mut invariant_executions = Vec::new();
    let (mutation_plan, merge_history_plan) = match authority_input {
        CommitAuthorityInput::Mutation(plan) => (Some(plan), None),
        CommitAuthorityInput::Merge(plan) => (None, Some(plan)),
    };
    let merged_plan = match (&mutation_plan, &merge_history_plan) {
        (Some(plan), None) => plan,
        (None, Some(plan)) => &plan.merged_plan,
        _ => unreachable!("authoritative commit context must carry exactly one authority input"),
    };
    let merge_parent_count = merge_history_plan
        .as_ref()
        .map(|plan| plan.requested_merge_parent_count)
        .unwrap_or(options.merge_parent_branches.len());
    let PreparedAuthorityScope {
        structural_summary,
        mut working_state,
    } = match prepared_scope {
        Some(scope) => scope,
        None => {
            let (structural_summary, working_state) = prepare_authoritative_working_state_scope(
                runtime,
                merged_plan,
                merge_parent_count,
            );
            PreparedAuthorityScope {
                structural_summary,
                working_state,
            }
        }
    };
    let public_structural_summary = structural_summary.public_summary(
        runtime
            .config
            .schema
            .descriptor_semantics_policy
            .current_write_version(),
    );
    commit_log.begin_phase(CommitPhase::DraftPreparation);
    let phase_started = Instant::now();
    record_preparation_counters(runtime, &working_state, &structural_summary);
    commit_log.record_structural_summary(&public_structural_summary);
    commit_log.complete_phase(CommitPhase::DraftPreparation);
    phase_timing.working_state_preparation_micros = elapsed_micros(phase_started);

    commit_log.begin_phase(CommitPhase::InvariantPreCheck);
    let phase_started = Instant::now();
    let pre_commit_invariants = runtime
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
    let mutation = run_authoritative_mutation_for_runtime(
        runtime,
        transaction_id,
        &mut working_state,
        &merged_plan,
        options.target_branch.as_ref(),
    )
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
    let history = match &merge_history_plan {
        None => {
            let mut transaction = RelationalTransaction {
                runtime,
                transaction_id,
                options: options.clone(),
                batches: Vec::new(),
                savepoints: Vec::new(),
                last_merged_plan: None,
            };
            resolve_commit_history(&mut transaction, version_id)
        }
        Some(plan) => {
            resolve_commit_history_for_merge(runtime, &options, plan, version_id)
        }
    }
    .map_err(|error| attach_rejection(&mut commit_log, CommitPhase::HistoryResolution, error))?;
    let commit_id = history.commit_id;
    let branch_id = history.branch_id.clone();
    let commit_reference = history.commit_reference.clone();
    let history_summary = history.summary();
    let merge_base_commits = history.merge_base_commits;
    let merge_parent_branches = options.merge_parent_branches.clone();
    commit_log.record_history_resolution(&history_summary);
    commit_log.complete_phase(CommitPhase::HistoryResolution);
    phase_timing.history_resolution_micros = elapsed_micros(phase_started);
    let additional_diagnostics_entries = merge_history_plan
        .as_ref()
        .map(|plan| {
            vec![crate::merge::logic::merge_execution_summary_entry(
                &plan.merge_execution_summary,
                &plan.structural_summary,
                commit_id,
            )]
        })
        .unwrap_or_default();

    let schema_continuity = resolve_schema_continuity(runtime, &branch_id, &options)
        .map_err(|error| attach_rejection(&mut commit_log, CommitPhase::ArtifactAssembly, error))?;
    emit_schema_continuity_diagnostic(runtime, &branch_id, &schema_continuity);

    {
        commit_log.begin_phase(CommitPhase::InvariantPostCheck);
        let phase_started = Instant::now();
        let post_invariants = runtime
            .invariant_authority()
            .enforce_snapshot_publication_for_working_state(&working_state, version_id, &merged_plan)
            .map_err(TransactionCommitError::publication)
            .map_err(|error| attach_rejection(&mut commit_log, CommitPhase::InvariantPostCheck, error))?;
        commit_log.record_invariant_outcomes(&post_invariants);
        invariant_executions.push(post_invariants);
        commit_log.complete_phase(CommitPhase::InvariantPostCheck);
        phase_timing.invariant_post_check_micros = elapsed_micros(phase_started);
    }

    commit_log.begin_phase(CommitPhase::ArtifactAssembly);
    let phase_started = Instant::now();
    let patch_records = std::mem::take(&mut effect.publication.patch_records);
    let patch = assemble_patch(runtime, commit_reference.commit_id, patch_records);
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
            runtime,
            crate::diagnostics::data::DiagnosticsScope::PatchPublication,
            failure_class,
            serde_json::json!({
                "failure_class": label,
                "commit_id": commit_reference.commit_id.0,
                "patch_record_count": patch.records.len(),
            }),
        )
    }
    let patch_budget_summary = CommitPatchBudgetSummary {
        patch_record_count: patch.records.len(),
        max_patch_records_per_commit: runtime.config.publication.policy.max_patch_records_per_commit,
    };
    commit_log.record_patch_budget(&patch_budget_summary);
    enforce_patch_budget(runtime, &patch)
        .map_err(|error| attach_rejection(&mut commit_log, CommitPhase::ArtifactAssembly, error))?;
    let publication = prepare_publication_artifacts(
        runtime,
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
        additional_diagnostics_entries,
    )
    .map_err(|error| attach_rejection(&mut commit_log, CommitPhase::ArtifactAssembly, error))?;
    let change_summary = publication.change_summary.clone();
    let aspect_summary = publication.aspect_summary.clone();
    let aspect_evaluation_traces = publication.aspect_evaluation_traces.clone();
    let aspect_emission_traces = publication.aspect_emission_traces.clone();
    let publication_summary = publication.summary.clone();
    let publication_snapshot = publication.finalize.artifacts.bundle.snapshot.clone();
    if runtime.config.diagnostics.profile.detailed_traces_enabled {
        for trace in &aspect_evaluation_traces {
            runtime
                .publication_authority()
                .push_diagnostic_artifact(trace.diagnostic_artifact());
        }
        for trace in &aspect_emission_traces {
            runtime
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
        runtime,
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
        runtime,
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

    if let Some(accounting) = merge_execution_accounting {
        runtime.performance_access().count_merge_execution_request(
            accounting.admitted_records,
            accounting.emitted_mutation_intents,
        );
    }

    let complexity_after = runtime
        .services
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned")
        .clone();
    let diagnostics = runtime.publication_access().diagnostics_since(diagnostics_start);
    let commit_summary = commit_log.summary().clone();
    let schema_summary = CommitSchemaSummary {
        transition: canonical_commit_envelope
            .schema_transition
            .as_ref()
            .map(SchemaTransitionSummary::from_artifact),
        descriptor_semantics_version: canonical_commit_envelope.descriptor_semantics_version,
    };
    let commit_outcome = CommitOutcome {
        transaction_id,
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
        query_packet_count: after
            .query_packet_count
            .saturating_sub(before.query_packet_count),
        query_packet_item_count: after
            .query_packet_item_count
            .saturating_sub(before.query_packet_item_count),
        query_parallel_legal_count: after
            .query_parallel_legal_count
            .saturating_sub(before.query_parallel_legal_count),
        query_parallel_profitable_count: after
            .query_parallel_profitable_count
            .saturating_sub(before.query_parallel_profitable_count),
        query_serial_strategy_count: after
            .query_serial_strategy_count
            .saturating_sub(before.query_serial_strategy_count),
        query_staged_parallel_strategy_count: after
            .query_staged_parallel_strategy_count
            .saturating_sub(before.query_staged_parallel_strategy_count),
        query_index_attempt_count: after
            .query_index_attempt_count
            .saturating_sub(before.query_index_attempt_count),
        query_index_path_count: after
            .query_index_path_count
            .saturating_sub(before.query_index_path_count),
        query_index_rejection_count: after
            .query_index_rejection_count
            .saturating_sub(before.query_index_rejection_count),
        query_index_parity_verification_count: after
            .query_index_parity_verification_count
            .saturating_sub(before.query_index_parity_verification_count),
        query_entity_records_emitted: after
            .query_entity_records_emitted
            .saturating_sub(before.query_entity_records_emitted),
        query_relation_records_emitted: after
            .query_relation_records_emitted
            .saturating_sub(before.query_relation_records_emitted),
        bulk_mutation_batch_count: after
            .bulk_mutation_batch_count
            .saturating_sub(before.bulk_mutation_batch_count),
        bulk_mutation_entity_target_count: after
            .bulk_mutation_entity_target_count
            .saturating_sub(before.bulk_mutation_entity_target_count),
        bulk_mutation_relation_target_count: after
            .bulk_mutation_relation_target_count
            .saturating_sub(before.bulk_mutation_relation_target_count),
        bulk_mutation_cross_partition_relation_count: after
            .bulk_mutation_cross_partition_relation_count
            .saturating_sub(before.bulk_mutation_cross_partition_relation_count),
        bulk_mutation_naming_normalization_count: after
            .bulk_mutation_naming_normalization_count
            .saturating_sub(before.bulk_mutation_naming_normalization_count),
        bulk_mutation_lineage_transition_count: after
            .bulk_mutation_lineage_transition_count
            .saturating_sub(before.bulk_mutation_lineage_transition_count),
        bulk_mutation_provenance_record_count: after
            .bulk_mutation_provenance_record_count
            .saturating_sub(before.bulk_mutation_provenance_record_count),
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
        custom_invariant_preparation_count: after
            .custom_invariant_preparation_count
            .saturating_sub(before.custom_invariant_preparation_count),
        custom_invariant_execution_count: after
            .custom_invariant_execution_count
            .saturating_sub(before.custom_invariant_execution_count),
        custom_invariant_panic_count: after
            .custom_invariant_panic_count
            .saturating_sub(before.custom_invariant_panic_count),
        custom_invariant_traversal_frontier_count: after
            .custom_invariant_traversal_frontier_count
            .saturating_sub(before.custom_invariant_traversal_frontier_count),
        custom_invariant_traversal_step_count: after
            .custom_invariant_traversal_step_count
            .saturating_sub(before.custom_invariant_traversal_step_count),
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
        inspection_connectivity_entity_scans: after
            .inspection_connectivity_entity_scans
            .saturating_sub(before.inspection_connectivity_entity_scans),
        inspection_connectivity_relation_scans: after
            .inspection_connectivity_relation_scans
            .saturating_sub(before.inspection_connectivity_relation_scans),
        inspection_connectivity_frontier_expansions: after
            .inspection_connectivity_frontier_expansions
            .saturating_sub(before.inspection_connectivity_frontier_expansions),
        inspection_connectivity_components_evaluated: after
            .inspection_connectivity_components_evaluated
            .saturating_sub(before.inspection_connectivity_components_evaluated),
        inspection_budget_refusals: after
            .inspection_budget_refusals
            .saturating_sub(before.inspection_budget_refusals),
        inspection_retention_entity_slot_scans: after
            .inspection_retention_entity_slot_scans
            .saturating_sub(before.inspection_retention_entity_slot_scans),
        inspection_retention_relation_slot_scans: after
            .inspection_retention_relation_slot_scans
            .saturating_sub(before.inspection_retention_relation_slot_scans),
        relation_cardinality_minimum_certification_contracts_evaluated: after
            .relation_cardinality_minimum_certification_contracts_evaluated
            .saturating_sub(before.relation_cardinality_minimum_certification_contracts_evaluated),
        relation_cardinality_minimum_certification_entity_slot_scans: after
            .relation_cardinality_minimum_certification_entity_slot_scans
            .saturating_sub(before.relation_cardinality_minimum_certification_entity_slot_scans),
        relation_cardinality_minimum_certification_relation_slot_scans: after
            .relation_cardinality_minimum_certification_relation_slot_scans
            .saturating_sub(before.relation_cardinality_minimum_certification_relation_slot_scans),
        lineage_recorded_candidate_width: after
            .lineage_recorded_candidate_width
            .saturating_sub(before.lineage_recorded_candidate_width),
        lineage_validated_candidate_width: after
            .lineage_validated_candidate_width
            .saturating_sub(before.lineage_validated_candidate_width),
        lineage_promotion_eligible_candidate_width: after
            .lineage_promotion_eligible_candidate_width
            .saturating_sub(before.lineage_promotion_eligible_candidate_width),
        lineage_promotion_rejection_count: after
            .lineage_promotion_rejection_count
            .saturating_sub(before.lineage_promotion_rejection_count),
        lineage_promotion_accepted_count: after
            .lineage_promotion_accepted_count
            .saturating_sub(before.lineage_promotion_accepted_count),
        lineage_finalization_event_batch_width: after
            .lineage_finalization_event_batch_width
            .saturating_sub(before.lineage_finalization_event_batch_width),
        lineage_finalization_decision_log_width: after
            .lineage_finalization_decision_log_width
            .saturating_sub(before.lineage_finalization_decision_log_width),
        lineage_historical_resolution_requests: after
            .lineage_historical_resolution_requests
            .saturating_sub(before.lineage_historical_resolution_requests),
        lineage_historical_resolution_branch_event_scans: after
            .lineage_historical_resolution_branch_event_scans
            .saturating_sub(before.lineage_historical_resolution_branch_event_scans),
        lineage_historical_resolution_traversed_events: after
            .lineage_historical_resolution_traversed_events
            .saturating_sub(before.lineage_historical_resolution_traversed_events),
        lineage_branch_divergence_requests: after
            .lineage_branch_divergence_requests
            .saturating_sub(before.lineage_branch_divergence_requests),
        lineage_branch_divergence_event_scans: after
            .lineage_branch_divergence_event_scans
            .saturating_sub(before.lineage_branch_divergence_event_scans),
        lineage_graph_snapshot_requests: after
            .lineage_graph_snapshot_requests
            .saturating_sub(before.lineage_graph_snapshot_requests),
        lineage_graph_snapshot_nodes_materialized: after
            .lineage_graph_snapshot_nodes_materialized
            .saturating_sub(before.lineage_graph_snapshot_nodes_materialized),
        lineage_graph_snapshot_events_materialized: after
            .lineage_graph_snapshot_events_materialized
            .saturating_sub(before.lineage_graph_snapshot_events_materialized),
        lineage_graph_snapshot_candidates_materialized: after
            .lineage_graph_snapshot_candidates_materialized
            .saturating_sub(before.lineage_graph_snapshot_candidates_materialized),
        lineage_branch_divergence_node_scans: after
            .lineage_branch_divergence_node_scans
            .saturating_sub(before.lineage_branch_divergence_node_scans),
        lineage_graph_snapshot_visibility_cache_hits: after
            .lineage_graph_snapshot_visibility_cache_hits
            .saturating_sub(before.lineage_graph_snapshot_visibility_cache_hits),
        lineage_graph_snapshot_visibility_cache_miss_reconstructions: after
            .lineage_graph_snapshot_visibility_cache_miss_reconstructions
            .saturating_sub(before.lineage_graph_snapshot_visibility_cache_miss_reconstructions),
        lineage_publication_event_width: after
            .lineage_publication_event_width
            .saturating_sub(before.lineage_publication_event_width),
        lineage_publication_decision_width: after
            .lineage_publication_decision_width
            .saturating_sub(before.lineage_publication_decision_width),
        schema_transition_atoms_inspected: after
            .schema_transition_atoms_inspected
            .saturating_sub(before.schema_transition_atoms_inspected),
        schema_changed_subtrees_inspected: after
            .schema_changed_subtrees_inspected
            .saturating_sub(before.schema_changed_subtrees_inspected),
        schema_unchanged_subtrees_reused_by_fingerprint: after
            .schema_unchanged_subtrees_reused_by_fingerprint
            .saturating_sub(before.schema_unchanged_subtrees_reused_by_fingerprint),
        schema_bridge_descriptors_built: after
            .schema_bridge_descriptors_built
            .saturating_sub(before.schema_bridge_descriptors_built),
        schema_normalized_descriptor_compositions: after
            .schema_normalized_descriptor_compositions
            .saturating_sub(before.schema_normalized_descriptor_compositions),
        schema_transition_continue_unchanged_count: after
            .schema_transition_continue_unchanged_count
            .saturating_sub(before.schema_transition_continue_unchanged_count),
        schema_transition_continue_transparent_bridge_count: after
            .schema_transition_continue_transparent_bridge_count
            .saturating_sub(before.schema_transition_continue_transparent_bridge_count),
        schema_transition_continue_visible_bridge_count: after
            .schema_transition_continue_visible_bridge_count
            .saturating_sub(before.schema_transition_continue_visible_bridge_count),
        schema_transition_continue_contract_upgrade_count: after
            .schema_transition_continue_contract_upgrade_count
            .saturating_sub(before.schema_transition_continue_contract_upgrade_count),
        schema_transition_require_renegotiation_count: after
            .schema_transition_require_renegotiation_count
            .saturating_sub(before.schema_transition_require_renegotiation_count),
        schema_transition_rejected_count: after
            .schema_transition_rejected_count
            .saturating_sub(before.schema_transition_rejected_count),
        schema_historical_interpretation_sensitive_boundaries: after
            .schema_historical_interpretation_sensitive_boundaries
            .saturating_sub(before.schema_historical_interpretation_sensitive_boundaries),
        schema_reconciliation_reject_lossy_narrowing_count: after
            .schema_reconciliation_reject_lossy_narrowing_count
            .saturating_sub(before.schema_reconciliation_reject_lossy_narrowing_count),
        schema_reconciliation_preserve_information_count: after
            .schema_reconciliation_preserve_information_count
            .saturating_sub(before.schema_reconciliation_preserve_information_count),
        schema_reconciliation_preserve_target_contract_count: after
            .schema_reconciliation_preserve_target_contract_count
            .saturating_sub(before.schema_reconciliation_preserve_target_contract_count),
        schema_reconciliation_preserve_source_contract_count: after
            .schema_reconciliation_preserve_source_contract_count
            .saturating_sub(before.schema_reconciliation_preserve_source_contract_count),
        schema_reconciliation_permit_lossy_narrowing_with_annotation_count: after
            .schema_reconciliation_permit_lossy_narrowing_with_annotation_count
            .saturating_sub(before.schema_reconciliation_permit_lossy_narrowing_with_annotation_count),
        schema_reconciliation_require_explicit_projection_count: after
            .schema_reconciliation_require_explicit_projection_count
            .saturating_sub(before.schema_reconciliation_require_explicit_projection_count),
        subscriber_resume_evaluations: after
            .subscriber_resume_evaluations
            .saturating_sub(before.subscriber_resume_evaluations),
        subscriber_continue_unchanged_count: after
            .subscriber_continue_unchanged_count
            .saturating_sub(before.subscriber_continue_unchanged_count),
        subscriber_continue_transparent_bridge_count: after
            .subscriber_continue_transparent_bridge_count
            .saturating_sub(before.subscriber_continue_transparent_bridge_count),
        subscriber_continue_visible_bridge_count: after
            .subscriber_continue_visible_bridge_count
            .saturating_sub(before.subscriber_continue_visible_bridge_count),
        subscriber_continue_contract_upgrade_count: after
            .subscriber_continue_contract_upgrade_count
            .saturating_sub(before.subscriber_continue_contract_upgrade_count),
        subscriber_require_renegotiation_count: after
            .subscriber_require_renegotiation_count
            .saturating_sub(before.subscriber_require_renegotiation_count),
        subscriber_rejected_count: after
            .subscriber_rejected_count
            .saturating_sub(before.subscriber_rejected_count),
        replay_digest_parity_checks: after
            .replay_digest_parity_checks
            .saturating_sub(before.replay_digest_parity_checks),
        replay_summary_parity_checks: after
            .replay_summary_parity_checks
            .saturating_sub(before.replay_summary_parity_checks),
        replay_deep_artifact_parity_checks: after
            .replay_deep_artifact_parity_checks
            .saturating_sub(before.replay_deep_artifact_parity_checks),
        replay_lineage_authority_lookup_requests: after
            .replay_lineage_authority_lookup_requests
            .saturating_sub(before.replay_lineage_authority_lookup_requests),
        replay_lineage_log_index_hits: after
            .replay_lineage_log_index_hits
            .saturating_sub(before.replay_lineage_log_index_hits),
        replay_lineage_checkpoint_index_hits: after
            .replay_lineage_checkpoint_index_hits
            .saturating_sub(before.replay_lineage_checkpoint_index_hits),
        replay_lineage_durable_basis_selections: after
            .replay_lineage_durable_basis_selections
            .saturating_sub(before.replay_lineage_durable_basis_selections),
        replay_lineage_history_fallback_basis_selections: after
            .replay_lineage_history_fallback_basis_selections
            .saturating_sub(before.replay_lineage_history_fallback_basis_selections),
        replay_lineage_authoritative_basis_rejections: after
            .replay_lineage_authoritative_basis_rejections
            .saturating_sub(before.replay_lineage_authoritative_basis_rejections),
        replay_lineage_digest_event_width: after
            .replay_lineage_digest_event_width
            .saturating_sub(before.replay_lineage_digest_event_width),
        replay_lineage_digest_decision_width: after
            .replay_lineage_digest_decision_width
            .saturating_sub(before.replay_lineage_digest_decision_width),
        merge_history_ancestry_traversals: after
            .merge_history_ancestry_traversals
            .saturating_sub(before.merge_history_ancestry_traversals),
        merge_history_ancestry_nodes_visited: after
            .merge_history_ancestry_nodes_visited
            .saturating_sub(before.merge_history_ancestry_nodes_visited),
        merge_history_parent_comparisons: after
            .merge_history_parent_comparisons
            .saturating_sub(before.merge_history_parent_comparisons),
        merge_history_replay_parent_checks: after
            .merge_history_replay_parent_checks
            .saturating_sub(before.merge_history_replay_parent_checks),
        merge_history_replay_planning_nodes_visited: after
            .merge_history_replay_planning_nodes_visited
            .saturating_sub(before.merge_history_replay_planning_nodes_visited),
        merge_history_durability_parent_checks: after
            .merge_history_durability_parent_checks
            .saturating_sub(before.merge_history_durability_parent_checks),
        merge_history_durability_validation_nodes_visited: after
            .merge_history_durability_validation_nodes_visited
            .saturating_sub(before.merge_history_durability_validation_nodes_visited),
        merge_planning_requests: after
            .merge_planning_requests
            .saturating_sub(before.merge_planning_requests),
        merge_planning_schema_kinds_snapshotted: after
            .merge_planning_schema_kinds_snapshotted
            .saturating_sub(before.merge_planning_schema_kinds_snapshotted),
        merge_planning_target_commits_scoped: after
            .merge_planning_target_commits_scoped
            .saturating_sub(before.merge_planning_target_commits_scoped),
        merge_planning_source_commits_scoped: after
            .merge_planning_source_commits_scoped
            .saturating_sub(before.merge_planning_source_commits_scoped),
        merge_planning_target_records_scoped: after
            .merge_planning_target_records_scoped
            .saturating_sub(before.merge_planning_target_records_scoped),
        merge_planning_source_records_scoped: after
            .merge_planning_source_records_scoped
            .saturating_sub(before.merge_planning_source_records_scoped),
        merge_identity_candidates_discovered: after
            .merge_identity_candidates_discovered
            .saturating_sub(before.merge_identity_candidates_discovered),
        merge_identity_effective_declarations: after
            .merge_identity_effective_declarations
            .saturating_sub(before.merge_identity_effective_declarations),
        merge_identity_target_records_scanned: after
            .merge_identity_target_records_scanned
            .saturating_sub(before.merge_identity_target_records_scanned),
        merge_identity_target_records_indexed: after
            .merge_identity_target_records_indexed
            .saturating_sub(before.merge_identity_target_records_indexed),
        merge_conflict_records_classified: after
            .merge_conflict_records_classified
            .saturating_sub(before.merge_conflict_records_classified),
        merge_topology_relation_candidates_scoped: after
            .merge_topology_relation_candidates_scoped
            .saturating_sub(before.merge_topology_relation_candidates_scoped),
        merge_topology_endpoint_incidences_scoped: after
            .merge_topology_endpoint_incidences_scoped
            .saturating_sub(before.merge_topology_endpoint_incidences_scoped),
        merge_topology_region_conflicts_detected: after
            .merge_topology_region_conflicts_detected
            .saturating_sub(before.merge_topology_region_conflicts_detected),
        merge_topology_region_records_escalated: after
            .merge_topology_region_records_escalated
            .saturating_sub(before.merge_topology_region_records_escalated),
        merge_causal_records_annotated: after
            .merge_causal_records_annotated
            .saturating_sub(before.merge_causal_records_annotated),
        merge_policy_records_resolved: after
            .merge_policy_records_resolved
            .saturating_sub(before.merge_policy_records_resolved),
        merge_lowered_records_emitted: after
            .merge_lowered_records_emitted
            .saturating_sub(before.merge_lowered_records_emitted),
        merge_decision_log_width: after
            .merge_decision_log_width
            .saturating_sub(before.merge_decision_log_width),
        merge_planning_elapsed_nanos: after
            .merge_planning_elapsed_nanos
            .saturating_sub(before.merge_planning_elapsed_nanos),
        merge_execution_verification_requests: after
            .merge_execution_verification_requests
            .saturating_sub(before.merge_execution_verification_requests),
        merge_execution_branch_head_checks: after
            .merge_execution_branch_head_checks
            .saturating_sub(before.merge_execution_branch_head_checks),
        merge_execution_merge_base_checks: after
            .merge_execution_merge_base_checks
            .saturating_sub(before.merge_execution_merge_base_checks),
        merge_execution_schema_kinds_snapshotted: after
            .merge_execution_schema_kinds_snapshotted
            .saturating_sub(before.merge_execution_schema_kinds_snapshotted),
        merge_execution_compiled_plan_digest_checks: after
            .merge_execution_compiled_plan_digest_checks
            .saturating_sub(before.merge_execution_compiled_plan_digest_checks),
        merge_execution_attempts: after
            .merge_execution_attempts
            .saturating_sub(before.merge_execution_attempts),
        merge_execution_requests: after
            .merge_execution_requests
            .saturating_sub(before.merge_execution_requests),
        merge_execution_records_admitted: after
            .merge_execution_records_admitted
            .saturating_sub(before.merge_execution_records_admitted),
        merge_execution_mutation_intents_emitted: after
            .merge_execution_mutation_intents_emitted
            .saturating_sub(before.merge_execution_mutation_intents_emitted),
        descriptor_version_mismatches_encountered: after
            .descriptor_version_mismatches_encountered
            .saturating_sub(before.descriptor_version_mismatches_encountered),
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
    }
}
