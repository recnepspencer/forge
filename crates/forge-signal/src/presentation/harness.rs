use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use forge_harness::facade::{
    diagnostics_id, explanation_id, provenance_id, run_id, scenario_id, snapshot_id,
    AdapterSupport, AttachmentRecord, CaptureDepth, ClockDomain, DiagnosticsHarnessAdapter,
    DiagnosticsLevel, DiagnosticsRecord, ExecutionMode, ExecutionPhase, ExecutionProfile,
    ExecutionRequest, ExplanationHarnessAdapter, ExplanationRecord, HarnessAdapter,
    HarnessCapabilities, ObservationStatus, PerformanceHarnessAdapter, ProvenanceHarnessAdapter,
    ProvenanceRecord, RecordSchemaVersion, RunOutcome, RunRecord, RunStatus, ScenarioFixture,
    SnapshotObservation, SnapshotRecord, TargetStatusRecord,
};
use serde_json::{json, Value};

use crate::facade::{
    DiagnosticsProfile, EvaluationRequestMode, ExecutionReadView, ExecutionReport, NodeExplanation,
    NodeId, NodeState, PreparedEvaluation, SignalError, SignalGraph, StageExecutor,
};

pub trait SignalEvaluationDriver: Send + Sync {
    fn evaluate<'a>(
        &self,
        node: NodeId,
        view: &ExecutionReadView<'a>,
    ) -> Result<PreparedEvaluation, SignalError>;
}

impl<F> SignalEvaluationDriver for F
where
    F: for<'a> Fn(NodeId, &ExecutionReadView<'a>) -> Result<PreparedEvaluation, SignalError>
        + Send
        + Sync,
{
    fn evaluate<'a>(
        &self,
        node: NodeId,
        view: &ExecutionReadView<'a>,
    ) -> Result<PreparedEvaluation, SignalError> {
        self(node, view)
    }
}

#[derive(Clone)]
pub struct SignalFixtureFactory {
    builder: Arc<dyn Fn() -> Result<SignalHarnessRuntime, SignalError> + Send + Sync>,
}

impl SignalFixtureFactory {
    pub fn new<F>(builder: F) -> Self
    where
        F: Fn() -> Result<SignalHarnessRuntime, SignalError> + Send + Sync + 'static,
    {
        Self {
            builder: Arc::new(builder),
        }
    }

    pub fn build_runtime(&self) -> Result<SignalHarnessRuntime, SignalError> {
        (self.builder)()
    }
}

#[derive(Clone)]
pub struct SignalMutationAction {
    name: String,
    apply: Arc<dyn Fn(&mut SignalHarnessRuntime) -> Result<(), SignalError> + Send + Sync>,
}

impl SignalMutationAction {
    pub fn new<F>(name: impl Into<String>, apply: F) -> Self
    where
        F: Fn(&mut SignalHarnessRuntime) -> Result<(), SignalError> + Send + Sync + 'static,
    {
        Self {
            name: name.into(),
            apply: Arc::new(apply),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn apply(&self, runtime: &mut SignalHarnessRuntime) -> Result<(), SignalError> {
        (self.apply)(runtime)
    }
}

pub struct SignalHarnessRuntime {
    graph: SignalGraph,
    evaluator: Arc<dyn SignalEvaluationDriver>,
    labels: BTreeMap<String, NodeId>,
}

impl SignalHarnessRuntime {
    pub fn builder() -> SignalHarnessRuntimeBuilder {
        SignalHarnessRuntimeBuilder::new()
    }

    pub fn graph(&self) -> &SignalGraph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut SignalGraph {
        &mut self.graph
    }

    pub fn label(&mut self, name: impl Into<String>, node: NodeId) {
        self.labels.insert(name.into(), node);
    }

    pub fn resolve(&self, label: &str) -> Result<NodeId, SignalError> {
        self.labels.get(label).copied().ok_or_else(|| {
            SignalError::invalid_input(format!("unknown harness target label `{label}`"))
        })
    }

    pub fn labels(&self) -> &BTreeMap<String, NodeId> {
        &self.labels
    }
}

pub struct SignalHarnessRuntimeBuilder {
    graph: SignalGraph,
    evaluator: Option<Arc<dyn SignalEvaluationDriver>>,
    labels: BTreeMap<String, NodeId>,
}

impl SignalHarnessRuntimeBuilder {
    pub fn new() -> Self {
        Self {
            graph: SignalGraph::new(),
            evaluator: None,
            labels: BTreeMap::new(),
        }
    }

    pub fn graph(&self) -> &SignalGraph {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut SignalGraph {
        &mut self.graph
    }

    pub fn label(mut self, name: impl Into<String>, node: NodeId) -> Self {
        self.labels.insert(name.into(), node);
        self
    }

    pub fn insert_label(&mut self, name: impl Into<String>, node: NodeId) {
        self.labels.insert(name.into(), node);
    }

    pub fn with_evaluator<F>(mut self, evaluator: F) -> Self
    where
        F: SignalEvaluationDriver + 'static,
    {
        self.evaluator = Some(Arc::new(evaluator));
        self
    }

    pub fn set_evaluator<F>(&mut self, evaluator: F)
    where
        F: SignalEvaluationDriver + 'static,
    {
        self.evaluator = Some(Arc::new(evaluator));
    }

    pub fn build(self) -> Result<SignalHarnessRuntime, SignalError> {
        let evaluator = self.evaluator.ok_or_else(|| {
            SignalError::invalid_input("signal harness runtime requires an evaluator")
        })?;
        Ok(SignalHarnessRuntime {
            graph: self.graph,
            evaluator,
            labels: self.labels,
        })
    }
}

pub struct SignalHarnessSession {
    runtime: Option<SignalHarnessRuntime>,
}

impl SignalHarnessSession {
    fn runtime(&self) -> Result<&SignalHarnessRuntime, SignalError> {
        self.runtime.as_ref().ok_or_else(|| {
            SignalError::invalid_input("signal harness fixture must be loaded before use")
        })
    }

    fn runtime_mut(&mut self) -> Result<&mut SignalHarnessRuntime, SignalError> {
        self.runtime.as_mut().ok_or_else(|| {
            SignalError::invalid_input("signal harness fixture must be loaded before use")
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SignalHarnessAdapter;

impl SignalHarnessAdapter {
    fn diagnostics_profile(level: DiagnosticsLevel) -> DiagnosticsProfile {
        match level {
            DiagnosticsLevel::Off | DiagnosticsLevel::Operational => {
                DiagnosticsProfile::Operational
            }
            DiagnosticsLevel::Development => DiagnosticsProfile::Development,
            DiagnosticsLevel::Forensic => DiagnosticsProfile::Forensic,
        }
    }

    fn executor(mode: ExecutionMode) -> Result<StageExecutor, SignalError> {
        match mode {
            ExecutionMode::RuntimeDefault | ExecutionMode::Serial => Ok(StageExecutor::Serial),
            ExecutionMode::StagedParallel => {
                #[cfg(feature = "parallel")]
                {
                    Ok(StageExecutor::Parallel)
                }
                #[cfg(not(feature = "parallel"))]
                {
                    Err(SignalError::invalid_input(
                        "staged-parallel execution requested without the `parallel` feature",
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
        })
    }

    fn explanation_summary(explanation: &NodeExplanation) -> Value {
        json!({
            "node": explanation.node.to_string(),
            "state": format!("{:?}", explanation.state),
            "execution_record_id": explanation.execution_record_id,
            "upstream_count": explanation.upstream.len(),
            "propagation_suppressed": explanation.propagation_suppressed,
            "output_change": explanation.output_change.map(|change| format!("{change:?}")),
        })
    }
}

impl HarnessAdapter for SignalHarnessAdapter {
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
        comparison_modes.insert(forge_harness::facade::ComparisonMode::Exact);
        comparison_modes.insert(forge_harness::facade::ComparisonMode::Semantic);
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
            replay_support: AdapterSupport::Unsupported,
            lineage_support: AdapterSupport::Unsupported,
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
        for operation in &batch.operations {
            operation.apply(runtime)?;
        }
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
        let targets = request
            .targets
            .iter()
            .map(|label| runtime.resolve(label))
            .collect::<Result<Vec<_>, _>>()?;

        let plan = runtime
            .graph
            .build_evaluation_plan(&targets, EvaluationRequestMode::Default)?;
        let evaluator = Arc::clone(&runtime.evaluator);
        let report = runtime.graph.execute_prepared_plan_with_executor(
            &plan,
            &move |node, view| evaluator.evaluate(node, view),
            Self::executor(profile.execution_mode)?,
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
            schema_version: RecordSchemaVersion::V1,
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
                    value: Some(json!({
                        "node": node.to_string(),
                        "state": format!("{state:?}"),
                    })),
                })
            })
            .collect::<Result<Vec<_>, SignalError>>()?;

        Ok(SnapshotRecord {
            schema_version: RecordSchemaVersion::V1,
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

impl DiagnosticsHarnessAdapter for SignalHarnessAdapter {
    fn capture_diagnostics(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        profile: &ExecutionProfile,
    ) -> Result<DiagnosticsRecord, Self::Error> {
        let runtime = runtime.runtime()?;
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, "diagnostics");
        let summary = runtime
            .graph
            .diagnostics_summary(Self::diagnostics_profile(profile.diagnostics_level));

        Ok(DiagnosticsRecord {
            schema_version: RecordSchemaVersion::V1,
            diagnostics_id: diagnostics_id(&run_id),
            run_id,
            adapter_name: self.adapter_name().to_string(),
            profile_name: profile.name.clone(),
            level: profile.diagnostics_level,
            time_marker: profile.time_marker.clone(),
            attachments: Vec::new(),
            summary: serde_json::to_value(summary).unwrap_or_else(|_| json!({})),
            extensions: BTreeMap::new(),
        })
    }
}

impl ExplanationHarnessAdapter for SignalHarnessAdapter {
    fn capture_explanations(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<ExplanationRecord<Self::TargetId>>, Self::Error> {
        let runtime = runtime.runtime()?;
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        request
            .targets
            .iter()
            .map(|label| {
                let node = runtime.resolve(label)?;
                let explanation = runtime.graph.explain(node)?;
                Ok(ExplanationRecord {
                    schema_version: RecordSchemaVersion::V1,
                    explanation_id: explanation_id(&run_id, label),
                    run_id: run_id.clone(),
                    adapter_name: self.adapter_name().to_string(),
                    target: label.clone(),
                    time_marker: profile.time_marker.clone(),
                    attachments: Vec::new(),
                    summary: Self::explanation_summary(&explanation),
                    extensions: BTreeMap::new(),
                })
            })
            .collect()
    }
}

impl ProvenanceHarnessAdapter for SignalHarnessAdapter {
    fn capture_provenance(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<Vec<ProvenanceRecord<Self::TargetId>>, Self::Error> {
        let runtime = runtime.runtime()?;
        let scenario_id = scenario_id(&fixture.name);
        let run_id = run_id(&scenario_id, &profile.name, &request.name);
        request
            .targets
            .iter()
            .map(|label| {
                let node = runtime.resolve(label)?;
                let explanation = runtime.graph.explain(node)?;
                Ok(ProvenanceRecord {
                    schema_version: RecordSchemaVersion::V1,
                    provenance_id: provenance_id(&run_id, label),
                    run_id: run_id.clone(),
                    adapter_name: self.adapter_name().to_string(),
                    target: label.clone(),
                    time_marker: profile.time_marker.clone(),
                    attachments: Vec::new(),
                    summary: json!({
                        "execution_record_id": explanation.execution_record_id,
                        "upstream_count": explanation.upstream.len(),
                        "propagation_suppressed": explanation.propagation_suppressed,
                    }),
                    extensions: BTreeMap::new(),
                })
            })
            .collect()
    }
}

impl PerformanceHarnessAdapter for SignalHarnessAdapter {
    fn capture_performance(
        &self,
        runtime: &Self::Runtime,
        _fixture: &ScenarioFixture<Self::Fixture>,
        _profile: &ExecutionProfile,
    ) -> Result<Value, Self::Error> {
        let runtime = runtime.runtime()?;
        Ok(serde_json::to_value(runtime.graph.metrics()).unwrap_or_else(|_| json!({})))
    }
}
