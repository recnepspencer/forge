use crate::data::aspect::AspectMask;
use crate::data::output::{OutputChange, PartitionSubscription};
use crate::data::trace::TraceSummary;
use crate::facade::*;
use crate::tests::support::*;

#[test]
fn dirty_partition_scopes_are_classified_before_stale_output_diff_trace() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let branch = graph.node().build();
    let target = graph.node().build();
    graph
        .add_partition_detail_dependency(branch, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph.add_dependency(target, branch, ASPECT_A).unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, branch, &mut compute).unwrap();
    evaluate(&mut graph, target, &mut compute).unwrap();

    {
        let entry = graph.get_entry_mut(branch).unwrap();
        entry.set_state(NodeState::Dirty);
        entry.set_dirty_aspects(AspectMask::from_aspect(ASPECT_A));
        entry.add_dirty_partition_scope(
            ASPECT_A,
            PartitionSubscription::partition_and_detail("wing", "rib-12"),
        );
        let trace = TraceSummary {
            output_change: OutputChange::Unchanged,
            ..TraceSummary::default()
        };
        entry.set_trace_summary(Some(trace));
    }
    {
        let entry = graph.get_entry_mut(target).unwrap();
        entry.set_state(NodeState::Dirty);
        entry.set_dirty_aspects(AspectMask::from_aspect(ASPECT_A));
    }

    let plan = graph
        .build_evaluation_plan(&[target], EvaluationRequestMode::Default)
        .unwrap();
    let branch_task = plan
        .stages
        .iter()
        .flat_map(|stage| stage.tasks.iter())
        .find(|task| task.node == branch)
        .expect("branch task should be scheduled");

    assert!(matches!(
        branch_task.reason,
        TaskReason::PartitionScopedDependency
    ));
}

#[test]
fn maybe_stale_nodes_with_dirty_partition_scopes_do_not_fast_validate_clean() {
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
            Ok(view.finish(NodeEvaluationResult::from_version(version_ab(2, 0))))
        })
        .unwrap();

    assert_eq!(report.tasks_validated_clean, 0, "{report:?}");
    assert_eq!(report.tasks_executed, 1, "{report:?}");
}

#[test]
fn dependency_snapshots_collapse_conflicting_duplicate_logical_entries_to_latest_version() {
    let mut graph = SignalGraph::new();
    let source = graph.create_node();
    let node = graph.create_node();
    let mut snapshot = crate::data::dependency::DependencySnapshot::empty();
    snapshot.record(source, ASPECT_A, 1, None);
    snapshot.record(source, ASPECT_A, 3, None);
    snapshot.record(source, ASPECT_A, 2, None);

    graph.set_dep_snapshot(node, snapshot).unwrap();

    let entries = graph.get_dep_snapshot(node).unwrap().entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].cached_version, 3);
}

#[test]
fn output_identity_uses_profile_native_stable_hash_constants() {
    let identity = OutputIdentity::new("forge-signal");
    #[cfg(feature = "profile-compact")]
    assert_eq!(identity.stable_hash(), 0x3ffa6bfdb82b40cf_u64);
    #[cfg(any(feature = "profile-standard", feature = "profile-extended"))]
    assert_eq!(
        identity.stable_hash(),
        0x3b58a385f680a7e1d1206b1e9f2feb17_u128
    );
}
