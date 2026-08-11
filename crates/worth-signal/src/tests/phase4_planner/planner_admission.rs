use crate::facade::{
    mark_dirty, ContextRequirement, EvaluationOutput, EvaluationRequestMode, NodeId, SignalError,
    SignalGraph, TaskReason,
};
use crate::tests::support::{
    evaluate, mask_a, version_ab, GraphDependencyBatchExt, ASPECT_A, ASPECT_B,
};
use std::sync::atomic::{AtomicU32, Ordering};

#[test]
fn build_evaluation_plan_orders_chain_by_stage_depth() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();
    graph.append_dependency(b, a, ASPECT_A).unwrap();
    graph.append_dependency(c, b, ASPECT_A).unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, a, &mut compute).unwrap();
    evaluate(&mut graph, b, &mut compute).unwrap();
    evaluate(&mut graph, c, &mut compute).unwrap();
    mark_dirty(&mut graph, a, ASPECT_A).unwrap();

    let plan = graph
        .build_evaluation_plan(&[c], EvaluationRequestMode::Default)
        .unwrap();

    assert_eq!(plan.summary.stage_count, 3);
    assert_eq!(plan.stages[0].tasks[0].node, a);
    assert_eq!(plan.stages[1].tasks[0].node, b);
    assert_eq!(plan.stages[2].tasks[0].node, c);
    assert!(plan.stages[2].tasks[0].direct_request);
}

#[test]
fn build_evaluation_plan_omits_clean_target() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, node, &mut compute).unwrap();

    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap();

    assert_eq!(plan.summary.task_count, 0);
    assert!(plan.stages.is_empty());
}

#[test]
fn build_evaluation_plan_prunes_dirty_target_when_contract_reads_do_not_intersect() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().reads_aspects(mask_a()).build();
    graph
        .append_dependency(dependent, source, ASPECT_B)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_B).unwrap();

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();

    assert_eq!(plan.summary.task_count, 0);
    assert!(plan.stages.is_empty());
}

#[test]
fn build_evaluation_plan_rejects_missing_relational_snapshot_context() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .requires_context(ContextRequirement::RelationalSnapshot)
        .build();

    let err = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap_err();

    assert!(matches!(
        err,
        SignalError::ContractViolation {
            node: contract_node,
            requirement: ContextRequirement::RelationalSnapshot,
        } if contract_node == node
    ));
}

#[test]
fn force_on_demand_plans_and_executes_clean_target() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut bootstrap = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, node, &mut bootstrap).unwrap();

    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();

    assert_eq!(plan.summary.task_count, 1);
    assert!(matches!(
        plan.stages[0].tasks[0].reason,
        TaskReason::ConditionForced
    ));

    let calls = AtomicU32::new(0);
    let report = graph
        .execute_prepared_plan(&plan, &(), &|_view| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(version_ab(2, 0)))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(report.tasks_executed, 1, "{report:?}");
}

#[test]
fn execute_plan_returns_execution_report_and_updates_trace_record_id() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let source_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let report = graph
        .execute_prepared_plan(&plan, &(), &|view| {
            let node = view.node();
            if node == source {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(source_v2(
                    node,
                    view.graph(),
                )?))
            } else {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                    dependent_compute(node, view.graph())?,
                ))
            }
        })
        .unwrap();

    assert_eq!(report.stage_count, 2);
    assert_eq!(report.task_count, 2);
    assert!(report.tasks_executed >= 2);
    assert!(graph
        .get_entry(dependent)
        .unwrap()
        .trace_summary()
        .unwrap()
        .execution_record_id
        .is_some());
    assert_eq!(
        graph
            .observe()
            .explain(dependent)
            .unwrap()
            .execution_record_id,
        graph
            .get_entry(dependent)
            .unwrap()
            .trace_summary()
            .unwrap()
            .execution_record_id
    );
}
