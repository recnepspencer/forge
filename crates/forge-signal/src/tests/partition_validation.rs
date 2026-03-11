use crate::data::aspect::AspectMask;
use crate::data::output::PartitionSubscription;
use crate::data::trace::TraceSummary;
use crate::facade::*;
use crate::tests::support::*;

#[test]
fn maybe_stale_partition_nodes_recompute_when_changed_region_evidence_is_absent() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .add_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    {
        let source_entry = graph.get_entry_mut(source).unwrap();
        source_entry.set_aspect_version(version_ab(2, 0));
        source_entry.set_trace_summary(Some(TraceSummary::default()));
    }
    {
        let entry = graph.get_entry_mut(dependent).unwrap();
        entry.set_state(NodeState::MaybeStale);
        entry.set_dirty_aspects(AspectMask::from_aspect(ASPECT_A));
        entry.add_dirty_partition_scope(
            ASPECT_A,
            PartitionSubscription::partition_and_detail("wing", "rib-12"),
        );
    }

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let report = graph
        .execute_prepared_plan(&plan, &|_node, view| {
            Ok(view.finish(NodeEvaluationResult::from_version(version_ab(3, 0))))
        })
        .unwrap();

    assert_eq!(report.tasks_validated_clean, 0);
    assert_eq!(report.tasks_executed, 1);
}
