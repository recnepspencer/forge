use forge_harness::facade::{
    DiagnosticsHarnessAdapter, DiagnosticsLevel, ExecutionProfile, ExecutionRequest,
    ExplanationHarnessAdapter, HarnessAdapter, HarnessRunner, MutationBatch,
    ProvenanceHarnessAdapter, ReplayHarnessAdapter, ReplayRequest, ScenarioPlan,
};

use crate::facade::{
    mark_dirty, Aspect, AspectVersion, ExecutionReadView, NodeId, PreparedEvaluation,
    SignalEvaluationDriver, SignalFixtureFactory, SignalHarnessAdapter, SignalHarnessRuntime,
    SignalHarnessRuntimeBuilder, SignalMutationAction,
};

const ASPECT_A: Aspect = Aspect::new(0);

struct BasicEvaluator {
    source: NodeId,
}

impl SignalEvaluationDriver for BasicEvaluator {
    fn evaluate<'a>(
        &self,
        node: NodeId,
        _view: &ExecutionReadView<'a>,
    ) -> Result<PreparedEvaluation, crate::facade::SignalError> {
        if node == self.source {
            Ok(PreparedEvaluation::from_result(
                AspectVersion::from_updates([(ASPECT_A, 1)]),
            ))
        } else {
            Ok(PreparedEvaluation::from_result(
                AspectVersion::from_updates([(ASPECT_A, 2)]),
            ))
        }
    }
}

fn basic_fixture() -> forge_harness::facade::ScenarioFixture<SignalFixtureFactory> {
    ScenarioPlan::new(
        "signal-basic",
        SignalFixtureFactory::new(|| {
            let mut builder = SignalHarnessRuntimeBuilder::new();
            let source = builder.graph_mut().node().build();
            let dependent = builder.graph_mut().node().build();
            builder
                .graph_mut()
                .add_dependency(dependent, source, ASPECT_A)?;
            builder.insert_label("source", source);
            builder.insert_label("dependent", dependent);
            builder.set_evaluator(BasicEvaluator { source });
            builder.build()
        }),
    )
    .declare_input("source")
    .declare_observation("dependent")
    .compile()
}

#[test]
fn signal_harness_adapter_executes_serial_fixture() {
    let adapter = SignalHarnessAdapter;
    let runner = HarnessRunner::new(adapter);
    let fixture = basic_fixture();
    let mutation = MutationBatch::new("mark-source-dirty").push(SignalMutationAction::new(
        "dirty-source",
        |runtime: &mut SignalHarnessRuntime| {
            let source = runtime.resolve("source")?;
            mark_dirty(runtime.graph_mut(), source, ASPECT_A)
        },
    ));
    let request = ExecutionRequest::new("pull-dependent", vec!["dependent".to_string()]);
    let profile =
        ExecutionProfile::serial("serial").with_diagnostics_level(DiagnosticsLevel::Development);

    let bundle = runner
        .execute_core(&fixture, Some(&mutation), &request, &profile)
        .unwrap();

    assert_eq!(bundle.run.adapter_name, "forge-signal");
    assert_eq!(bundle.run.scenario_name, "signal-basic");
    assert_eq!(bundle.run.target_statuses.len(), 1);
    assert!(bundle.pre_snapshot.is_some());
    assert!(bundle.post_snapshot.is_some());
}

#[test]
fn signal_harness_adapter_captures_diagnostics_explanations_and_provenance() {
    let adapter = SignalHarnessAdapter;
    let fixture = basic_fixture();
    let request = ExecutionRequest::new("pull-dependent", vec!["dependent".to_string()]);
    let profile = ExecutionProfile::serial("serial");
    let mut session = adapter.create_runtime().unwrap();

    adapter.load_fixture(&mut session, &fixture).unwrap();
    let _run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .unwrap();

    let diagnostics = adapter
        .capture_diagnostics(&session, &fixture, &profile)
        .unwrap();
    let explanations = adapter
        .capture_explanations(&session, &fixture, &request, &profile)
        .unwrap();
    let provenance = adapter
        .capture_provenance(&session, &fixture, &request, &profile)
        .unwrap();

    assert_eq!(diagnostics.adapter_name, "forge-signal");
    assert_eq!(explanations.len(), 1);
    assert_eq!(provenance.len(), 1);
}

#[test]
fn signal_harness_adapter_captures_v2_replay_summary() {
    let adapter = SignalHarnessAdapter;
    let fixture = basic_fixture();
    let request = ExecutionRequest::new("pull-dependent", vec!["dependent".to_string()]);
    let profile = ExecutionProfile::serial("serial");
    let mut session = adapter.create_runtime().unwrap();

    adapter.load_fixture(&mut session, &fixture).unwrap();
    let run = adapter
        .execute(&mut session, &fixture, &request, &profile)
        .unwrap();

    let replay = adapter
        .capture_replay(
            &session,
            &fixture,
            &ReplayRequest {
                name: "replay-dependent".to_string(),
                source_run: run.clone(),
                request: request.clone(),
                profile: profile.clone(),
            },
        )
        .unwrap();

    assert_eq!(
        replay.schema_version,
        forge_harness::facade::RecordSchemaVersion::V2
    );
    assert_eq!(replay.requested_targets, request.targets);
    assert!(replay.summary.get("execution_report").is_some());
}
