use serde::Serialize;
use std::collections::BTreeMap;

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
        Self {
            state: seed.0.max(1),
        }
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
    history: &mut BTreeMap<SignalBranchId, Vec<SignalSnapshotId>>,
) -> SignalSnapshotV1
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    let branch = runtime.observe().current_branch();
    let snapshot = runtime.capture_branch_snapshot(branch.clone()).unwrap();
    let mut truth = model.branch(branch.id).clone();
    truth.head_snapshot = Some(snapshot.meta.snapshot_id);
    snapshots.insert(snapshot.meta.snapshot_id, snapshot.clone());
    history
        .entry(branch.id)
        .or_default()
        .push(snapshot.meta.snapshot_id);
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

fn trace_adv(message: impl AsRef<str>) {
    if std::env::var_os("FORGE_SIGNAL_TRACE_ADV").is_some() {
        eprintln!("{}", message.as_ref());
    }
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
    ) where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
        T: Copy + Ord,
    {
        self.steps.push(WorkflowStep {
            index,
            label: label.into(),
            active_branch: runtime.observe().current_branch().name,
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

fn geometry_evaluator(
    fixture: &GeometryFixture,
) -> impl for<'ctx> Fn(&mut crate::logic::context::EvaluationContext<'ctx, ()>) -> Result<crate::logic::evaluation::EvaluationOutput, SignalError>
       + Sync {
    let source_a = fixture.source_a;
    let source_b = fixture.source_b;
    let delta_gate = fixture.delta_gate;
    let filtered_gate = fixture.filtered_gate;
    let demand_gate = fixture.demand_gate;
    let fused = fixture.fused;
    move |ctx| {
        let node = ctx.node();
        if node == delta_gate {
            let a = ctx.read_aspect_version(source_a, ASPECT_A)?.get(ASPECT_A);
            return Ok(ctx.finish(
                NodeEvaluationResult::from_version(version_ab(a, 0))
                    .with_output_identity(format!("geom-delta-{a}"))
                    .with_continuity_token("geom-delta"),
            ));
        }
        if node == filtered_gate {
            let b = ctx.read_aspect_version(source_b, ASPECT_B)?.get(ASPECT_B);
            return Ok(ctx.finish(
                NodeEvaluationResult::from_version(version_ab(0, b))
                    .with_output_identity(format!("geom-filter-{b}"))
                    .with_continuity_token("geom-filter"),
            ));
        }
        if node == demand_gate {
            let a = ctx.read_aspect_version(source_a, ASPECT_A)?.get(ASPECT_A);
            return Ok(ctx.finish(
                NodeEvaluationResult::from_version(version_ab(a, 0))
                    .with_output_identity(format!("geom-demand-{a}"))
                    .with_continuity_token("geom-demand"),
            ));
        }
        if node == fused {
            let a = ctx.read_aspect_version(delta_gate, ASPECT_A)?.get(ASPECT_A);
            let b = ctx
                .read_aspect_version(filtered_gate, ASPECT_B)?
                .get(ASPECT_B);
            let demand = ctx.read_aspect_version(demand_gate, ASPECT_A)?.get(ASPECT_A);
            return Ok(ctx.finish(
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

fn fintech_evaluator(
    fixture: &FintechFixture,
) -> impl for<'ctx> Fn(&mut crate::logic::context::EvaluationContext<'ctx, ()>) -> Result<crate::logic::evaluation::EvaluationOutput, SignalError>
       + Sync {
    let ticks = fixture.ticks;
    let volatility = fixture.volatility;
    let throttle = fixture.throttle;
    let alert = fixture.alert;
    let risk = fixture.risk;
    move |ctx| {
        let node = ctx.node();
        if node == throttle {
            let a = ctx.read_aspect_version(ticks, ASPECT_A)?.get(ASPECT_A);
            return Ok(ctx.finish(
                NodeEvaluationResult::from_version(version_ab(a, 0))
                    .with_output_identity(format!("throttle-{a}"))
                    .with_continuity_token("throttle"),
            ));
        }
        if node == alert {
            let b = ctx
                .read_aspect_version(volatility, ASPECT_B)?
                .get(ASPECT_B);
            return Ok(ctx.finish(
                NodeEvaluationResult::from_version(version_ab(0, b))
                    .with_output_identity(format!("alert-{b}"))
                    .with_continuity_token("alert"),
            ));
        }
        if node == risk {
            let a = ctx.read_aspect_version(throttle, ASPECT_A)?.get(ASPECT_A);
            let b = ctx.read_aspect_version(alert, ASPECT_B)?.get(ASPECT_B);
            return Ok(ctx.finish(
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
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
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
        .reads_aspects(mask_a())
        .delta_threshold(2.0)
        .build();
    let filtered_gate = runtime
        .graph_mut()
        .node()
        .reads_aspects(mask_b())
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

    let family = define_keyed_computation(&mut runtime, "geom-workflow", ());
    let keyed_def = family.keyed("feature-panel");
    let keyed = keyed_def.node(&mut runtime);
    let memo_key = keyed_def.memoized("mesh-v1");

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
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(policy);

    let ticks = runtime.graph_mut().node().output_identity().build();
    let volatility = runtime.graph_mut().node().output_identity().build();
    let throttle = runtime
        .graph_mut()
        .node()
        .reads_aspects(mask_a())
        .delta_threshold(2.0)
        .build();
    let alert = runtime
        .graph_mut()
        .node()
        .reads_aspects(mask_b())
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
    runtime
        .graph_mut()
        .add_dependency(risk, throttle, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(risk, alert, ASPECT_B)
        .unwrap();

    let family = define_keyed_computation(&mut runtime, "fintech-workflow", ());
    let keyed_def = family.keyed("risk-book");
    let keyed = keyed_def.node(&mut runtime);
    let memo_key = keyed_def.memoized("risk-v1");

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
            tx.read(fixture.source_a, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("geom-source-a-1"),
                ))
            })?;
            tx.read(fixture.source_b, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(0, 1))
                        .with_output_identity("geom-source-b-1"),
                ))
            })?;
            tx.evaluate_keyed(
                fixture.keyed,
                &fixture.memo_key,
                &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 1))
                            .with_output_identity("geom-keyed-1")
                            .with_output_change(OutputChange::Refreshed),
                    ))
                },
            )?;
            Ok(())
        })
        .unwrap();

    fixture
        .runtime
        .evaluate_dirty_with_executor(&(), &geometry_evaluator(fixture), StageExecutor::Serial)
        .unwrap();
    fixture
        .runtime
        .evaluate_with_plan_and_executor(fixture.demand_gate, &(), &geometry_evaluator(fixture),
            EvaluationRequestMode::ForceOnDemand,
            StageExecutor::Serial,
        )
        .unwrap();

    let main = fixture.runtime.observe().current_branch();
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
            tx.read(fixture.ticks, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("tick-1"),
                ))
            })?;
            tx.read(
                fixture.volatility,
                &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(0, 1))
                            .with_output_identity("vol-1"),
                    ))
                },
            )?;
            tx.evaluate_keyed(
                fixture.keyed,
                &fixture.memo_key,
                &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 1))
                            .with_output_identity("risk-keyed-1")
                            .with_output_change(OutputChange::Refreshed),
                    ))
                },
            )?;
            Ok(())
        })
        .unwrap();

    fixture
        .runtime
        .evaluate_dirty_with_executor(&(), &fintech_evaluator(fixture), StageExecutor::Serial)
        .unwrap();

    let main = fixture.runtime.observe().current_branch();
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
                errors.push(format!(
                    "node {node} points at stale subscriber {subscriber}"
                ));
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
    if expected.head_snapshot.is_some()
        && graph.observe().branch_head_snapshot_id(current_branch.id).is_none()
    {
        errors.push(format!(
            "branch `{}` lost its head snapshot metadata",
            current_branch.name,
        ));
    }
    if graph.observe().recent_execution_history_diagnostics().len() > policy.history_limit {
        errors.push(format!(
            "history retention exceeded policy: {} > {}",
            graph.observe().recent_execution_history_diagnostics().len(),
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
    if !matches!(executor, StageExecutor::Serial) {
        trace_adv(format!(
            "[geometry {:?} seed={}] setup:start",
            workflow, seed.0
        ));
    }
    let mut harness = SignalAdversarialHarness::new(seed, WorkflowDomain::GeometryKernel, workflow);
    let mut fixture = build_geometry_fixture(policy);
    let mut model = ReferenceModel::default();
    let mut snapshots = BTreeMap::new();
    let mut branch_history = BTreeMap::new();
    let (main, main_snapshot) = seed_geometry_baseline(&mut fixture, &mut model);
    if !matches!(executor, StageExecutor::Serial) {
        trace_adv(format!(
            "[geometry {:?} seed={}] setup:seeded-main",
            workflow, seed.0
        ));
    }
    snapshots.insert(main_snapshot.meta.snapshot_id, main_snapshot.clone());
    branch_history.insert(main.id, vec![main_snapshot.meta.snapshot_id]);
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
        .capture_branch_snapshot(fixture.runtime.observe().current_branch())
        .unwrap();
    if !matches!(executor, StageExecutor::Serial) {
        trace_adv(format!(
            "[geometry {:?} seed={}] setup:feature-ready",
            workflow, seed.0
        ));
    }
    snapshots.insert(feature_snapshot.meta.snapshot_id, feature_snapshot.clone());
    branch_history
        .entry(feature.id)
        .or_default()
        .push(feature_snapshot.meta.snapshot_id);
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
        .capture_branch_snapshot(fixture.runtime.observe().current_branch())
        .unwrap();
    if !matches!(executor, StageExecutor::Serial) {
        trace_adv(format!(
            "[geometry {:?} seed={}] setup:analysis-ready",
            workflow, seed.0
        ));
    }
    snapshots.insert(
        analysis_snapshot.meta.snapshot_id,
        analysis_snapshot.clone(),
    );
    branch_history
        .entry(analysis.id)
        .or_default()
        .push(analysis_snapshot.meta.snapshot_id);
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
        if !matches!(executor, StageExecutor::Serial) {
            trace_adv(format!(
                "[geometry {:?} seed={}] step={} branch={}",
                workflow,
                seed.0,
                step,
                fixture.runtime.observe().current_branch().name
            ));
        }
        let action = match workflow {
            AdversarialWorkflow::FeatureEditRewireRestoreChurn => rng.choose(6),
            AdversarialWorkflow::PartitionScopeCliffSession => rng.choose(5),
            _ => rng.choose(6),
        };

        match action {
            0 => {
                let target =
                    [main.clone(), feature.clone(), analysis.clone()][rng.choose(3)].clone();
                fixture.runtime.switch_branch(target.clone()).unwrap();
                model.active = target.id;
                model.branch_mut(target.id).head_snapshot =
                    fixture.runtime.observe().branch_head_snapshot_id(target.id);
                harness.record(&fixture.runtime, step + 3, "switch-branch", None);
            }
            1 => {
                let delta = rng.small_delta();
                let current = model.branch(model.active).clone();
                let next_a = current.a + delta;
                let result = fixture
                    .runtime
                    .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                        tx.mark_dirty_with_regions(
                            fixture.source_a,
                            ASPECT_A,
                            &[ChangedRegion::new("wing").with_detail(format!("panel-{step}"))],
                        )?;
                        tx.read(
                            fixture.source_a,
                            &move |view| {
                                Ok(view.finish(
                                    NodeEvaluationResult::from_version(version_ab(next_a, 0))
                                        .with_output_identity(format!("geom-source-a-{next_a}")),
                                ))
                            },
                        )?;
                        Ok(())
                    });
                result.unwrap();
                model.branch_mut(model.active).a = next_a;
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&(), &geometry_evaluator(&fixture), executor)
                    .unwrap();
                if rng.coin() {
                    capture_active_branch_snapshot(
                        &mut fixture.runtime,
                        &mut model,
                        &mut snapshots,
                        &mut branch_history,
                    );
                }
                harness.record(&fixture.runtime, step + 3, "update-source-a", None);
            }
            2 => {
                let delta = rng.small_delta();
                let current = model.branch(model.active).clone();
                let next_b = current.b + delta;
                let result = fixture
                    .runtime
                    .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                        tx.mark_dirty_with_regions(
                            fixture.source_b,
                            ASPECT_B,
                            &[ChangedRegion::new("lod")],
                        )?;
                        tx.read(
                            fixture.source_b,
                            &move |view| {
                                Ok(view.finish(
                                    NodeEvaluationResult::from_version(version_ab(0, next_b))
                                        .with_output_identity(format!("geom-source-b-{next_b}")),
                                ))
                            },
                        )?;
                        Ok(())
                    });
                result.unwrap();
                model.branch_mut(model.active).b = next_b;
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&(), &geometry_evaluator(&fixture), executor)
                    .unwrap();
                if rng.coin() {
                    capture_active_branch_snapshot(
                        &mut fixture.runtime,
                        &mut model,
                        &mut snapshots,
                        &mut branch_history,
                    );
                }
                harness.record(&fixture.runtime, step + 3, "update-source-b", None);
            }
            3 => {
                let delta = rng.small_delta();
                let current = model.branch(model.active).clone();
                let bad_a = current.a + delta;
                let err = fixture
                    .runtime
                    .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                        tx.mark_dirty(fixture.source_a, ASPECT_A)?;
                        tx.read(
                            fixture.source_a,
                            &move |view| {
                                Ok(view.finish(
                                    NodeEvaluationResult::from_version(version_ab(bad_a, 0))
                                        .with_output_identity(format!("geom-source-a-bad-{bad_a}")),
                                ))
                            },
                        )?;
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
                    .evaluate_with_plan_and_executor(fixture.demand_gate, &(), &geometry_evaluator(&fixture),
                        EvaluationRequestMode::ForceOnDemand,
                        executor,
                    )
                    .unwrap();
                harness.record(&fixture.runtime, step + 3, "force-on-demand", None);
            }
            _ => {
                let active = fixture.runtime.observe().current_branch();
                let candidates = branch_history
                    .get(&active.id)
                    .expect("branch restore should always have branch-local snapshot history");
                let snapshot_id = candidates[rng.choose(candidates.len())];
                let restore_snapshot = snapshots
                    .get(&snapshot_id)
                    .expect("branch restore should use a stored branch-local snapshot")
                    .clone();
                let restore_snapshot = if restore_snapshot.meta.branch_id == active.id {
                    restore_snapshot
                } else {
                    capture_active_branch_snapshot(
                        &mut fixture.runtime,
                        &mut model,
                        &mut snapshots,
                        &mut branch_history,
                    )
                };
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
                    fixture.runtime.observe().branch_head_snapshot_id(active.id);
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
            fixture.runtime.observe().current_branch(),
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
        .replay_for_branch(fixture.runtime.observe().current_branch().id);
    let lineage = fixture.runtime.graph().observe().lineage_for_node(fixture.fused);
    (harness, replay, lineage)
}

fn fintech_session(
    seed: WorkflowSeed,
    workflow: AdversarialWorkflow,
    policy: SignalRuntimePolicy,
    executor: StageExecutor,
) -> (SignalAdversarialHarness, ReplaySlice, Vec<LineageRecord>) {
    if !matches!(executor, StageExecutor::Serial) {
        trace_adv(format!(
            "[fintech {:?} seed={}] setup:start",
            workflow, seed.0
        ));
    }
    let mut harness = SignalAdversarialHarness::new(seed, WorkflowDomain::Fintech, workflow);
    let mut fixture = build_fintech_fixture(policy);
    let mut model = ReferenceModel::default();
    let mut snapshots = BTreeMap::new();
    let mut branch_history = BTreeMap::new();
    let (main, main_snapshot) = seed_fintech_baseline(&mut fixture, &mut model);
    if !matches!(executor, StageExecutor::Serial) {
        trace_adv(format!(
            "[fintech {:?} seed={}] setup:seeded-main",
            workflow, seed.0
        ));
    }
    snapshots.insert(main_snapshot.meta.snapshot_id, main_snapshot.clone());
    branch_history.insert(main.id, vec![main_snapshot.meta.snapshot_id]);
    harness.record(&fixture.runtime, 0, "seed-main-baseline", None);

    let what_if = fixture.runtime.create_branch("what-if").unwrap();
    model
        .branches
        .insert(what_if.id, model.branch(main.id).clone());
    fixture.runtime.switch_branch(what_if.clone()).unwrap();
    model.active = what_if.id;
    let what_if_snapshot = fixture
        .runtime
        .capture_branch_snapshot(fixture.runtime.observe().current_branch())
        .unwrap();
    if !matches!(executor, StageExecutor::Serial) {
        trace_adv(format!(
            "[fintech {:?} seed={}] setup:what-if-ready",
            workflow, seed.0
        ));
    }
    snapshots.insert(what_if_snapshot.meta.snapshot_id, what_if_snapshot.clone());
    branch_history
        .entry(what_if.id)
        .or_default()
        .push(what_if_snapshot.meta.snapshot_id);
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
        .capture_branch_snapshot(fixture.runtime.observe().current_branch())
        .unwrap();
    if !matches!(executor, StageExecutor::Serial) {
        trace_adv(format!(
            "[fintech {:?} seed={}] setup:correction-ready",
            workflow, seed.0
        ));
    }
    snapshots.insert(
        correction_snapshot.meta.snapshot_id,
        correction_snapshot.clone(),
    );
    branch_history
        .entry(correction.id)
        .or_default()
        .push(correction_snapshot.meta.snapshot_id);
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
        if !matches!(executor, StageExecutor::Serial) {
            trace_adv(format!(
                "[fintech {:?} seed={}] step={} branch={}",
                workflow,
                seed.0,
                step,
                fixture.runtime.observe().current_branch().name
            ));
        }
        let action = match workflow {
            AdversarialWorkflow::LateTickCorrectionWithBranchReplay => rng.choose(6),
            AdversarialWorkflow::RiskAlertFlapUnderMemoChurn => rng.choose(5),
            _ => rng.choose(6),
        };

        match action {
            0 => {
                let target =
                    [main.clone(), what_if.clone(), correction.clone()][rng.choose(3)].clone();
                fixture.runtime.switch_branch(target.clone()).unwrap();
                model.active = target.id;
                model.branch_mut(target.id).head_snapshot =
                    fixture.runtime.observe().branch_head_snapshot_id(target.id);
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
                        tx.read(
                            fixture.ticks,
                            &move |view| {
                                Ok(view.finish(
                                    NodeEvaluationResult::from_version(version_ab(next_a, 0))
                                        .with_output_identity(format!("ticks-{next_a}")),
                                ))
                            },
                        )?;
                        tx.evaluate_keyed(
                            fixture.keyed,
                            &fixture.memo_key,
                            &|view| {
                                Ok(view.finish(
                                    NodeEvaluationResult::from_version(version_ab(
                                        next_a, current.b,
                                    ))
                                    .with_output_identity(format!(
                                        "risk-keyed-{next_a}-{}",
                                        current.b
                                    ))
                                    .with_output_change(OutputChange::Refreshed),
                                ))
                            },
                        )?;
                        Ok(())
                    })
                    .unwrap();
                model.branch_mut(model.active).a = next_a;
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&(), &fintech_evaluator(&fixture), executor)
                    .unwrap();
                if rng.coin() {
                    capture_active_branch_snapshot(
                        &mut fixture.runtime,
                        &mut model,
                        &mut snapshots,
                        &mut branch_history,
                    );
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
                        tx.read(
                            fixture.volatility,
                            &move |view| {
                                Ok(view.finish(
                                    NodeEvaluationResult::from_version(version_ab(0, next_b))
                                        .with_output_identity(format!("volatility-{next_b}")),
                                ))
                            },
                        )?;
                        Ok(())
                    })
                    .unwrap();
                model.branch_mut(model.active).b = next_b;
                fixture
                    .runtime
                    .evaluate_dirty_with_executor(&(), &fintech_evaluator(&fixture), executor)
                    .unwrap();
                if rng.coin() {
                    capture_active_branch_snapshot(
                        &mut fixture.runtime,
                        &mut model,
                        &mut snapshots,
                        &mut branch_history,
                    );
                }
                harness.record(&fixture.runtime, step + 3, "update-volatility", None);
            }
            3 => {
                let current = model.branch(model.active).clone();
                let err = fixture
                    .runtime
                    .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
                        tx.mark_dirty(fixture.ticks, ASPECT_A)?;
                        tx.read(
                            fixture.ticks,
                            &move |view| {
                                Ok(view.finish(
                                    NodeEvaluationResult::from_version(version_ab(
                                        current.a + 10,
                                        0,
                                    ))
                                    .with_output_identity("bad-ticks"),
                                ))
                            },
                        )?;
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
                let active = fixture.runtime.observe().current_branch();
                let candidates = branch_history
                    .get(&active.id)
                    .expect("branch restore should always have branch-local snapshot history");
                let snapshot_id = candidates[rng.choose(candidates.len())];
                let snapshot = snapshots
                    .get(&snapshot_id)
                    .expect("branch restore should use a stored branch-local snapshot")
                    .clone();
                let snapshot = if snapshot.meta.branch_id == active.id {
                    snapshot
                } else {
                    capture_active_branch_snapshot(
                        &mut fixture.runtime,
                        &mut model,
                        &mut snapshots,
                        &mut branch_history,
                    )
                };
                fixture
                    .runtime
                    .restore_branch_snapshot(active.clone(), &snapshot)
                    .unwrap();
                let restored = model
                    .snapshots
                    .get(&snapshot.meta.snapshot_id)
                    .unwrap()
                    .clone();
                *model.branch_mut(active.id) = restored;
                model.branch_mut(active.id).head_snapshot =
                    fixture.runtime.observe().branch_head_snapshot_id(active.id);
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
                    .evaluate_dirty_with_executor(&(), &fintech_evaluator(&fixture), executor)
                    .unwrap();
                harness.record(&fixture.runtime, step + 3, "drain-dirty", None);
            }
        }

        let report = assert_runtime_invariants(
            fixture.runtime.graph(),
            &model,
            fixture.runtime.observe().current_branch(),
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
        .replay_for_branch(fixture.runtime.observe().current_branch().id);
    let lineage = fixture.runtime.graph().observe().lineage_for_node(fixture.risk);
    (harness, replay, lineage)
}

#[test]
fn geometry_kernel_adversarial_seed_matrix_keeps_invariants() {
    for (seed, workflow) in [
        (
            WorkflowSeed(7),
            AdversarialWorkflow::FeatureEditRewireRestoreChurn,
        ),
        (
            WorkflowSeed(19),
            AdversarialWorkflow::PartitionScopeCliffSession,
        ),
    ] {
        let _ = geometry_session(
            seed,
            workflow,
            SignalRuntimePolicy::kernel().with_history_limit(8),
            StageExecutor::Serial,
        );
    }
}

#[test]
fn fintech_adversarial_seed_matrix_keeps_invariants() {
    for (seed, workflow) in [
        (
            WorkflowSeed(11),
            AdversarialWorkflow::LateTickCorrectionWithBranchReplay,
        ),
        (
            WorkflowSeed(23),
            AdversarialWorkflow::RiskAlertFlapUnderMemoChurn,
        ),
    ] {
        let _ = fintech_session(
            seed,
            workflow,
            SignalRuntimePolicy::fintech().with_history_limit(8),
            StageExecutor::Serial,
        );
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
            if !replay_diff.is_empty() {
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
fn parallel_geometry_hostile_session_matches_serial_truth() {
    trace_adv("[parallel-test] geometry:start");
    let workflow = AdversarialWorkflow::FeatureEditRewireRestoreChurn;
    let seed = WorkflowSeed(41);
    let serial = geometry_session(
        seed,
        workflow,
        SignalRuntimePolicy::development().with_history_limit(8),
        StageExecutor::Serial,
    );
    trace_adv("[parallel-test] geometry:serial-finished");
    let parallel = geometry_session(
        seed,
        workflow,
        SignalRuntimePolicy::development().with_history_limit(8),
        StageExecutor::aggressive_parallel(),
    );
    trace_adv("[parallel-test] geometry:parallel-finished");

    let replay_diff = compare_replay_slices(&serial.1, &parallel.1);
    let lineage_diff = compare_lineage_records(&serial.2, &parallel.2);
    if !replay_diff.is_empty() || !lineage_diff.is_empty() {
        parallel.0.panic_diff(
            &SignalRuntimePolicy::development(),
            "serial-vs-parallel",
            "geometry executor differential drift",
            replay_diff.mismatches.len(),
            lineage_diff.mismatches.len(),
        );
    }
}

#[cfg(feature = "parallel")]
#[test]
fn parallel_fintech_hostile_session_matches_serial_truth() {
    trace_adv("[parallel-test] fintech:start");
    let workflow = AdversarialWorkflow::RiskAlertFlapUnderMemoChurn;
    let seed = WorkflowSeed(53);
    let serial = fintech_session(
        seed,
        workflow,
        SignalRuntimePolicy::development().with_history_limit(8),
        StageExecutor::Serial,
    );
    trace_adv("[parallel-test] fintech:serial-finished");
    let parallel = fintech_session(
        seed,
        workflow,
        SignalRuntimePolicy::development().with_history_limit(8),
        StageExecutor::aggressive_parallel(),
    );
    trace_adv("[parallel-test] fintech:parallel-finished");

    let replay_diff = compare_replay_slices(&serial.1, &parallel.1);
    let lineage_diff = compare_lineage_records(&serial.2, &parallel.2);
    if !replay_diff.is_empty() || !lineage_diff.is_empty() {
        parallel.0.panic_diff(
            &SignalRuntimePolicy::development(),
            "serial-vs-parallel",
            "fintech executor differential drift",
            replay_diff.mismatches.len(),
            lineage_diff.mismatches.len(),
        );
    }
}

#[cfg(feature = "parallel")]
#[test]
fn focused_parallel_branch_restore_and_evaluate_dirty_regression() {
    trace_adv("[parallel-test] focused-regression:start");
    let mut fixture =
        build_geometry_fixture(SignalRuntimePolicy::development().with_history_limit(8));
    let mut model = ReferenceModel::default();
    let (main, main_snapshot) = seed_geometry_baseline(&mut fixture, &mut model);
    trace_adv("[parallel-test] focused-regression:seeded-main");

    let feature = fixture.runtime.create_branch("feature").unwrap();
    fixture.runtime.switch_branch(feature.clone()).unwrap();
    trace_adv("[parallel-test] focused-regression:feature-branch");

    let mut ctx = ();
    fixture
        .runtime
        .transaction(&mut ctx, |tx: &mut DefaultTx<'_>| {
            tx.mark_dirty(fixture.source_a, ASPECT_A)?;
            tx.read(fixture.source_a, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(4, 1))
                        .with_output_identity("source-a-4"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    trace_adv("[parallel-test] focused-regression:mutated-feature");

    fixture
        .runtime
        .evaluate_dirty_with_executor(&(), 
            &geometry_evaluator(&fixture),
            StageExecutor::aggressive_parallel(),
        )
        .unwrap();
    trace_adv("[parallel-test] focused-regression:parallel-evaluated");

    let feature_snapshot = fixture.runtime.capture_snapshot();
    fixture
        .runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .unwrap();
    trace_adv("[parallel-test] focused-regression:feature-restored");

    fixture.runtime.switch_branch(main.clone()).unwrap();
    fixture
        .runtime
        .restore_branch_snapshot(main, &main_snapshot)
        .unwrap();
    trace_adv("[parallel-test] focused-regression:main-restored");

    let replay = fixture.runtime.observe().replay_for_branch(feature.id);
    assert!(
        !replay.frames.is_empty(),
        "parallel branch restore regression should leave observable replay"
    );
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
        &[EventDomain::Audit]
    }

    fn provides(&self) -> &'static [Self::DataId] {
        &[EventDomain::Audit]
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
    assert!(matches!(
        FailureInjectionPoint::DuringEventFlush,
        FailureInjectionPoint::DuringEventFlush
    ));
    let mut runtime: EventRuntime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults()
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
    let head_before = runtime.observe().branch_head_snapshot_id(feature.id);
    let replay_before = runtime.observe().replay_for_branch(feature.id);

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    tx.mark_dirty(source, ASPECT_A).unwrap();
    tx.emit_event(WorkflowEvent::Tick);
    tx.flush_events(CheckpointBarrier::PerOperation).unwrap();
    let outcome = tx.commit();
    assert!(outcome.is_err());

    assert_eq!(runtime.observe().branch_head_snapshot_id(feature.id), head_before);
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        0
    );
    let replay_after = runtime.observe().replay_for_branch(feature.id);
    assert!(
        replay_after.frames.len() >= replay_before.frames.len(),
        "flush failure should be visible without silently advancing branch truth"
    );
    runtime
        .switch_branch(
            runtime
                .branch_ancestry(feature.id)
                .first()
                .cloned()
                .unwrap(),
        )
        .unwrap();
    runtime
        .restore_branch_snapshot(runtime.observe().current_branch(), &baseline)
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
