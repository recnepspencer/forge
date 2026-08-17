use crate::frontier_signal_adapter::SignalFrontierSurfaceEvidence;
use worth_signal::facade::adapters::{
    FrontierRouteEvidenceReason, FrontierRouteEvidenceReceipt, InvalidationPlanningEstimate,
    SignalInvalidationExecutionReceipt,
};
use worth_signal::facade::specialist::{EvaluationOutput, RunMode};
use worth_signal::facade::{
    mark_dirty, Aspect, AspectVersion, DependencyEdge, SignalError, SignalGraph, SignalRuntime,
};

pub(super) fn sample_signal_planning_surface() -> SignalFrontierSurfaceEvidence {
    let (estimate, _) = performed_signal_frontier_evidence();
    SignalFrontierSurfaceEvidence::from_invalidation_planning_estimate(&estimate)
}

pub(super) fn sample_signal_execution_surface() -> SignalFrontierSurfaceEvidence {
    let (_, receipt) = performed_signal_frontier_evidence();
    SignalFrontierSurfaceEvidence::from_invalidation_execution_receipt(&receipt)
}

fn performed_signal_frontier_evidence() -> (
    InvalidationPlanningEstimate,
    SignalInvalidationExecutionReceipt,
) {
    let aspect = Aspect::new(0);
    let mut graph = SignalGraph::new();
    let source = graph.node().produces_aspects([aspect]).build();
    let dependent = graph.node().reads_aspects([aspect]).build();
    graph
        .set_dependencies(dependent, [DependencyEdge::new(source, aspect)])
        .expect("sample Signal dependency should install");
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    runtime
        .evaluate_dirty(&(), &|_| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(signal_version(1)))
        })
        .expect("sample Signal graph should initialize");
    let (_, receipt) = runtime
        .observe_invalidation_execution(|runtime| {
            mark_dirty(runtime.graph_mut(), source, aspect)?;
            runtime.evaluate_dirty(&(), &|_| {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(signal_version(
                    2,
                )))
            })
        })
        .expect("sample Signal invalidation should execute");
    let estimate = runtime
        .graph()
        .observe()
        .latest_invalidation_planning_estimate()
        .cloned()
        .expect("sample invalidation should retain its public planning estimate");
    (estimate, receipt)
}

pub(super) fn sample_stage_execution_record(
    reason: FrontierRouteEvidenceReason,
) -> FrontierRouteEvidenceReceipt {
    FrontierRouteEvidenceReceipt::from_reason(reason)
}

pub(super) fn runtime_signal_stage_execution_record() -> FrontierRouteEvidenceReceipt {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let bootstrap = graph
        .build_evaluation_plan(&[node], RunMode::ForceOnDemand)
        .expect("force-on-demand bootstrap should build");
    graph
        .execute_prepared_plan(&bootstrap, &(), &|_ctx| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(signal_version(1)))
        })
        .expect("bootstrap evaluation should succeed");

    let plan = graph
        .build_evaluation_plan(&[node], RunMode::ForceOnDemand)
        .expect("second force-on-demand plan should build");
    let report = graph
        .execute_prepared_plan(&plan, &(), &|ctx| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(signal_version(
                ctx.node().index() as u64 + 2,
            )))
        })
        .expect("runtime plan should execute");

    FrontierRouteEvidenceReceipt::from_stage_execution_record(
        report
            .stages
            .first()
            .expect("runtime execution report should record one stage"),
    )
    .expect("runtime stage record should lower into a signal facade route receipt")
}

pub(super) fn signal_version(revision: u64) -> AspectVersion {
    AspectVersion::from_updates([(Aspect::new(0), revision)])
}
