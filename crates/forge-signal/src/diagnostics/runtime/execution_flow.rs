use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::flow::{
    ApplySummary, ChangeInputSummary, FlowCauseSample, FlowSummary, InvalidationSummary,
    PlanningSummary, PrecomputeSummary,
};
use crate::diagnostics::policy::OrdinaryAccessLane;
use crate::diagnostics::replay::{ReplayEvent, ReplayEventDetail, ReplayEventKind};
use crate::diagnostics::summary::{
    EvaluationPlanSummary, ExecutionHistorySummary, ExplanationSummary, GraphSummary,
};
use crate::logic::explain::CausalLinkKind;
use crate::logic::planner::{ExecutionReport, TaskExecutionOutcome};

pub(crate) fn record_semantic_execution(
    graph: &mut SignalGraph,
    plan_summary: EvaluationPlanSummary,
    first_target: Option<NodeId>,
    report: &ExecutionReport,
) {
    let runtime_policy = graph.runtime_policy();
    let retention_budget = runtime_policy.retention_budget;
    let profile = runtime_policy.tier;
    let (change, invalidation) = graph
        .diagnostics_state()
        .pending_change_summary()
        .unwrap_or_else(|| {
            (
                ChangeInputSummary::new(Vec::new(), Vec::new(), 0, None),
                InvalidationSummary::empty_frontier(),
            )
        });
    let explanation = if retention_budget.retain_flow_explanation {
        first_target
            .as_ref()
            .and_then(|target| graph.observe().explain(*target).ok())
            .map(|explanation| ExplanationSummary::from_explanation(&explanation, profile))
    } else {
        None
    };
    let flow = FlowSummary::new(
        profile,
        change,
        invalidation,
        PlanningSummary::from_summary(plan_summary),
        PrecomputeSummary::from_report(report, profile),
        ApplySummary::from_report(report, profile),
        if retention_budget.retain_stage_details {
            sample_flow_causes(graph, report, retention_budget.detail_limit.get())
        } else {
            Vec::new()
        },
        Vec::new(),
        None,
        explanation,
    );
    let graph_summary = GraphSummary::from_graph(
        graph,
        profile,
        retention_budget.detail_limit,
        OrdinaryAccessLane,
    );
    let history = if execution_history_unchanged(report) {
        graph
            .diagnostics_state()
            .recent_history()
            .back()
            .cloned()
            .unwrap_or_else(|| {
                ExecutionHistorySummary::from_graph(
                    graph,
                    profile,
                    retention_budget.detail_limit,
                    retention_budget.retain_history_details,
                    OrdinaryAccessLane,
                )
            })
    } else {
        ExecutionHistorySummary::from_graph(
            graph,
            profile,
            retention_budget.detail_limit,
            retention_budget.retain_history_details,
            OrdinaryAccessLane,
        )
    };
    graph
        .diagnostics_state_mut()
        .complete_flow(flow, history, graph_summary);
    for task in report
        .stages
        .iter()
        .flat_map(|stage| stage.task_records.iter())
    {
        let cursor = graph.diagnostics_state_mut().allocate_replay_cursor();
        let branch_id = graph.observe().current_branch().id;
        let lineage_artifact_id = graph.node_lineage_artifact_id(task.node).ok().flatten();
        let persistent_correspondence_kind = graph
            .node_reuse_boundary_authority(task.node)
            .ok()
            .flatten()
            .and_then(|authority| authority.persistent_correspondence_kind());
        let composition_region_count = graph
            .node_reuse_boundary_authority(task.node)
            .ok()
            .flatten()
            .map(|authority| authority.composition_region_count())
            .filter(|count| *count > 0);
        graph
            .diagnostics_state_mut()
            .record_replay_event(ReplayEvent::new(
                cursor,
                ReplayEventKind::TaskApplied,
                branch_id,
                None,
                Some(task.node),
                Some(task.id.0),
                Some(task.semantic_segment_id.0),
                lineage_artifact_id,
                Some(task.reuse_origin),
                persistent_correspondence_kind,
                composition_region_count,
                Some(ReplayEventDetail::TaskOutcome(task.outcome)),
            ));
    }
}

fn sample_flow_causes(
    graph: &SignalGraph,
    report: &ExecutionReport,
    limit: usize,
) -> Vec<FlowCauseSample> {
    let mut nodes = Vec::new();
    for task in report
        .stages
        .iter()
        .flat_map(|stage| stage.task_records.iter())
        .map(|record| record.node)
    {
        if !nodes.contains(&task) {
            nodes.push(task);
        }
        if nodes.len() >= limit {
            break;
        }
    }

    let mut samples = Vec::new();
    for node in nodes {
        let Ok(explanation) = graph.observe().explain(node) else {
            continue;
        };
        let mut suspect_classes = Vec::new();
        if explanation.rewiring.is_some() {
            suspect_classes.push("rewiring".to_string());
        }
        if explanation.causal_links.iter().any(|link| {
            matches!(
                link.scope.kind,
                crate::logic::explain::ScopeProvenanceKind::Direct
                    | crate::logic::explain::ScopeProvenanceKind::Translated
                    | crate::logic::explain::ScopeProvenanceKind::Discarded
                    | crate::logic::explain::ScopeProvenanceKind::InsufficientEvidence
            )
        }) {
            suspect_classes.push("locality".to_string());
        }
        if explanation.causal_links.iter().any(|link| {
            matches!(
                link.disposition,
                crate::logic::explain::CausalDisposition::Conservative
            ) || matches!(
                link.kind,
                CausalLinkKind::SkippedByComparator | CausalLinkKind::ConditionDeferred { .. }
            )
        }) && !suspect_classes.contains(&"validation".to_string())
        {
            suspect_classes.push("validation".to_string());
        }
        samples.push(FlowCauseSample {
            node,
            cause_kinds: explanation
                .causal_links
                .iter()
                .take(limit)
                .map(|link| link.kind.to_string())
                .collect(),
            scope_kinds: explanation
                .causal_links
                .iter()
                .filter(|link| {
                    !matches!(
                        link.scope.kind,
                        crate::logic::explain::ScopeProvenanceKind::None
                    )
                })
                .take(limit)
                .map(|link| format!("{:?}", link.scope.kind))
                .collect(),
            scope_notes: explanation
                .causal_links
                .iter()
                .filter_map(|link| link.note.clone())
                .take(limit)
                .collect(),
            suspect_classes,
            rewired: explanation.rewiring.is_some(),
            conservative_recompute: explanation.causal_links.iter().any(|link| {
                matches!(
                    link.disposition,
                    crate::logic::explain::CausalDisposition::Conservative
                )
            }),
        });
    }
    samples
}

fn execution_history_unchanged(report: &ExecutionReport) -> bool {
    report
        .stages
        .iter()
        .flat_map(|stage| &stage.task_records)
        .all(|task| {
            matches!(
                task.outcome,
                TaskExecutionOutcome::ValidatedClean
                    | TaskExecutionOutcome::ConditionDeferred
                    | TaskExecutionOutcome::ConditionRevertedClean
                    | TaskExecutionOutcome::Pruned
            )
        })
}
