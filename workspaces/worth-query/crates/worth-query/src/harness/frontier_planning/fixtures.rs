use worth_signal::facade::adapters::{
    FrontierRouteEvidenceReason, FrontierRouteEvidenceReceipt, InvalidationPlanningEstimate,
    SignalInvalidationExecutionReceipt,
};
use worth_signal::facade::specialist::{EvaluationOutput, RunMode};
use worth_signal::facade::{Aspect, AspectVersion, SignalError, SignalGraph};

pub(super) fn sample_signal_planning_estimate() -> InvalidationPlanningEstimate {
    InvalidationPlanningEstimate::default()
}

pub(super) fn sample_signal_execution_receipt() -> SignalInvalidationExecutionReceipt {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let observation = graph.begin_invalidation_execution_observation();
    let plan = graph
        .build_evaluation_plan(&[node], RunMode::ForceOnDemand)
        .expect("force-on-demand sample plan should build");
    graph
        .execute_prepared_plan(&plan, &(), &|_ctx| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(signal_version(1)))
        })
        .expect("sample invalidation execution should succeed");
    graph
        .finish_invalidation_execution_observation(observation)
        .expect("performed sample invalidation should mint a receipt")
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
