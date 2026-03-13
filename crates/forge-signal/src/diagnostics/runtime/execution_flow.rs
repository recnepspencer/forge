use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::diagnostics::flow::{
    ApplySummary, ChangeInputSummary, FlowCauseSample, FlowSummary, InvalidationSummary,
    PlanningSummary, PrecomputeSummary,
};
use crate::diagnostics::policy::DiagnosticsPolicy;
use crate::diagnostics::replay::{ReplayEvent, ReplayEventKind};
use crate::diagnostics::summary::{
    EvaluationPlanSummary, ExecutionHistorySummary, ExplanationSummary,
};
use crate::logic::planner::{ExecutionReport, TaskExecutionOutcome};

pub(crate) fn record_semantic_execution(
    graph: &mut SignalGraph,
    plan_summary: EvaluationPlanSummary,
    first_target: Option<NodeId>,
    report: &ExecutionReport,
) {
    let profile = DiagnosticsPolicy::from_profile(graph.diagnostics_profile()).profile;
    let (change, invalidation) = graph
        .diagnostics_state()
        .pending_change_summary()
        .unwrap_or_else(|| {
            (
                ChangeInputSummary::new(Vec::new(), Vec::new(), 0, None),
                InvalidationSummary::new(0, 0, 0),
            )
        });
    let explanation = if DiagnosticsPolicy::from_profile(profile).retain_flow_explanation {
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
        sample_flow_causes(graph, report, profile.detail_limit()),
        Vec::new(),
        None,
        explanation,
    );
    let history = if execution_history_unchanged(report) {
        graph
            .diagnostics_state()
            .recent_history()
            .back()
            .cloned()
            .unwrap_or_else(|| ExecutionHistorySummary::from_graph(graph, profile))
    } else {
        ExecutionHistorySummary::from_graph(graph, profile)
    };
    graph.diagnostics_state_mut().complete_flow(flow, history);
    for task in report
        .stages
        .iter()
        .flat_map(|stage| stage.task_records.iter())
    {
        let cursor = graph.diagnostics_state_mut().allocate_replay_cursor();
        let branch_id = graph.observe().current_branch().id;
        let lineage_artifact_id = graph
            .get_entry(task.node)
            .ok()
            .and_then(|entry| entry.get_runtime_artifact_state())
            .and_then(|state| state.lineage_artifact_id);
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
                Some(task_outcome_label(task.outcome).to_owned()),
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
            ) || link.kind == "SkippedByComparator"
                || link.kind.starts_with("ConditionDeferred::")
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
                .map(|link| link.kind.clone())
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

fn task_outcome_label(outcome: TaskExecutionOutcome) -> &'static str {
    match outcome {
        TaskExecutionOutcome::Recomputed => "Recomputed",
        TaskExecutionOutcome::ValidatedClean => "ValidatedClean",
        TaskExecutionOutcome::ConditionDeferred => "ConditionDeferred",
        TaskExecutionOutcome::ConditionRevertedClean => "ConditionRevertedClean",
        TaskExecutionOutcome::MemoizedReuse => "MemoizedReuse",
        TaskExecutionOutcome::PropagationSuppressed => "PropagationSuppressed",
        TaskExecutionOutcome::Pruned => "Pruned",
    }
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
