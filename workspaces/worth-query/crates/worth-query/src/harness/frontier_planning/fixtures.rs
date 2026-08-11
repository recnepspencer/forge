use worth_signal::facade::adapters::{
    FrontierEntryClassification, FrontierExecutionCounters, FrontierExecutionSummary,
    FrontierInclusionBasis, FrontierPlan, FrontierPredictedCounters, FrontierRouteEvidenceReason,
    FrontierRouteEvidenceReceipt, FrontierSeedCause, FrontierWaveEntryPlan,
    FrontierWaveEntrySummary, FrontierWavePlan, FrontierWaveSummary, InvalidationSeed,
    InvalidationSeedBatch, PartitionScopeSet, TouchedScopeSummary,
};
use worth_signal::facade::specialist::{EvaluationOutput, RunMode};
use worth_signal::facade::{
    Aspect, AspectVersion, NodeId, PartitionSubscription, SignalError, SignalGraph,
};

pub(super) fn sample_signal_frontier_plan() -> FrontierPlan {
    let seed = InvalidationSeed::new(
        NodeId::new(7, 0),
        Aspect::new(0),
        vec![PartitionSubscription::whole_partition("wing")],
        FrontierSeedCause::DirtySource,
    );
    let wave = FrontierWavePlan::new(
        0,
        Aspect::new(0),
        [FrontierWaveEntryPlan::new(
            NodeId::new(8, 0),
            FrontierEntryClassification::DirectDirty,
            FrontierInclusionBasis::PartitionScopeOverlap,
            vec![PartitionSubscription::whole_partition("wing")],
            [0],
        )],
    );

    FrontierPlan::new(
        InvalidationSeedBatch::new([seed]),
        vec![wave],
        Vec::new(),
        TouchedScopeSummary::new(
            PartitionScopeSet::new([PartitionSubscription::whole_partition("wing")]),
            vec![NodeId::new(7, 0), NodeId::new(8, 0)],
            vec![NodeId::new(7, 0)],
        ),
        FrontierPredictedCounters {
            seed_count: 1,
            group_count: 1,
            direct_wave_count: 1,
            transitive_wave_count: 0,
            direct_dirty_count: 1,
            maybe_stale_count: 0,
            partition_scoped_checks: 1,
            partition_match_count: 1,
            detail_match_count: 0,
            cycle_check_candidate_count: 0,
        },
    )
}

pub(super) fn sample_signal_frontier_summary() -> FrontierExecutionSummary {
    FrontierExecutionSummary::new(
        1,
        vec![FrontierWaveSummary::new(
            0,
            Aspect::new(0),
            [FrontierWaveEntrySummary::new(
                NodeId::new(8, 0),
                FrontierEntryClassification::DirectDirty,
                FrontierInclusionBasis::PartitionScopeOverlap,
                vec![PartitionSubscription::whole_partition("wing")],
            )],
        )],
        Vec::new(),
        TouchedScopeSummary::new(
            PartitionScopeSet::new([PartitionSubscription::whole_partition("wing")]),
            vec![NodeId::new(7, 0), NodeId::new(8, 0)],
            vec![NodeId::new(7, 0)],
        ),
        FrontierExecutionCounters {
            frontier_seed_count: 1,
            frontier_group_count: 1,
            frontier_direct_wave_count: 1,
            frontier_transitive_wave_count: 0,
            frontier_partition_scoped_check_count: 1,
            frontier_direct_dirty_count: 1,
            frontier_maybe_stale_count: 0,
            frontier_partition_match_count: 1,
            frontier_detail_match_count: 0,
            frontier_cycle_check_candidate_count: 0,
            frontier_cycle_check_visited_count: 0,
            frontier_trace_retained_count: 0,
        },
    )
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
