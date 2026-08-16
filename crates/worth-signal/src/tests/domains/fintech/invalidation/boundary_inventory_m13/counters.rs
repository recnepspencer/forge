use super::ExactBoundarySymbol;

pub(super) const COUNTER_AND_EXPORT_BOUNDARIES: &[ExactBoundarySymbol] = &[
    boundary!(
        "derived frontier diagnostics writer",
        "logic/invalidation/routing/counters.rs",
        "../../../../../logic/invalidation/routing/counters.rs",
        "record_diagnostic_projection",
        1
    ),
    boundary!(
        "derived frontier diagnostics projection call",
        "logic/invalidation/routing/application.rs",
        "../../../../../logic/invalidation/routing/application.rs",
        "record_diagnostic_projection",
        2
    ),
    boundary!(
        "performed effect telemetry writer",
        "data/graph/runtime/effect/evidence.rs",
        "../../../../../data/graph/runtime/effect/evidence.rs",
        "record_effect_telemetry",
        1
    ),
    boundary!(
        "serial and parallel effect telemetry publication call",
        "data/graph/runtime/effect/output_commit.rs",
        "../../../../../data/graph/runtime/effect/output_commit.rs",
        "record_effect_telemetry",
        1
    ),
    boundary!(
        "public planning estimate owner",
        "data/proof/invalidation/plan.rs",
        "../../../../../data/proof/invalidation/plan.rs",
        "InvalidationPlanningEstimate",
        5
    ),
    boundary!(
        "internal diagnostics projection owner",
        "data/proof/invalidation/execution.rs",
        "../../../../../data/proof/invalidation/execution.rs",
        "FrontierDiagnosticsProjection",
        3
    ),
    boundary!(
        "internal diagnostics sidecar owner",
        "data/proof/invalidation/execution.rs",
        "../../../../../data/proof/invalidation/execution.rs",
        "FrontierDiagnosticsSidecar",
        3
    ),
    boundary!(
        "realized direct candidate telemetry",
        "data/telemetry/execution.rs",
        "../../../../../data/telemetry/execution.rs",
        "direct_subscriber_candidates_examined",
        1
    ),
    boundary!(
        "realized contract rejection telemetry",
        "data/telemetry/execution.rs",
        "../../../../../data/telemetry/execution.rs",
        "direct_contract_rejections",
        1
    ),
    boundary!(
        "realized causality rejection telemetry",
        "data/telemetry/execution.rs",
        "../../../../../data/telemetry/execution.rs",
        "direct_causality_rejections",
        1
    ),
    boundary!(
        "legacy node visit telemetry",
        "data/telemetry/execution.rs",
        "../../../../../data/telemetry/execution.rs",
        "invalidation_nodes_visited",
        1
    ),
    boundary!(
        "legacy transitive width telemetry",
        "data/telemetry/execution.rs",
        "../../../../../data/telemetry/execution.rs",
        "transitive_frontier_width",
        1
    ),
    boundary!(
        "transaction candidate telemetry aggregation",
        "logic/transaction/runtime/state/observer/metrics.rs",
        "../../../../../logic/transaction/runtime/state/observer/metrics.rs",
        "direct_subscriber_candidates_examined",
        6
    ),
    boundary!(
        "transaction contract telemetry aggregation",
        "logic/transaction/runtime/state/observer/metrics.rs",
        "../../../../../logic/transaction/runtime/state/observer/metrics.rs",
        "direct_contract_rejections",
        6
    ),
    boundary!(
        "transaction causality telemetry aggregation",
        "logic/transaction/runtime/state/observer/metrics.rs",
        "../../../../../logic/transaction/runtime/state/observer/metrics.rs",
        "direct_causality_rejections",
        6
    ),
    boundary!(
        "subscriber membership mutation owner",
        "data/graph/topology/subscriber_edges.rs",
        "../../../../../data/graph/topology/subscriber_edges.rs",
        "set_subscribers_sorted",
        3
    ),
    boundary!(
        "producer-only subscriber membership reader",
        "data/graph/topology/subscriber_edges.rs",
        "../../../../../data/graph/topology/subscriber_edges.rs",
        "raw_subscribers_of",
        3
    ),
    boundary!(
        "runtime facade dirty-batch export",
        "facade/runtime.rs",
        "../../../../../facade/runtime.rs",
        "mark_dirty_batch",
        1
    ),
];
