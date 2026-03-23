use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use forge_harness::facade::{
    bench, parity_suite, run_id, scenario_id, snapshot_id, AdapterSupport, AttachmentRecord,
    CaptureDepth, ClockDomain, ComparisonMode, DiagnosticsLevel, ExecutionMode, ExecutionPhase,
    ExecutionProfile, ExecutionRequest, HarnessAdapter, HarnessBench, HarnessCapabilities,
    ObservationStatus, ParitySuite, RecordSchemaVersion, RunOutcome, RunRecord, RunStatus,
    ScenarioFixture, SnapshotObservation, SnapshotPayload, SnapshotRecord, StructuredValue,
    TargetStatusRecord,
};
use serde_json::{json, Value};

use crate::facade::*;

use super::runtime::{
    SignalFixtureFactory, SignalHarnessSession, SignalMutationAction, SignalMutationKind,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct SignalHarnessBridge;

pub fn signal_bench(
    fixture: ScenarioFixture<SignalFixtureFactory>,
    request: ExecutionRequest<String>,
) -> HarnessBench<SignalHarnessBridge, SignalFixtureFactory, SignalMutationAction, String> {
    bench(SignalHarnessBridge, fixture, request)
}

pub fn signal_parity_suite(
    fixture: ScenarioFixture<SignalFixtureFactory>,
    request: ExecutionRequest<String>,
    baseline_profile: ExecutionProfile,
) -> ParitySuite<SignalHarnessBridge, SignalFixtureFactory, SignalMutationAction, String> {
    parity_suite(SignalHarnessBridge, fixture, request, baseline_profile)
}

impl SignalHarnessBridge {
    #[cfg(test)]
    fn requires_condition_aware_execution(
        graph: &SignalGraph,
        plan: &crate::logic::planner::EvaluationPlan,
    ) -> Result<bool, SignalError> {
        for task in plan.stages.iter().flat_map(|stage| &stage.tasks) {
            let config = graph.get_entry(task.node)?.get_eval_config();
            if !matches!(config.condition, EvaluationCondition::Always)
                || config.comparator.is_some()
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) fn diagnostics_profile(level: DiagnosticsLevel) -> DiagnosticsTier {
        match level {
            DiagnosticsLevel::Off | DiagnosticsLevel::Operational => {
                DiagnosticsTier::Operational
            }
            DiagnosticsLevel::Development => DiagnosticsTier::Development,
            DiagnosticsLevel::Forensic => DiagnosticsTier::Forensic,
        }
    }

    pub(super) fn runtime_policy(level: DiagnosticsLevel) -> SignalRuntimePolicy {
        SignalRuntimePolicy::for_tier(Self::diagnostics_profile(level))
    }

    pub(super) fn runtime_policy_summary(policy: SignalRuntimePolicy) -> Value {
        json!({
            "profile": format!("{:?}", policy.tier),
            "history_limit": policy.retention_budget.history_limit,
            "detail_limit": policy.retention_budget.detail_limit,
            "retain_history_details": policy.retention_budget.retain_history_details,
            "retain_flow_explanation": policy.retention_budget.retain_flow_explanation,
            "retain_latest_failure_context": policy.retention_budget.retain_latest_failure_context,
            "retain_stage_details": policy.retention_budget.retain_stage_details,
            "capture_forensic_failure_context": policy.retention_budget.capture_forensic_failure_context,
            "explanation_retention": format!("{:?}", policy.retention_budget.explanation_retention),
            "provenance_retention": format!("{:?}", policy.retention_budget.provenance_retention),
            "replay_detail": format!("{:?}", policy.retention_budget.replay_detail),
            "semantic_retention": format!("{:?}", policy.retention_budget.semantic_detail),
            "parallel_admission": {
                "operational_min_parallel_tasks": policy.parallel_admission.operational_min_parallel_tasks,
                "development_min_parallel_tasks": policy.parallel_admission.development_min_parallel_tasks,
                "forensic_min_parallel_tasks": policy.parallel_admission.forensic_min_parallel_tasks,
                "full_parallel_min_tasks": policy.parallel_admission.full_parallel_min_tasks,
            },
        })
    }

    pub(super) fn artifact_materialization_label(
        mode: DiagnosticsAvailability,
    ) -> &'static str {
        match mode {
            DiagnosticsAvailability::RetainedAvailable => "retained",
            DiagnosticsAvailability::ReconstructedAvailable => "reconstructed",
            DiagnosticsAvailability::OmittedByTier => "omitted_by_tier",
            DiagnosticsAvailability::DeniedByBudget => "denied_by_budget",
            DiagnosticsAvailability::UnavailableNotRetained => "unavailable_not_retained",
            DiagnosticsAvailability::UnavailableNotReconstructable => "unavailable_not_reconstructable",
        }
    }

    fn executor(mode: ExecutionMode) -> Result<StageExecutor, SignalError> {
        match mode {
            ExecutionMode::RuntimeDefault | ExecutionMode::Serial => Ok(StageExecutor::Serial),
            ExecutionMode::StagedParallel => {
                #[cfg(feature = "parallel")]
                {
                    Ok(StageExecutor::staged_parallel_precompute(2))
                }
                #[cfg(not(feature = "parallel"))]
                {
                    Err(SignalError::invalid_input(
                        "staged-parallel execution requested without the `parallel` feature",
                    ))
                }
            }
            ExecutionMode::FullParallel => {
                #[cfg(feature = "parallel")]
                {
                    Ok(StageExecutor::full_parallel(2))
                }
                #[cfg(not(feature = "parallel"))]
                {
                    Err(SignalError::invalid_input(
                        "full-parallel execution requested without the `parallel` feature",
                    ))
                }
            }
        }
    }

    fn observation_status(state: NodeState) -> ObservationStatus {
        match state {
            NodeState::Clean => ObservationStatus::Clean,
            NodeState::MaybeStale => ObservationStatus::MaybeStale,
            NodeState::Dirty => ObservationStatus::Dirty,
        }
    }

    fn report_summary(report: &ExecutionReport) -> Value {
        json!({
            "stage_count": report.stage_count,
            "task_count": report.task_count,
            "tasks_executed": report.tasks_executed,
            "tasks_pruned": report.tasks_pruned,
            "tasks_validated_clean": report.tasks_validated_clean,
            "tasks_deferred_by_condition": report.tasks_deferred_by_condition,
            "tasks_reverted_clean_by_condition": report.tasks_reverted_clean_by_condition,
            "tasks_satisfied_by_memoization": report.tasks_satisfied_by_memoization,
            "tasks_with_suppressed_propagation": report.tasks_with_suppressed_propagation,
            "core_storage_profile": CORE_STORAGE_PROFILE_ID,
        })
    }

    pub(super) fn explanation_summary(explanation: &NodeExplanation) -> Value {
        json!({
            "node": explanation.node.to_string(),
            "state": format!("{:?}", explanation.state),
            "contract_reads_mask": explanation.contract_reads.bits(),
            "contract_produces_mask": explanation.contract_produces.bits(),
            "contract_partition_scope_count": explanation.contract_partition_scope.as_ref().map(|scopes| scopes.len()).unwrap_or(0),
            "required_context": format!("{:?}", explanation.required_context),
            "execution_record_id": explanation.execution_record_id,
            "semantic_segment_id": explanation.semantic_segment_id,
            "upstream_count": explanation.upstream.len(),
            "propagation_suppressed": explanation.propagation_suppressed,
            "output_change": explanation.output_change.map(|change| format!("{change:?}")),
        })
    }
}

impl HarnessAdapter for SignalHarnessBridge {
    type Runtime = SignalHarnessSession;
    type Fixture = SignalFixtureFactory;
    type Mutation = SignalMutationAction;
    type TargetId = String;
    type Error = SignalError;

    fn adapter_name(&self) -> &'static str {
        "forge-signal"
    }

    fn capabilities(&self) -> HarnessCapabilities {
        let mut execution_modes = BTreeSet::new();
        execution_modes.insert(ExecutionMode::RuntimeDefault);
        execution_modes.insert(ExecutionMode::Serial);
        #[cfg(feature = "parallel")]
        execution_modes.insert(ExecutionMode::StagedParallel);
        #[cfg(feature = "parallel")]
        execution_modes.insert(ExecutionMode::FullParallel);

        let mut diagnostics_levels = BTreeSet::new();
        diagnostics_levels.insert(DiagnosticsLevel::Off);
        diagnostics_levels.insert(DiagnosticsLevel::Operational);
        diagnostics_levels.insert(DiagnosticsLevel::Development);
        diagnostics_levels.insert(DiagnosticsLevel::Forensic);

        let mut capture_depths = BTreeSet::new();
        capture_depths.insert(CaptureDepth::Minimal);
        capture_depths.insert(CaptureDepth::Standard);
        capture_depths.insert(CaptureDepth::Rich);

        let mut comparison_modes = BTreeSet::new();
        comparison_modes.insert(ComparisonMode::Exact);
        comparison_modes.insert(ComparisonMode::Semantic);
        let mut clock_domains = BTreeSet::new();
        clock_domains.insert(ClockDomain::Logical);
        let mut execution_phases = BTreeSet::new();
        execution_phases.insert(ExecutionPhase::Evaluate);

        let mut rich_record_kinds = BTreeSet::new();
        rich_record_kinds.insert("execution_report".to_string());
        rich_record_kinds.insert("graph_diagnostics".to_string());
        rich_record_kinds.insert("node_explanation".to_string());
        rich_record_kinds.insert("graph_metrics".to_string());

        HarnessCapabilities {
            execution_modes,
            diagnostics_levels,
            capture_depths,
            comparison_modes,
            clock_domains,
            execution_phases,
            replay_support: AdapterSupport::Supported,
            lineage_support: AdapterSupport::Supported,
            provenance_support: AdapterSupport::Supported,
            event_stream_support: AdapterSupport::Unsupported,
            performance_counter_support: AdapterSupport::Supported,
            workload_budget_support: AdapterSupport::Unsupported,
            attachment_support: AdapterSupport::Supported,
            rich_record_kinds,
        }
    }

    fn create_runtime(&self) -> Result<Self::Runtime, Self::Error> {
        Ok(SignalHarnessSession { runtime: None })
    }

    fn load_fixture(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
    ) -> Result<(), Self::Error> {
        runtime.runtime = Some(fixture.fixture.build_runtime()?);
        Ok(())
    }

    fn apply_mutation_batch(
        &self,
        runtime: &mut Self::Runtime,
        batch: &forge_harness::facade::MutationBatch<Self::Mutation>,
    ) -> Result<(), Self::Error> {
        let runtime = runtime.runtime_mut()?;
        let mut pending_regions =
            BTreeMap::<(u32, u32, u8), (NodeId, Aspect, Option<BTreeSet<ChangedRegion>>)>::new();

        for operation in &batch.operations {
            match operation.kind() {
                SignalMutationKind::MarkDirty { label, aspect } => {
                    let node = runtime.resolve(label)?;
                    pending_regions.insert(
                        (node.index(), node.generation(), aspect.id()),
                        (node, *aspect, None),
                    );
                }
                SignalMutationKind::MarkDirtyWithRegions {
                    label,
                    aspect,
                    changed_regions,
                } => {
                    let node = runtime.resolve(label)?;
                    let key = (node.index(), node.generation(), aspect.id());
                    let entry = pending_regions
                        .entry(key)
                        .or_insert_with(|| (node, *aspect, Some(BTreeSet::new())));
                    if let Some(regions) = &mut entry.2 {
                        regions.extend(changed_regions.iter().cloned());
                    }
                }
            }
        }

        let dirty = DirtyBatch::new(pending_regions.into_values().map(
            |(node, aspect, regions)| {
                DirtyBatchEntry::new(
                    node,
                    aspect,
                    regions
                        .map(|regions| regions.into_iter().collect::<Vec<_>>())
                        .unwrap_or_default(),
                )
            },
        ));
        mark_dirty_batch(runtime.graph_mut(), &dirty)?;
        Ok(())
    }

    fn execute(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<RunRecord<Self::TargetId>, Self::Error> {
        let runtime = runtime.runtime_mut()?;
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        let runtime_policy = Self::runtime_policy(profile.diagnostics_level);
        let targets = request
            .targets
            .iter()
            .map(|label| runtime.resolve(label))
            .collect::<Result<Vec<_>, _>>()?;
        runtime.graph.set_runtime_policy(runtime_policy);

        let plan = runtime
            .graph
            .build_evaluation_plan(&targets, EvaluationRequestMode::Default)?;
        let evaluator = Arc::clone(&runtime.evaluator);
        let executor = Self::executor(profile.execution_mode)?;
        #[cfg(test)]
        let report = if Self::requires_condition_aware_execution(&runtime.graph, &plan)? {
            let mut comparator = DefaultComparatorPolicyResolver {
                fallback: VersionComparatorPolicy::Exact,
                custom: DefaultComparatorResolver,
            };
            let mut condition = DefaultConditionResolver;
            crate::logic::planner::execute_test_prepared_plan_with_resolvers(
                &mut runtime.graph,
                &plan,
                &(),
                &move |ctx| evaluator.evaluate(ctx),
                &mut comparator,
                &mut condition,
            )?
        } else {
            runtime.graph.execute_prepared_plan_with_executor(
                &plan,
                &(),
                &move |ctx| evaluator.evaluate(ctx),
                executor,
            )?
        };
        #[cfg(not(test))]
        let report = runtime.graph.execute_prepared_plan_with_executor(
            &plan,
            &(),
            &move |ctx| evaluator.evaluate(ctx),
            executor,
        )?;

        let target_statuses = request
            .targets
            .iter()
            .map(|label| {
                let node = runtime.resolve(label)?;
                let state = runtime.graph.get_state(node)?;
                Ok(TargetStatusRecord {
                    target: label.clone(),
                    status: Self::observation_status(state),
                    detail: Some(format!("{state:?}")),
                })
            })
            .collect::<Result<Vec<_>, SignalError>>()?;

        Ok(RunRecord {
            schema_version: RecordSchemaVersion::V2,
            run_id: run_id.clone(),
            scenario_id,
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            feed_batch: request.feed_batch.clone(),
            execution_mode: profile.execution_mode,
            diagnostics_level: profile.diagnostics_level,
            status: RunStatus::Succeeded,
            outcome: RunOutcome::Completed,
            budget_usage: None,
            requested_targets: request.targets.clone(),
            target_statuses,
            changed_targets: request.targets.clone(),
            attachments: vec![AttachmentRecord::json(
                "evaluation-plan-summary",
                serde_json::to_value(&plan.summary).unwrap_or_else(|_| json!({})),
            )],
            summary: Self::report_summary(&report),
            extensions: BTreeMap::from([
                (
                    "evaluation_plan_summary".to_string(),
                    serde_json::to_value(&plan.summary).unwrap_or_else(|_| json!({})),
                ),
                (
                    "execution_report".to_string(),
                    serde_json::to_value(&report).unwrap_or_else(|_| json!({})),
                ),
                (
                    "stage_parallel_admission".to_string(),
                    json!(report
                        .stages
                        .iter()
                        .map(|stage| {
                            #[cfg(feature = "parallel")]
                            let serial_apply_rejection_reason = stage
                                .serial_apply_rejection_reason
                                .map(|reason| reason.code());
                            #[cfg(not(feature = "parallel"))]
                            let serial_apply_rejection_reason: Option<&'static str> = None;
                            json!({
                                "stage_index": stage.stage_index,
                                "reason": stage.parallel_admission_reason.map(|reason| reason.code()),
                                "message": stage.parallel_admission_message(),
                                "serial_apply_rejection_reason": serial_apply_rejection_reason,
                            })
                        })
                        .collect::<Vec<_>>()),
                ),
                (
                    "runtime_policy".to_string(),
                    Self::runtime_policy_summary(runtime_policy),
                ),
                (
                    "core_storage_profile".to_string(),
                    json!(CORE_STORAGE_PROFILE_ID),
                ),
            ]),
        })
    }

    fn capture_snapshot(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<SnapshotRecord<Self::TargetId>, Self::Error> {
        let runtime = runtime.runtime()?;
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        let observations = request
            .targets
            .iter()
            .map(|label| {
                let node = runtime.resolve(label)?;
                let state = runtime.graph.get_state(node)?;
                Ok(SnapshotObservation {
                    target: label.clone(),
                    status: Self::observation_status(state),
                    detail: Some(format!("{state:?}")),
                    value: Some(SnapshotPayload::Structured(StructuredValue::Json(json!({
                        "node": node.to_string(),
                        "state": format!("{state:?}"),
                    })))),
                })
            })
            .collect::<Result<Vec<_>, SignalError>>()?;

        Ok(SnapshotRecord {
            schema_version: RecordSchemaVersion::V2,
            snapshot_id: snapshot_id(&run_id, "capture"),
            run_id,
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            observations,
            attachments: Vec::new(),
            extensions: BTreeMap::new(),
        })
    }
}



