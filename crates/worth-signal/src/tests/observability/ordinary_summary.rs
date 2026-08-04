use super::runtime_world::build_runtime;
use crate::facade::{
    render_execution_history_summary, DiagnosticsTier, NodeId, SignalGraph, SignalRuntimePolicy,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A};

#[test]
fn ordinary_summary_surfaces_do_not_trigger_artifact_reconstruction() {
    let mut runtime = build_runtime(SignalGraph::new());
    runtime.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = runtime.graph_mut().node().build();
    let dependent = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(runtime.graph_mut(), source, &mut compute).unwrap();
    evaluate(runtime.graph_mut(), dependent, &mut compute).unwrap();

    let before = runtime
        .observe()
        .metrics()
        .storage
        .hot_path_artifact_reconstruction_count;

    let diagnostics = runtime.observe().diagnostics();
    let _graph_summary = diagnostics.summary(DiagnosticsTier::Operational);
    let history = diagnostics.history(DiagnosticsTier::Operational);
    let _recent = diagnostics.recent_history();
    let _replay = runtime.graph().replay_events();
    let rendered = render_execution_history_summary(&history);

    let after = runtime
        .observe()
        .metrics()
        .storage
        .hot_path_artifact_reconstruction_count;
    assert_eq!(
        before, after,
        "ordinary diagnostics/history/replay reads must not trigger artifact reconstruction"
    );
    assert!(rendered.contains("ExecutionHistorySummary"));
}
