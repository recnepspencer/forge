use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::CanonicalDependencies;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::host_computed::{admit_or_error, HostComputedApiFamily};
use crate::data::node::{AuthorityPolicy, PathClass};
use crate::data::performance::{ResolvedExecutionStrategy, ResolvedMaintenanceStrategy};
use crate::logic::planner::precompute::{PreparedTaskPatch, StageExecutionData};
use crate::logic::planner::semantic::StageSemanticIdentity;
use crate::logic::planner::types::{
    EligibleTask, LoweredApplyPlan, LoweredStagePlan, LoweredTask, LoweredTaskExecution,
    StageExecutor,
};
use crate::logic::prepared::{PreparedEvaluationOrigin, PreparedEvaluationOutcome};

use super::super::serial_batch::LoweredSerialStage;
use super::footprint::{
    build_apply_footprint, build_lowered_dirty_delta, build_touched_scope_summary, structural_delta,
};
use super::strategy::build_lowered_apply_plan;

pub(super) enum LoweredStageExecutionForm {
    Serial(LoweredSerialStage),
    Generic(LoweredStagePlan),
}

pub(super) fn build_stage_execution_form(
    graph: &mut SignalGraph,
    stage_index: u32,
    stage_tasks: &[EligibleTask],
    stage_execution: StageExecutionData,
    comparator_resolver: &impl ComparatorPolicyResolver,
    executor: StageExecutor,
    stage_identities: &[StageSemanticIdentity],
) -> Result<LoweredStageExecutionForm, SignalError> {
    let prepared_patches = stage_execution.into_patches(stage_tasks);
    let resolved_policy = graph.resolved_performance_policy();

    if should_lower_direct_serial(executor) {
        return Ok(LoweredStageExecutionForm::Serial(
            LoweredSerialStage::from_prepared_patches(
                graph,
                stage_index,
                stage_tasks,
                prepared_patches,
                resolved_policy.maintenance_strategy,
                resolved_policy.authority_policy,
                stage_identities,
            )?,
        ));
    }

    let lowered_tasks = prepared_patches
        .into_iter()
        .map(|patch| lower_task_patch(graph, patch, comparator_resolver))
        .collect::<Result<Vec<_>, SignalError>>()?;
    let lowered_apply_plan = build_lowered_apply_plan(graph, stage_index, &lowered_tasks, executor);
    let dirty_delta = build_lowered_dirty_delta(&lowered_tasks);
    let touched_scope = build_touched_scope_summary(&lowered_tasks);
    let authority_policy = lowered_tasks
        .iter()
        .find(|task| matches!(task.authority_policy(), AuthorityPolicy::AuthoritativeOnly))
        .map(|task| task.authority_policy())
        .unwrap_or(resolved_policy.authority_policy);
    let lowered_stage = LoweredStagePlan::new(
        stage_index,
        lowered_tasks,
        lowered_apply_plan,
        structural_delta(dirty_delta, touched_scope),
        resolved_policy.execution_strategy,
        resolved_policy.maintenance_strategy,
        authority_policy,
    );

    if matches!(
        lowered_stage.lowered_apply_plan(),
        LoweredApplyPlan::Serial(_)
    ) {
        let (
            stage_index,
            tasks,
            lowered_apply_plan,
            dirty_delta,
            _execution_strategy,
            _maintenance_strategy,
            authority_policy,
        ) = lowered_stage.into_parts();
        let LoweredApplyPlan::Serial(plan) = lowered_apply_plan else {
            unreachable!("checked above")
        };
        #[cfg(not(feature = "parallel"))]
        let _ = plan;
        return Ok(LoweredStageExecutionForm::Serial(
            LoweredSerialStage::from_lowered_tasks(
                stage_index,
                stage_tasks,
                authority_policy,
                dirty_delta,
                resolved_policy.maintenance_strategy,
                #[cfg(feature = "parallel")]
                plan.rejection_reason,
                tasks,
                stage_identities,
            ),
        ));
    }

    Ok(LoweredStageExecutionForm::Generic(lowered_stage))
}

#[cfg(feature = "parallel")]
fn should_lower_direct_serial(executor: StageExecutor) -> bool {
    !executor.is_full_parallel()
}

#[cfg(not(feature = "parallel"))]
fn should_lower_direct_serial(_executor: StageExecutor) -> bool {
    true
}

pub(super) fn validate_lowered_stage_plan(lowered: &LoweredStagePlan) {
    let rich_task_count = lowered
        .tasks()
        .iter()
        .filter(|task| matches!(task.path_class(), PathClass::Rich))
        .count();
    let authoritative_task_count = lowered
        .tasks()
        .iter()
        .filter(|task| matches!(task.authority_policy(), AuthorityPolicy::AuthoritativeOnly))
        .count();
    let recomputed_task_count = lowered
        .tasks()
        .iter()
        .filter(|task| task.execution().recomputed())
        .count();

    debug_assert!(
        lowered.task_count() == lowered.tasks().len(),
        "lowered task count must match staged task collection"
    );
    debug_assert!(
        lowered.dirty_delta().is_empty() || !lowered.tasks().is_empty(),
        "structural delta should only be populated for non-empty lowered stages"
    );
    debug_assert!(
        !matches!(
            lowered.execution_strategy(),
            ResolvedExecutionStrategy::FullGraphPass
        ) || lowered.tasks().is_empty()
            || !lowered.apply_groups().is_empty(),
        "full-graph execution stages must still lower into apply groups"
    );
    debug_assert!(
        !matches!(
            lowered.maintenance_strategy(),
            ResolvedMaintenanceStrategy::Rebuild
        ) || lowered.dirty_delta().dirty.is_some(),
        "rebuild-oriented stages must carry a narrowed dirty delta"
    );
    debug_assert!(
        !matches!(
            lowered.authority_policy(),
            AuthorityPolicy::AuthoritativeOnly
        ) || authoritative_task_count > 0,
        "authoritative lowered stages must include authoritative tasks"
    );
    debug_assert!(
        rich_task_count <= lowered.tasks().len(),
        "rich-path accounting must remain bounded by lowered tasks"
    );
    debug_assert!(
        recomputed_task_count <= lowered.tasks().len(),
        "recomputed-task accounting must remain bounded by lowered tasks"
    );
}

fn lower_task_patch(
    graph: &mut SignalGraph,
    patch: PreparedTaskPatch,
    comparator_resolver: &impl ComparatorPolicyResolver,
) -> Result<LoweredTask, SignalError> {
    #[cfg(not(feature = "parallel"))]
    let _ = comparator_resolver;
    graph.refresh_runtime_dependencies_of(patch.node)?;
    let current_dependencies =
        CanonicalDependencies::from_slice(graph.current_runtime_dependencies_of(patch.node)?);
    let mut telemetry_guard = graph.telemetry_mut();
    let telemetry = telemetry_guard.as_deref_mut();
    let admitted = admit_or_error(
        HostComputedApiFamily::CorePreparedEvaluation,
        patch.node,
        current_dependencies.as_slice(),
        patch.prepared,
        telemetry,
    )?;
    drop(telemetry_guard);
    let (prepared, _admitted_reads, dependency_patch) = admitted.into_parts();
    let next_dependencies = CanonicalDependencies::from_slice(dependency_patch.next_dependencies());
    let before_state = graph.get_state(patch.node)?;
    let before_artifact_state = graph.node_runtime_artifact_finalize_image(patch.node)?;
    let contract = graph.get_contract(patch.node)?;
    #[cfg(feature = "parallel")]
    let comparator_policy = comparator_resolver.policy_for_node(
        patch.node,
        graph.node_eval_config(patch.node)?.comparator.as_ref(),
    );
    let recomputed = matches!(prepared.outcome, PreparedEvaluationOutcome::Evaluate)
        && !matches!(prepared.origin, PreparedEvaluationOrigin::MemoizedReuse);
    let partition_aware = !prepared.result.changed_regions.is_empty();
    let rewiring = super::super::lowering_support::rewiring_summary_from_lowered_edges(
        current_dependencies.as_slice(),
        next_dependencies.as_slice(),
    );
    let footprint = build_apply_footprint(patch.node, &current_dependencies, &next_dependencies);
    let dependency_updates = super::super::lowering_support::count_dependency_updates(
        current_dependencies.as_slice(),
        next_dependencies.as_slice(),
    );

    Ok(LoweredTask::new(
        patch.task_index,
        patch.node,
        contract.semantics.produces,
        next_dependencies,
        #[cfg(feature = "parallel")]
        comparator_policy,
        contract.execution.path_class,
        contract.authority.policy,
        footprint,
        LoweredTaskExecution::new(
            prepared,
            before_state,
            before_artifact_state,
            dependency_updates,
            recomputed,
            partition_aware,
            rewiring,
        ),
    ))
}
