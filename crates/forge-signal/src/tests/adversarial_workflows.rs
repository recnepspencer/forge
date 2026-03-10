use std::collections::BTreeMap;
use serde::Serialize;

use crate::data::checkpoint::CheckpointBarrier;
use crate::data::event_subscriber::{EventSubscriber, SubscriberId};
use crate::data::subscriber_context::SubscriberContext;
use crate::facade::*;
use crate::logic::transaction::SignalRuntime;
use crate::tests::support::*;

type DefaultRuntime = SignalRuntime<(), (), (), (), ()>;
type DefaultTx<'a> = SignalTransaction<'a, (), (), (), (), ()>;
type EventRuntime = SignalRuntime<EventDomain, (), WorkflowEvent, (), ()>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
struct WorkflowSeed(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
enum WorkflowDomain {
    GeometryKernel,
    Fintech,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
enum AdversarialWorkflow {
    FeatureEditRewireRestoreChurn,
    PartitionScopeCliffSession,
    LateTickCorrectionWithBranchReplay,
    RiskAlertFlapUnderMemoChurn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
enum FailureInjectionPoint {
    DuringEvaluation,
    DuringBranchLocalWork,
    DuringSnapshotRestoreChurn,
    DuringEventFlush,
}

#[derive(Clone, Debug, Serialize)]
struct WorkflowStep {
    index: usize,
    label: String,
    active_branch: String,
    failure_injection: Option<FailureInjectionPoint>,
}

#[derive(Clone, Debug, Serialize)]
struct InvariantReport {
    step_index: usize,
    errors: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct DifferentialFailureBundle {
    domain: WorkflowDomain,
    workflow: AdversarialWorkflow,
    seed: WorkflowSeed,
    policy: String,
    executor: String,
    steps: Vec<WorkflowStep>,
    note: String,
    replay_diff_count: usize,
    lineage_diff_count: usize,
}

#[derive(Clone)]
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: WorkflowSeed) -> Self {
        Self { state: seed.0.max(1) }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 32) as u32
    }

    fn choose(&mut self, len: usize) -> usize {
        (self.next_u32() as usize) % len
    }

    fn coin(&mut self) -> bool {
        self.next_u32() & 1 == 0
    }

    fn small_delta(&mut self) -> u64 {
        1 + (self.next_u32() % 3) as u64
    }
}

#[derive(Clone)]
struct BranchTruth {
    a: u64,
    b: u64,
    head_snapshot: Option<SignalSnapshotId>,
}

#[derive(Default, Clone)]
struct ReferenceModel {
    active: SignalBranchId,
    branches: BTreeMap<SignalBranchId, BranchTruth>,
    snapshots: BTreeMap<SignalSnapshotId, BranchTruth>,
}

impl ReferenceModel {
    fn branch(&self, branch_id: SignalBranchId) -> &BranchTruth {
        self.branches.get(&branch_id).unwrap()
    }

    fn branch_mut(&mut self, branch_id: SignalBranchId) -> &mut BranchTruth {
        self.branches.get_mut(&branch_id).unwrap()
    }
}

fn capture_active_branch_snapshot<D, I, E, Ctx, T>(
    runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    model: &mut ReferenceModel,
    snapshots: &mut BTreeMap<SignalSnapshotId, SignalSnapshotV1>,
) -> SignalSnapshotV1
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let branch = runtime.current_branch();
    let snapshot = runtime.capture_branch_snapshot(branch.clone()).unwrap();
    let mut truth = model.branch(branch.id).clone();
    truth.head_snapshot = Some(snapshot.meta.snapshot_id);
    snapshots.insert(snapshot.meta.snapshot_id, snapshot.clone());
    model.snapshots.insert(snapshot.meta.snapshot_id, truth);
    model.branch_mut(branch.id).head_snapshot = Some(snapshot.meta.snapshot_id);
    snapshot
}

struct GeometryFixture {
    runtime: DefaultRuntime,
    source_a: NodeId,
    source_b: NodeId,
    delta_gate: NodeId,
    filtered_gate: NodeId,
    demand_gate: NodeId,
    fused: NodeId,
    keyed: NodeId,
    memo_key: KeyedComputation,
}

struct FintechFixture {
    runtime: DefaultRuntime,
    ticks: NodeId,
    volatility: NodeId,
    throttle: NodeId,
    alert: NodeId,
    risk: NodeId,
    keyed: NodeId,
    memo_key: KeyedComputation,
}

struct SignalAdversarialHarness {
    seed: WorkflowSeed,
    domain: WorkflowDomain,
    workflow: AdversarialWorkflow,
    steps: Vec<WorkflowStep>,
}

impl SignalAdversarialHarness {
    fn new(seed: WorkflowSeed, domain: WorkflowDomain, workflow: AdversarialWorkflow) -> Self {
        Self {
            seed,
            domain,
            workflow,
            steps: Vec::new(),
        }
    }

    fn record<D, I, E, Ctx, T>(
        &mut self,
        runtime: &SignalRuntime<D, I, E, Ctx, T>,
        index: usize,
        label: impl Into<String>,
        failure_injection: Option<FailureInjectionPoint>,
    )
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        T: Copy + Ord,
    {
        self.steps.push(WorkflowStep {
            index,
            label: label.into(),
            active_branch: runtime.current_branch().name,
            failure_injection,
        });
    }

    fn panic_invariant(
        &self,
        policy: &SignalRuntimePolicy,
        executor: &str,
        report: InvariantReport,
    ) -> ! {
        let bundle = DifferentialFailureBundle {
            domain: self.domain,
            workflow: self.workflow,
            seed: self.seed,
            policy: format!("{:?}", policy.profile),
            executor: executor.to_string(),
            steps: self.steps.clone(),
            note: report.errors.join(" | "),
            replay_diff_count: 0,
            lineage_diff_count: 0,
        };
        panic!("{}", serde_json::to_string_pretty(&bundle).unwrap());
    }

    fn panic_diff(
        &self,
        policy: &SignalRuntimePolicy,
        executor: &str,
        note: impl Into<String>,
        replay_diff_count: usize,
        lineage_diff_count: usize,
    ) -> ! {
        let bundle = DifferentialFailureBundle {
            domain: self.domain,
            workflow: self.workflow,
            seed: self.seed,
            policy: format!("{:?}", policy.profile),
            executor: executor.to_string(),
            steps: self.steps.clone(),
            note: note.into(),
            replay_diff_count,
            lineage_diff_count,
        };
        panic!("{}", serde_json::to_string_pretty(&bundle).unwrap());
    }
}

fn geometry_precompute(
    fixture: &GeometryFixture,
) -> impl Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync {
    let source_a = fixture.source_a;
    let source_b = fixture.source_b;
    let delta_gate = fixture.delta_gate;
    let filtered_gate = fixture.filtered_gate;
    let demand_gate = fixture.demand_gate;
    let fused = fixture.fused;
    move |node, view| {
        if node == delta_gate {
            let a = view.read_aspect_version(source_a, ASPECT_A)?.get(ASPECT_A);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(a, 0))
                    .with_output_identity(format!("geom-delta-{a}"))
                    .with_continuity_token("geom-delta"),
            ));
        }
        if node == filtered_gate {
            let b = view.read_aspect_version(source_b, ASPECT_B)?.get(ASPECT_B);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(0, b))
                    .with_output_identity(format!("geom-filter-{b}"))
                    .with_continuity_token("geom-filter"),
            ));
        }
        if node == demand_gate {
            let a = view.read_aspect_version(source_a, ASPECT_A)?.get(ASPECT_A);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(a, 0))
                    .with_output_identity(format!("geom-demand-{a}"))
                    .with_continuity_token("geom-demand"),
            ));
        }
        if node == fused {
            let a = view.read_aspect_version(delta_gate, ASPECT_A)?.get(ASPECT_A);
            let b = view.read_aspect_version(filtered_gate, ASPECT_B)?.get(ASPECT_B);
            let demand = view.read_aspect_version(demand_gate, ASPECT_A)?.get(ASPECT_A);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(a.max(demand), b))
                    .with_output_identity(format!("geom-fused-{a}-{b}-{demand}"))
                    .with_continuity_token("geom-fused"),
            ));
        }
        Err(SignalError::invalid_input(format!(
            "unexpected geometry node {node}"
        )))
    }
}

fn fintech_precompute(
    fixture: &FintechFixture,
) -> impl Fn(NodeId, &ExecutionReadView<'_>) -> Result<PreparedEvaluation, SignalError> + Sync {
    let ticks = fixture.ticks;
    let volatility = fixture.volatility;
    let throttle = fixture.throttle;
    let alert = fixture.alert;
    let risk = fixture.risk;
    move |node, view| {
        if node == throttle {
            let a = view.read_aspect_version(ticks, ASPECT_A)?.get(ASPECT_A);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(a, 0))
                    .with_output_identity(format!("throttle-{a}"))
                    .with_continuity_token("throttle"),
            ));
        }
        if node == alert {
            let b = view.read_aspect_version(volatility, ASPECT_B)?.get(ASPECT_B);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(0, b))
                    .with_output_identity(format!("alert-{b}"))
                    .with_continuity_token("alert"),
            ));
        }
        if node == risk {
            let a = view.read_aspect_version(throttle, ASPECT_A)?.get(ASPECT_A);
            let b = view.read_aspect_version(alert, ASPECT_B)?.get(ASPECT_B);
            return Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(a, b))
                    .with_output_identity(format!("risk-{a}-{b}"))
                    .with_continuity_token("risk-surface"),
            ));
        }
        Err(SignalError::invalid_input(format!(
            "unexpected fintech node {node}"
        )))
    }
}

fn build_geometry_fixture(policy: SignalRuntimePolicy) -> GeometryFixture {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime.set_runtime_policy(policy);

    let source_a = runtime
        .graph_mut()
        .node()
        .output_identity()
        .partitioned_output()
        .build();
    let source_b = runtime
        .graph_mut()
        .node()
        .output_identity()
        .partitioned_output()
        .build();
    let delta_gate = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(mask_a())
        .delta_threshold(2.0)
        .build();
    let filtered_gate = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(mask_b())
        .aspect_filter(mask_b())
        .build();
    let demand_gate = runtime.graph_mut().node().on_demand().build();
    let fused = runtime.graph_mut().node().output_identity().build();

    runtime
        .graph_mut()
        .add_partition_detail_dependency(delta_gate, source_a, ASPECT_A, "wing", "left")
        .unwrap();
    runtime
        .graph_mut()
        .add_partition_dependency(filtered_gate, source_b, ASPECT_B, "lod")
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(demand_gate, source_a, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(fused, delta_gate, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(fused, filtered_gate, ASPECT_B)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(fused, demand_gate, ASPECT_A)
        .unwrap();

    let family = runtime.register_computation_family("geom-workflow");
    let keyed = runtime.keyed_node(&family, "feature-panel");
    let memo_key = KeyedComputation::new(family.clone(), "feature-panel").with_memo_key("mesh-v1");

    GeometryFixture {
        runtime,
        source_a,
        source_b,
        delta_gate,
        filtered_gate,
        demand_gate,
        fused,
        keyed,
        memo_key,
    }
}

fn build_fintech_fixture(policy: SignalRuntimePolicy) -> FintechFixture {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime.set_runtime_policy(policy);

    let ticks = runtime.graph_mut().node().output_identity().build();
    let volatility = runtime.graph_mut().node().output_identity().build();
    let throttle = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(mask_a())
        .delta_threshold(2.0)
        .build();
    let alert = runtime
        .graph_mut()
        .node()
        .depends_on_aspects(mask_b())
        .aspect_filter(mask_b())
        .build();
    let risk = runtime.graph_mut().node().output_identity().build();

    runtime
        .graph_mut()
        .add_dependency(throttle, ticks, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(alert, volatility, ASPECT_B)
        .unwrap();
    runtime.graph_mut().add_dependency(risk, throttle, ASPECT_A).unwrap();
    runtime.graph_mut().add_dependency(risk, alert, ASPECT_B).unwrap();

    let family = runtime.register_computation_family("fintech-workflow");
    let keyed = runtime.keyed_node(&family, "risk-book");
    let memo_key = KeyedComputation::new(family.clone(), "risk-book").with_memo_key("risk-v1");

    FintechFixture {
        runtime,
        ticks,
        volatility,
        throttle,
        alert,
        risk,
        keyed,
        memo_key,
    }
}

fn seed_geometry_baseline(
    fixture: &mut GeometryFixture,
    model: &mut ReferenceModel,
) -> (SignalBranchHandle, SignalSnapshotV1) {
    let mut ctx = ();
    fixture
        .runtime
        .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
            tx.read(fixture.source_a, &|_node, view: &ExecutionReadView<'_>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("geom-source-a-1"),
                ))
            })?;
            tx.read(fixture.source_b, &|_node, view: &ExecutionReadView<'_>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(0, 1))
                        .with_output_identity("geom-source-b-1"),
                ))
            })?;
            tx.evaluate_keyed(fixture.keyed, &fixture.memo_key, &|_id, view: &ExecutionReadView<'_>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 1))
                        .with_output_identity("geom-keyed-1")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    fixture
        .runtime
        .evaluate_dirty_with_executor(&geometry_precompute(fixture), StageExecutor::Serial)
        .unwrap();
    fixture
        .runtime
        .evaluate_with_plan_and_executor(
            fixture.demand_gate,
            &geometry_precompute(fixture),
            EvaluationRequestMode::ForceOnDemand,
            StageExecutor::Serial,
        )
        .unwrap();

    let main = fixture.runtime.current_branch();
    model.active = main.id;
    model.branches.insert(
        main.id,
        BranchTruth {
            a: 1,
            b: 1,
            head_snapshot: None,
        },
    );
    let snapshot = fixture.runtime.capture_snapshot();
    let mut truth = model.branch(main.id).clone();
    truth.head_snapshot = Some(snapshot.meta.snapshot_id);
    model.snapshots.insert(snapshot.meta.snapshot_id, truth);
    model.branch_mut(main.id).head_snapshot = Some(snapshot.meta.snapshot_id);
    (main, snapshot)
}

fn seed_fintech_baseline(
    fixture: &mut FintechFixture,
    model: &mut ReferenceModel,
) -> (SignalBranchHandle, SignalSnapshotV1) {
    let mut ctx = ();
    fixture
        .runtime
        .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
            tx.read(fixture.ticks, &|_node, view: &ExecutionReadView<'_>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("tick-1"),
                ))
            })?;
            tx.read(fixture.volatility, &|_node, view: &ExecutionReadView<'_>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(0, 1))
                        .with_output_identity("vol-1"),
                ))
            })?;
            tx.evaluate_keyed(fixture.keyed, &fixture.memo_key, &|_id, view: &ExecutionReadView<'_>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 1))
                        .with_output_identity("risk-keyed-1")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    fixture
        .runtime
        .evaluate_dirty_with_executor(&fintech_precompute(fixture), StageExecutor::Serial)
        .unwrap();

    let main = fixture.runtime.current_branch();
    model.active = main.id;
    model.branches.insert(
        main.id,
        BranchTruth {
            a: 1,
            b: 1,
            head_snapshot: None,
        },
    );
    let snapshot = fixture.runtime.capture_snapshot();
    let mut truth = model.branch(main.id).clone();
    truth.head_snapshot = Some(snapshot.meta.snapshot_id);
    model.snapshots.insert(snapshot.meta.snapshot_id, truth);
    model.branch_mut(main.id).head_snapshot = Some(snapshot.meta.snapshot_id);
    (main, snapshot)
}

fn assert_runtime_invariants(
    graph: &SignalGraph,
    model: &ReferenceModel,
    current_branch: SignalBranchHandle,
    node_a: NodeId,
    aspect_a: Aspect,
    node_b: NodeId,
    aspect_b: Aspect,
    policy: SignalRuntimePolicy,
) -> InvariantReport {
    let mut errors = Vec::new();
    for node in graph.live_node_ids() {
        let entry = graph.get_entry(node).unwrap();
        if !matches!(entry.get_state(), NodeState::Clean) && entry.get_dirty_aspects().is_empty() {
            errors.push(format!("dirty node {node} has an empty dirty-aspect mask"));
        }
        for dependency in graph.dependencies_of(node).unwrap() {
            if !graph.is_alive(dependency.source()) {
                errors.push(format!(
                    "node {node} depends on stale upstream {}",
                    dependency.source()
                ));
                continue;
            }
            if !graph
                .subscribers_of(dependency.source())
                .unwrap()
                .contains(&node)
            {
                errors.push(format!(
                    "subscriber index missing back-edge {} -> {node}",
                    dependency.source()
                ));
            }
        }
        for subscriber in graph.subscribers_of(node).unwrap() {
            if !graph.is_alive(*subscriber) {
                errors.push(format!("node {node} points at stale subscriber {subscriber}"));
                continue;
            }
            let has_backref = graph
                .dependencies_of(*subscriber)
                .unwrap()
                .iter()
                .any(|dependency| dependency.source() == node);
            if !has_backref {
                errors.push(format!(
                    "dependency index missing back-edge {node} -> {subscriber}"
                ));
            }
        }
    }

    let expected = model.branch(current_branch.id);
    let actual_a = graph
        .get_entry(node_a)
        .unwrap()
        .get_aspect_version()
        .get(aspect_a);
    let actual_b = graph
        .get_entry(node_b)
        .unwrap()
        .get_aspect_version()
        .get(aspect_b);
    if actual_a != expected.a {
        errors.push(format!(
            "branch `{}` expected aspect A version {}, got {}",
            current_branch.name, expected.a, actual_a
        ));
    }
    if actual_b != expected.b {
        errors.push(format!(
            "branch `{}` expected aspect B version {}, got {}",
            current_branch.name, expected.b, actual_b
        ));
    }
    if graph.branch_head_snapshot_id(current_branch.id) != expected.head_snapshot {
        errors.push(format!(
            "branch `{}` head snapshot drifted: expected {:?}, got {:?}",
            current_branch.name,
            expected.head_snapshot,
            graph.branch_head_snapshot_id(current_branch.id)
        ));
    }
    if graph.recent_execution_history_diagnostics().len() > policy.history_limit {
        errors.push(format!(
            "history retention exceeded policy: {} > {}",
            graph.recent_execution_history_diagnostics().len(),
            policy.history_limit
        ));
    }

    InvariantReport {
        step_index: 0,
        errors,
    }
}

fn geometry_session(
    seed: WorkflowSeed,
    workflow: AdversarialWorkflow,
    policy: SignalRuntimePolicy,
    executor: StageExecutor,
) -> (SignalAdversarialHarness, ReplaySlice, Vec<LineageRecord>) {
    let mut harness = SignalAdversarialHarness::new(seed, WorkflowDomain::GeometryKernel, workflow);
    let mut fixture = build_geometry_fixture(policy);
    let mut model = ReferenceModel::default();
    let mut snapshots = BTreeMap::new();
    let (main, main_snapshot) = seed_geometry_baseline(&mut fixture, &mut model);
    snapshots.insert(main_snapshot.meta.snapshot_id, main_snapshot.clone());
    harness.record(&fixture.runtime, 0, "seed-main-baseline", None);

    let feature = fixture.runtime.create_branch("feature").unwrap();
    model
        .branches
        .insert(feature.id, model.branch(main.id).clone());
    fixture.runtime.switch_branch(feature.clone()).unwrap();
    model.active = feature.id;
    harness.record(&fixture.runtime, 1, "create-feature-branch", None);
    let feature_snapshot = fixture
        .runtime
        .capture_branch_snapshot(feature.clone())
        .unwrap();
    snapshots.insert(feature_snapshot.meta.snapshot_id, feature_snapshot.clone());
    let mut feature_truth = model.branch(feature.id).clone();
    feature_truth.head_snapshot = Some(feature_snapshot.meta.snapshot_id);
    model
        .snapshots
        .insert(feature_snapshot.meta.snapshot_id, feature_truth);
    model.branch_mut(feature.id).head_snapshot = Some(feature_snapshot.meta.snapshot_id);

    let analysis = fixture.runtime.create_branch("analysis").unwrap();
    model
        .branches
        .insert(analysis.id, model.branch(feature.id).clone());
    fixture.runtime.switch_branch(analysis.clone()).unwrap();
    model.active = analysis.id;
    let analysis_snapshot = fixture
        .runtime
        .capture_branch_snapshot(analysis.clone())
        .unwrap();
    snapshots.insert(analysis_snapshot.meta.snapshot_id, analysis_snapshot.clone());
    let mut analysis_truth = model.branch(analysis.id).clone();
    analysis_truth.head_snapshot = Some(analysis_snapshot.meta.snapshot_id);
    model
        .snapshots
        .insert(analysis_snapshot.meta.snapshot_id, analysis_truth);
    model.branch_mut(analysis.id).head_snapshot = Some(analysis_snapshot.meta.snapshot_id);
    harness.record(&fixture.runtime, 2, "create-analysis-branch", None);

    let mut rng = Lcg::new(seed);
    let mut ctx = ();
    for step in 0..24 {
        let action = match workflow {
            AdversarialWorkflow::FeatureEditRewireRestoreChurn => rng.choose(6),
            AdversarialWorkflow::PartitionScopeCliffSession => rng.choose(5),
            _ => rng.choose(6),
        };

        match action {
            0 => {
                let target = [main.clone(), feature.clone(), analysis.clone()][rng.choose(3)].clone();
                fixture.runtime.switch_branch(target.clone()).unwrap();
                model.active = target.id;
                harness.record(&fixture.runtime, step + 3, "switch-branch", None);
            }
            1 => {
                let delta = rng.small_delta();
                let current = model.branch(model.active).clone();
                let next_a = current.a + delta;
                let result = fixture.runtime.transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                    tx.mark_dirty_with_regions(
                        fixture.source_a,
                        ASPECT_A,
                        &[ChangedRegion::new("wing").with_detail(format!("panel-{step}"))],
                    )?;
                    tx.read(fixture.source_a, &move |_node, view: &ExecutionReadView<'_>| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(next_a, 0))
                                .with_output_identity(format!("geom-source-a-{next_a}")),
                        ))
                    })?;
                    Ok(())
                });
                result.unwrap();
                model.branch_mut(model.active).a = next_a;
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&geometry_precompute(&fixture), executor)
                    .unwrap();
                if rng.coin() {
                    capture_active_branch_snapshot(&mut fixture.runtime, &mut model, &mut snapshots);
                }
                harness.record(&fixture.runtime, step + 3, "update-source-a", None);
            }
            2 => {
                let delta = rng.small_delta();
                let current = model.branch(model.active).clone();
                let next_b = current.b + delta;
                let result = fixture.runtime.transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                    tx.mark_dirty_with_regions(
                        fixture.source_b,
                        ASPECT_B,
                        &[ChangedRegion::new("lod")],
                    )?;
                    tx.read(fixture.source_b, &move |_node, view: &ExecutionReadView<'_>| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(0, next_b))
                                .with_output_identity(format!("geom-source-b-{next_b}")),
                        ))
                    })?;
                    Ok(())
                });
                result.unwrap();
                model.branch_mut(model.active).b = next_b;
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&geometry_precompute(&fixture), executor)
                    .unwrap();
                if rng.coin() {
                    capture_active_branch_snapshot(&mut fixture.runtime, &mut model, &mut snapshots);
                }
                harness.record(&fixture.runtime, step + 3, "update-source-b", None);
            }
            3 => {
                let delta = rng.small_delta();
                let current = model.branch(model.active).clone();
                let bad_a = current.a + delta;
                let err = fixture.runtime.transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                    tx.mark_dirty(fixture.source_a, ASPECT_A)?;
                    tx.read(fixture.source_a, &move |_node, view: &ExecutionReadView<'_>| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(bad_a, 0))
                                .with_output_identity(format!("geom-source-a-bad-{bad_a}")),
                        ))
                    })?;
                    Err(SignalError::invalid_input("synthetic geometry rollback"))
                });
                assert!(err.is_err());
                harness.record(
                    &fixture.runtime,
                    step + 3,
                    "rollback-source-a-update",
                    Some(FailureInjectionPoint::DuringEvaluation),
                );
            }
            4 => {
                fixture
                    .runtime
                    .evaluate_with_plan_and_executor(
                        fixture.demand_gate,
                        &geometry_precompute(&fixture),
                        EvaluationRequestMode::ForceOnDemand,
                        executor,
                    )
                    .unwrap();
                harness.record(&fixture.runtime, step + 3, "force-on-demand", None);
            }
            _ => {
                let active = fixture.runtime.current_branch();
                let snapshot_id = model
                    .branch(active.id)
                    .head_snapshot
                    .expect("branch restore should always have a branch-local head snapshot");
                let restore_snapshot = snapshots
                    .get(&snapshot_id)
                    .expect("branch restore should use a stored branch-local snapshot")
                    .clone();
                fixture
                    .runtime
                    .restore_branch_snapshot(active.clone(), &restore_snapshot)
                    .unwrap();
                let restored = model
                    .snapshots
                    .get(&restore_snapshot.meta.snapshot_id)
                    .unwrap()
                    .clone();
                *model.branch_mut(active.id) = restored;
                model.branch_mut(active.id).head_snapshot =
                    fixture.runtime.branch_head_snapshot_id(active.id);
                harness.record(
                    &fixture.runtime,
                    step + 3,
                    "restore-current-branch",
                    Some(FailureInjectionPoint::DuringSnapshotRestoreChurn),
                );
            }
        }

        let report = assert_runtime_invariants(
            fixture.runtime.graph(),
            &model,
            fixture.runtime.current_branch(),
            fixture.source_a,
            ASPECT_A,
            fixture.source_b,
            ASPECT_B,
            policy,
        );
        if !report.errors.is_empty() {
            harness.panic_invariant(&policy, &format!("{executor:?}"), report);
        }
    }

    for branch in [main.clone(), feature.clone(), analysis.clone()] {
        fixture.runtime.switch_branch(branch.clone()).unwrap();
        let report = assert_runtime_invariants(
            fixture.runtime.graph(),
            &model,
            branch,
            fixture.source_a,
            ASPECT_A,
            fixture.source_b,
            ASPECT_B,
            policy,
        );
        if !report.errors.is_empty() {
            harness.panic_invariant(&policy, &format!("{executor:?}"), report);
        }
    }

    let replay = fixture
        .runtime
        .replay_for_branch(fixture.runtime.current_branch().id);
    let lineage = fixture.runtime.graph().lineage_for_node(fixture.fused);
    (harness, replay, lineage)
}

fn fintech_session(
    seed: WorkflowSeed,
    workflow: AdversarialWorkflow,
    policy: SignalRuntimePolicy,
    executor: StageExecutor,
) -> (SignalAdversarialHarness, ReplaySlice, Vec<LineageRecord>) {
    let mut harness = SignalAdversarialHarness::new(seed, WorkflowDomain::Fintech, workflow);
    let mut fixture = build_fintech_fixture(policy);
    let mut model = ReferenceModel::default();
    let mut snapshots = BTreeMap::new();
    let (main, main_snapshot) = seed_fintech_baseline(&mut fixture, &mut model);
    snapshots.insert(main_snapshot.meta.snapshot_id, main_snapshot.clone());
    harness.record(&fixture.runtime, 0, "seed-main-baseline", None);

    let what_if = fixture.runtime.create_branch("what-if").unwrap();
    model
        .branches
        .insert(what_if.id, model.branch(main.id).clone());
    fixture.runtime.switch_branch(what_if.clone()).unwrap();
    model.active = what_if.id;
    let what_if_snapshot = fixture
        .runtime
        .capture_branch_snapshot(what_if.clone())
        .unwrap();
    snapshots.insert(what_if_snapshot.meta.snapshot_id, what_if_snapshot.clone());
    let mut what_if_truth = model.branch(what_if.id).clone();
    what_if_truth.head_snapshot = Some(what_if_snapshot.meta.snapshot_id);
    model
        .snapshots
        .insert(what_if_snapshot.meta.snapshot_id, what_if_truth);
    model.branch_mut(what_if.id).head_snapshot = Some(what_if_snapshot.meta.snapshot_id);
    harness.record(&fixture.runtime, 1, "create-what-if-branch", None);

    let correction = fixture.runtime.create_branch("correction").unwrap();
    model
        .branches
        .insert(correction.id, model.branch(what_if.id).clone());
    fixture.runtime.switch_branch(correction.clone()).unwrap();
    model.active = correction.id;
    let correction_snapshot = fixture
        .runtime
        .capture_branch_snapshot(correction.clone())
        .unwrap();
    snapshots.insert(correction_snapshot.meta.snapshot_id, correction_snapshot.clone());
    let mut correction_truth = model.branch(correction.id).clone();
    correction_truth.head_snapshot = Some(correction_snapshot.meta.snapshot_id);
    model
        .snapshots
        .insert(correction_snapshot.meta.snapshot_id, correction_truth);
    model.branch_mut(correction.id).head_snapshot = Some(correction_snapshot.meta.snapshot_id);
    harness.record(&fixture.runtime, 2, "create-correction-branch", None);

    let mut rng = Lcg::new(seed);
    let mut ctx = ();
    for step in 0..24 {
        let action = match workflow {
            AdversarialWorkflow::LateTickCorrectionWithBranchReplay => rng.choose(6),
            AdversarialWorkflow::RiskAlertFlapUnderMemoChurn => rng.choose(5),
            _ => rng.choose(6),
        };

        match action {
            0 => {
                let target = [main.clone(), what_if.clone(), correction.clone()][rng.choose(3)]
                    .clone();
                fixture.runtime.switch_branch(target.clone()).unwrap();
                model.active = target.id;
                harness.record(&fixture.runtime, step + 3, "switch-branch", None);
            }
            1 => {
                let delta = rng.small_delta();
                let current = model.branch(model.active).clone();
                let next_a = current.a + delta;
                fixture
                    .runtime
                    .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                        tx.mark_dirty(fixture.ticks, ASPECT_A)?;
                        tx.read(fixture.ticks, &move |_node, view: &ExecutionReadView<'_>| {
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version_ab(next_a, 0))
                                    .with_output_identity(format!("ticks-{next_a}")),
                            ))
                        })?;
                        tx.evaluate_keyed(fixture.keyed, &fixture.memo_key, &|_id, view: &ExecutionReadView<'_>| {
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version_ab(next_a, current.b))
                                    .with_output_identity(format!("risk-keyed-{next_a}-{}", current.b))
                                    .with_output_change(OutputChange::Refreshed),
                            ))
                        })?;
                        Ok(())
                    })
                    .unwrap();
                model.branch_mut(model.active).a = next_a;
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&fintech_precompute(&fixture), executor)
                    .unwrap();
                if rng.coin() {
                    capture_active_branch_snapshot(&mut fixture.runtime, &mut model, &mut snapshots);
                }
                harness.record(&fixture.runtime, step + 3, "update-ticks", None);
            }
            2 => {
                let delta = rng.small_delta();
                let current = model.branch(model.active).clone();
                let next_b = current.b + delta;
                fixture
                    .runtime
                    .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                        tx.mark_dirty(fixture.volatility, ASPECT_B)?;
                        tx.read(fixture.volatility, &move |_node, view: &ExecutionReadView<'_>| {
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version_ab(0, next_b))
                                    .with_output_identity(format!("volatility-{next_b}")),
                            ))
                        })?;
                        Ok(())
                    })
                    .unwrap();
                model.branch_mut(model.active).b = next_b;
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&fintech_precompute(&fixture), executor)
                    .unwrap();
                if rng.coin() {
                    capture_active_branch_snapshot(&mut fixture.runtime, &mut model, &mut snapshots);
                }
                harness.record(&fixture.runtime, step + 3, "update-volatility", None);
            }
            3 => {
                let current = model.branch(model.active).clone();
                let err = fixture.runtime.transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                    tx.mark_dirty(fixture.ticks, ASPECT_A)?;
                    tx.read(fixture.ticks, &move |_node, view: &ExecutionReadView<'_>| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(current.a + 10, 0))
                                .with_output_identity("bad-ticks"),
                        ))
                    })?;
                    Err(SignalError::invalid_input("synthetic branch-local failure"))
                });
                assert!(err.is_err());
                harness.record(
                    &fixture.runtime,
                    step + 3,
                    "rollback-live-update",
                    Some(FailureInjectionPoint::DuringBranchLocalWork),
                );
            }
            4 => {
                let active = fixture.runtime.current_branch();
                let snapshot_id = model
                    .branch(active.id)
                    .head_snapshot
                    .expect("branch restore should always have a branch-local head snapshot");
                let snapshot = snapshots
                    .get(&snapshot_id)
                    .expect("branch restore should use a stored branch-local snapshot")
                    .clone();
                fixture
                    .runtime
                    .restore_branch_snapshot(active.clone(), &snapshot)
                    .unwrap();
                let restored = model.snapshots.get(&snapshot.meta.snapshot_id).unwrap().clone();
                *model.branch_mut(active.id) = restored;
                model.branch_mut(active.id).head_snapshot =
                    fixture.runtime.branch_head_snapshot_id(active.id);
                harness.record(
                    &fixture.runtime,
                    step + 3,
                    "restore-current-branch",
                    Some(FailureInjectionPoint::DuringSnapshotRestoreChurn),
                );
            }
            _ => {
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&fintech_precompute(&fixture), executor)
                    .unwrap();
                harness.record(&fixture.runtime, step + 3, "drain-dirty", None);
            }
        }

        let report = assert_runtime_invariants(
            fixture.runtime.graph(),
            &model,
            fixture.runtime.current_branch(),
            fixture.ticks,
            ASPECT_A,
            fixture.volatility,
            ASPECT_B,
            policy,
        );
        if !report.errors.is_empty() {
            harness.panic_invariant(&policy, &format!("{executor:?}"), report);
        }
    }

    for branch in [main.clone(), what_if.clone(), correction.clone()] {
        fixture.runtime.switch_branch(branch.clone()).unwrap();
        let report = assert_runtime_invariants(
            fixture.runtime.graph(),
            &model,
            branch,
            fixture.ticks,
            ASPECT_A,
            fixture.volatility,
            ASPECT_B,
            policy,
        );
        if !report.errors.is_empty() {
            harness.panic_invariant(&policy, &format!("{executor:?}"), report);
        }
    }

    let replay = fixture
        .runtime
        .replay_for_branch(fixture.runtime.current_branch().id);
    let lineage = fixture.runtime.graph().lineage_for_node(fixture.risk);
    (harness, replay, lineage)
}

#[test]
fn geometry_kernel_adversarial_seed_matrix_keeps_invariants() {
    for (seed, workflow) in [
        (WorkflowSeed(7), AdversarialWorkflow::FeatureEditRewireRestoreChurn),
        (WorkflowSeed(19), AdversarialWorkflow::PartitionScopeCliffSession),
    ] {
        let _ = geometry_session(seed, workflow, SignalRuntimePolicy::kernel().with_history_limit(8), StageExecutor::Serial);
    }
}

#[test]
fn fintech_adversarial_seed_matrix_keeps_invariants() {
    for (seed, workflow) in [
        (WorkflowSeed(11), AdversarialWorkflow::LateTickCorrectionWithBranchReplay),
        (WorkflowSeed(23), AdversarialWorkflow::RiskAlertFlapUnderMemoChurn),
    ] {
        let _ = fintech_session(seed, workflow, SignalRuntimePolicy::fintech().with_history_limit(8), StageExecutor::Serial);
    }
}

#[test]
fn policy_overlap_for_generated_workflows_matches_guaranteed_truth() {
    for (domain, workflow, seed) in [
        (
            WorkflowDomain::GeometryKernel,
            AdversarialWorkflow::FeatureEditRewireRestoreChurn,
            WorkflowSeed(31),
        ),
        (
            WorkflowDomain::Fintech,
            AdversarialWorkflow::LateTickCorrectionWithBranchReplay,
            WorkflowSeed(37),
        ),
    ] {
        let runs = [
            (
                "operational",
                match domain {
                    WorkflowDomain::GeometryKernel => geometry_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::operational().with_history_limit(4),
                        StageExecutor::Serial,
                    ),
                    WorkflowDomain::Fintech => fintech_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::operational().with_history_limit(4),
                        StageExecutor::Serial,
                    ),
                },
            ),
            (
                "development",
                match domain {
                    WorkflowDomain::GeometryKernel => geometry_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::development().with_history_limit(6),
                        StageExecutor::Serial,
                    ),
                    WorkflowDomain::Fintech => fintech_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::development().with_history_limit(6),
                        StageExecutor::Serial,
                    ),
                },
            ),
            (
                "forensic",
                match domain {
                    WorkflowDomain::GeometryKernel => geometry_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::forensic().with_history_limit(8),
                        StageExecutor::Serial,
                    ),
                    WorkflowDomain::Fintech => fintech_session(
                        seed,
                        workflow,
                        SignalRuntimePolicy::forensic().with_history_limit(8),
                        StageExecutor::Serial,
                    ),
                },
            ),
        ];

        for pair in runs.windows(2) {
            let (_, (h1, replay1, lineage1)) = &pair[0];
            let (name2, (h2, replay2, lineage2)) = &pair[1];
            let replay_diff = compare_replay_slices(replay1, replay2);
            let lineage_diff = compare_lineage_records(lineage1, lineage2);
            if !replay_diff.is_empty() || !lineage_diff.is_empty() {
                h2.panic_diff(
                    &SignalRuntimePolicy::development(),
                    "serial",
                    format!("policy overlap drift against {name2}"),
                    replay_diff.mismatches.len(),
                    lineage_diff.mismatches.len(),
                );
            }
            let _ = h1;
        }
    }
}

#[cfg(feature = "parallel")]
#[test]
fn generated_workflows_preserve_canonical_truth_across_serial_and_parallel() {
    for (domain, workflow, seed) in [
        (
            WorkflowDomain::GeometryKernel,
            AdversarialWorkflow::FeatureEditRewireRestoreChurn,
            WorkflowSeed(41),
        ),
        (
            WorkflowDomain::Fintech,
            AdversarialWorkflow::RiskAlertFlapUnderMemoChurn,
            WorkflowSeed(53),
        ),
    ] {
        let serial = match domain {
            WorkflowDomain::GeometryKernel => geometry_session(
                seed,
                workflow,
                SignalRuntimePolicy::development().with_history_limit(8),
                StageExecutor::Serial,
            ),
            WorkflowDomain::Fintech => fintech_session(
                seed,
                workflow,
                SignalRuntimePolicy::development().with_history_limit(8),
                StageExecutor::Serial,
            ),
        };
        let parallel = match domain {
            WorkflowDomain::GeometryKernel => geometry_session(
                seed,
                workflow,
                SignalRuntimePolicy::development().with_history_limit(8),
                StageExecutor::aggressive_parallel(),
            ),
            WorkflowDomain::Fintech => fintech_session(
                seed,
                workflow,
                SignalRuntimePolicy::development().with_history_limit(8),
                StageExecutor::aggressive_parallel(),
            ),
        };

        let replay_diff = compare_replay_slices(&serial.1, &parallel.1);
        let lineage_diff = compare_lineage_records(&serial.2, &parallel.2);
        if !replay_diff.is_empty() || !lineage_diff.is_empty() {
            parallel.0.panic_diff(
                &SignalRuntimePolicy::development(),
                "serial-vs-parallel",
                "executor differential drift",
                replay_diff.mismatches.len(),
                lineage_diff.mismatches.len(),
            );
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventDomain {
    Audit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkflowEvent {
    Tick,
}

struct FailingFlushSubscriber;

impl EventSubscriber for FailingFlushSubscriber {
    type Event = WorkflowEvent;
    type DataId = EventDomain;
    type RuntimeContext = ();

    fn id(&self) -> SubscriberId {
        SubscriberId::new(901)
    }

    fn name(&self) -> &'static str {
        "adversarial-flush-failure"
    }

    fn requires(&self) -> &'static [Self::DataId] {
        &[]
    }

    fn provides(&self) -> &'static [Self::DataId] {
        &[]
    }

    fn on_event(&mut self, _event: &Self::Event) {}

    fn on_checkpoint(
        &mut self,
        _barrier: CheckpointBarrier,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        Err(SignalError::internal("synthetic flush failure"))
    }
}

#[test]
fn event_flush_failure_workflow_does_not_advance_branch_truth() {
    let mut runtime: EventRuntime = SignalRuntime::builder(SignalGraph::new())
        .with_domains::<EventDomain>()
        .with_events::<WorkflowEvent>()
        .runtime_policy(SignalRuntimePolicy::development().with_history_limit(4))
        .build();
    runtime
        .event_bus_mut()
        .subscribe(Box::new(FailingFlushSubscriber))
        .unwrap();

    let source = runtime.graph_mut().node().build();
    let baseline = runtime.capture_snapshot();
    let feature = runtime.create_branch("event-feature").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let head_before = runtime.branch_head_snapshot_id(feature.id);
    let replay_before = runtime.replay_for_branch(feature.id);

    let mut ctx = ();
    let mut tx = runtime.begin();
    tx.mark_dirty(source, ASPECT_A).unwrap();
    tx.emit_event(WorkflowEvent::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();
    let outcome = tx.commit(&mut ctx);
    assert!(outcome.is_err());

    assert_eq!(runtime.branch_head_snapshot_id(feature.id), head_before);
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        0
    );
    let replay_after = runtime.replay_for_branch(feature.id);
    assert!(
        replay_after.frames.len() >= replay_before.frames.len(),
        "flush failure should be visible without silently advancing branch truth"
    );
    runtime.switch_branch(runtime.branch_ancestry(feature.id).first().cloned().unwrap())
        .unwrap();
    runtime
        .restore_branch_snapshot(runtime.current_branch(), &baseline)
        .unwrap();
}

#[ignore]
#[test]
fn long_geometry_churn_seed_matrix_stays_hard_to_surprise() {
    for seed in [WorkflowSeed(71), WorkflowSeed(89), WorkflowSeed(97)] {
        let _ = geometry_session(
            seed,
            AdversarialWorkflow::FeatureEditRewireRestoreChurn,
            SignalRuntimePolicy::kernel().with_history_limit(12),
            StageExecutor::Serial,
        );
    }
}

#[ignore]
#[cfg(feature = "parallel")]
#[test]
fn long_fintech_parallel_churn_seed_matrix_stays_hard_to_surprise() {
    for seed in [WorkflowSeed(101), WorkflowSeed(131), WorkflowSeed(149)] {
        let _ = fintech_session(
            seed,
            AdversarialWorkflow::RiskAlertFlapUnderMemoChurn,
            SignalRuntimePolicy::fintech().with_history_limit(12),
            StageExecutor::aggressive_parallel(),
        );
    }
}
