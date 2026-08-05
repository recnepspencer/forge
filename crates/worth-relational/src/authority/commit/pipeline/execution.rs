use super::artifact_assembly_phase::{
    assemble_authoritative_publication_phase, ArtifactAssemblyInput,
};
use super::authority_context::{
    AuthoritativeCommitContext, CommitAuthorityInput, PreparedAuthorityScope,
};
use super::bulk_mutation_telemetry::record_bulk_mutation_telemetry;
use super::complexity_delta::complexity_delta;
use super::draft_preparation_phase::record_draft_preparation_phase;
use super::history_resolution_phase::resolve_authoritative_history_phase;
use super::invariant_phase::{enforce_commit_boundary_phase, enforce_snapshot_publication_phase};
use super::mutation_phase::run_authoritative_mutation_phase;
use super::publication_phase::{
    append_durable_commit_phase, finalize_publication_phase, FinalizedPublicationPhase,
};
use super::rejection::{attach_rejection, stale_strategy_validation_basis};
use crate::authority::commit::phases::prepare::prepare_authoritative_working_state_scope;
use crate::authority::commit::phases::schema_continuity::{
    emit_schema_continuity_diagnostic, resolve_schema_continuity,
};
use crate::publication::bundle::PublicationStatus;
use crate::schema::data::SchemaTransitionSummary;
use crate::transactions::data::{
    CommitExecution, CommitLog, CommitOutcome, CommitPhase, CommitPublication, CommitResult,
    CommitSchemaSummary, CommitValidation, PublishedMergeExecutionAuthority,
    TransactionCommitError,
};

pub(crate) fn execute_authoritative_commit(
    runtime: &mut crate::logic::runtime::RelationalRuntime,
    context: AuthoritativeCommitContext,
) -> Result<CommitResult, TransactionCommitError> {
    let AuthoritativeCommitContext {
        transaction_id,
        options,
        mut phase_timing,
        authority_input,
        prepared_scope,
        merge_execution_accounting,
        bulk_mutation_telemetry,
        prevalidated_commit_boundary,
        validated_against_commit_id,
        validated_against_version_id,
        strategy_commit_artifacts,
    } = context;
    let mut commit_log = CommitLog::new();
    let diagnostics_start = runtime.publication().diagnostic_access().artifact_count();
    let complexity_before = runtime
        .services
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned")
        .clone();
    if let Some(telemetry) = bulk_mutation_telemetry.as_ref() {
        record_bulk_mutation_telemetry(runtime, telemetry);
    }
    enforce_validated_strategy_basis(
        runtime,
        &options,
        validated_against_commit_id,
        validated_against_version_id,
    )?;

    let mut invariant_executions = Vec::new();
    let (mutation_plan, merge_history_plan) = match authority_input {
        CommitAuthorityInput::Lowered(plan) => (Some(plan), None),
        CommitAuthorityInput::Merge(plan) => (None, Some(plan)),
    };
    let merged_plan = match (&mutation_plan, &merge_history_plan) {
        (Some(plan), None) => plan.merged_plan(),
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
        phase_timing: prepared_phase_timing,
    } = prepared_scope.unwrap_or_else(|| {
        let (structural_summary, working_state, phase_timing) =
            prepare_authoritative_working_state_scope(runtime, merged_plan, merge_parent_count);
        PreparedAuthorityScope {
            structural_summary,
            working_state,
            phase_timing,
        }
    });
    phase_timing.draft_structural_summary_micros =
        prepared_phase_timing.draft_structural_summary_micros;
    phase_timing.draft_working_state_clone_micros =
        prepared_phase_timing.draft_working_state_clone_micros;
    let public_structural_summary = structural_summary.public_summary(
        runtime
            .config
            .schema
            .descriptor_semantics_policy
            .current_write_version(),
    );
    record_draft_preparation_phase(
        runtime,
        &mut commit_log,
        &mut phase_timing,
        &working_state,
        &structural_summary,
        &public_structural_summary,
    );

    invariant_executions.push(enforce_commit_boundary_phase(
        runtime,
        &mut commit_log,
        &mut phase_timing,
        merged_plan,
        prevalidated_commit_boundary,
    )?);
    let mutation = run_authoritative_mutation_phase(
        runtime,
        &mut commit_log,
        &mut phase_timing,
        transaction_id,
        &mut working_state,
        merged_plan,
        options.target_branch.as_ref(),
    )?;
    let version_id = mutation.version_id;
    let effect = mutation.effect;
    invariant_executions.push(mutation.invariant_results);

    let history = resolve_authoritative_history_phase(
        runtime,
        &mut commit_log,
        &mut phase_timing,
        transaction_id,
        &options,
        version_id,
        merge_history_plan.as_ref(),
    )?;
    let commit_id = history.commit_id;
    let branch_id = history.branch_id.clone();
    let commit_reference = history.commit_reference.clone();
    let merge_base_commits = history.merge_base_commits;
    let merge_parent_branches = options.merge_parent_branches.clone();

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
    let merge_execution_authority = merge_history_plan
        .as_ref()
        .map(PublishedMergeExecutionAuthority::from_merge_plan);

    let schema_continuity = resolve_schema_continuity(runtime, &branch_id, &options)
        .map_err(|error| attach_rejection(&mut commit_log, CommitPhase::ArtifactAssembly, error))?;
    emit_schema_continuity_diagnostic(runtime, &branch_id, &schema_continuity);

    invariant_executions.push(enforce_snapshot_publication_phase(
        runtime,
        &mut commit_log,
        &mut phase_timing,
        &working_state,
        version_id,
        merged_plan,
    )?);

    let publication = assemble_authoritative_publication_phase(
        runtime,
        &mut commit_log,
        &mut phase_timing,
        ArtifactAssemblyInput {
            working_state: &mut working_state,
            effect,
            commit_reference: &commit_reference,
            branch_id: &branch_id,
            version_id,
            merge_parent_branches: &merge_parent_branches,
            merge_base_commits: &merge_base_commits,
            merged_plan,
            strategy_commit_artifacts: strategy_commit_artifacts.clone(),
            merge_execution_authority,
            schema_continuity: &schema_continuity,
            additional_diagnostics_entries,
        },
    )?;
    let aspect_evaluation_traces = publication.aspect_evaluation_traces.clone();
    let aspect_emission_traces = publication.aspect_emission_traces.clone();
    let publication_snapshot = publication.finalize.artifacts.bundle.snapshot.clone();

    append_durable_commit_phase(
        runtime,
        &mut commit_log,
        &mut phase_timing,
        &publication,
        commit_id,
        &branch_id,
    )?;
    let FinalizedPublicationPhase {
        canonical_commit_envelope,
        changed_records,
    } = finalize_publication_phase(
        runtime,
        &mut commit_log,
        &mut phase_timing,
        working_state,
        publication,
        version_id,
        history.previous_branch_head_version,
        commit_id,
        &commit_reference,
        &branch_id,
        &merge_base_commits,
        &merge_parent_branches,
    );

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
    let diagnostics = runtime
        .publication()
        .diagnostic_access()
        .artifacts_since(diagnostics_start);
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
            strategy_artifacts: strategy_commit_artifacts,
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

fn enforce_validated_strategy_basis(
    runtime: &crate::logic::runtime::RelationalRuntime,
    options: &crate::transactions::data::TransactionOptions,
    validated_against_commit_id: Option<crate::history::data::CommitId>,
    validated_against_version_id: Option<crate::identity::data::VersionId>,
) -> Result<(), TransactionCommitError> {
    let validation_basis_branch = options
        .target_branch
        .clone()
        .unwrap_or_else(|| runtime.config.history.main_branch.clone());
    let validation_basis_branch_head = runtime
        .history()
        .branch_head(&validation_basis_branch)
        .cloned();
    if let Some(validated_version_id) = validated_against_version_id {
        let observed_version_id = validation_basis_branch_head
            .as_ref()
            .map(|commit| commit.version_id)
            .unwrap_or_else(|| runtime.current_version_id());
        if validated_version_id != observed_version_id {
            return Err(stale_strategy_validation_basis(
                "validated strategy plan no longer matches the current committed version basis",
            ));
        }
    }
    if let Some(validated_commit_id) = validated_against_commit_id {
        if validation_basis_branch_head
            .as_ref()
            .map(|commit| commit.commit_id)
            != Some(validated_commit_id)
        {
            return Err(stale_strategy_validation_basis(
                "validated strategy plan no longer matches the current committed commit basis",
            ));
        }
    }
    Ok(())
}
