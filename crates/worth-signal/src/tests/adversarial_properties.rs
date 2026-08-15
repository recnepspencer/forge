use crate::data::dependency::DependencySnapshot;
use crate::facade::*;
use crate::tests::support::*;

fn region(partition: &str, detail: Option<&str>) -> ChangedRegion {
    match detail {
        Some(detail) => ChangedRegion::new(partition).with_detail(detail),
        None => ChangedRegion::new(partition),
    }
}

#[test]
fn dependency_snapshots_dedupe_semantically_identical_permutations() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();

    let mut left = DependencySnapshot::empty();
    left.record(b, ASPECT_A, 3, None);
    left.record(c, ASPECT_B, 9, None);
    left.record(a, ASPECT_A, 1, None);

    let mut right = DependencySnapshot::empty();
    right.record(c, ASPECT_B, 9, None);
    right.record(a, ASPECT_A, 1, None);
    right.record(b, ASPECT_A, 3, None);

    graph.set_dep_snapshot(a, left).unwrap();
    let left_id = graph.get_entry(a).unwrap().get_dep_snapshot_id();
    graph.set_dep_snapshot(b, right).unwrap();
    let right_id = graph.get_entry(b).unwrap().get_dep_snapshot_id();
    assert_eq!(
        left_id, right_id,
        "snapshot interning should treat reordered-but-equivalent dependency snapshots as identical"
    );
}

#[test]
fn whole_partition_changes_match_partition_and_detail_subscribers_under_permutations() {
    let mut graph = SignalGraph::new();
    let upstream = graph.node().partitioned_output().build();
    let downstream = graph.node().build();
    graph
        .append_partition_detail_dependency(downstream, upstream, ASPECT_A, "wing", "left")
        .unwrap();
    let mut baseline = |_id, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, upstream, &mut baseline).unwrap();
    evaluate(&mut graph, downstream, &mut baseline).unwrap();

    for (offset, changed) in [
        vec![region("wing", None)],
        vec![region("wing", Some("left"))],
        vec![region("wing", Some("right")), region("wing", None)],
    ]
    .into_iter()
    .enumerate()
    {
        mark_dirty_with_regions(&mut graph, upstream, ASPECT_A, &changed).unwrap();
        assert_eq!(graph.get_state(downstream).unwrap(), NodeState::Clean);
        let changed_for_commit = changed.clone();
        let mut commit = move |_id, _graph: &SignalGraph| {
            let mut result = NodeEvaluationResult::from_version(version_ab(offset as u64 + 2, 0));
            for region in &changed_for_commit {
                result = result.with_changed_region(region.clone());
            }
            Ok(result)
        };
        evaluate(&mut graph, upstream, &mut commit).unwrap();
        assert_eq!(
            graph.get_state(downstream).unwrap(),
            NodeState::Dirty,
            "whole-partition or matching detail commits must invalidate detail subscribers"
        );
        evaluate(&mut graph, downstream, &mut baseline).unwrap();
    }
}

#[test]
fn continuity_token_does_not_hide_real_identity_change_across_permutations() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    let mut first = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_output_identity("surface-a")
            .with_continuity_token("stable-surface"))
    };
    let mut second = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_output_identity("surface-b")
            .with_continuity_token("stable-surface"))
    };

    evaluate(&mut graph, source, &mut first).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut second).unwrap();

    let lineage = graph.observe().lineage_for_node(source);
    assert!(
        lineage.iter().any(|record| matches!(
            record.kind,
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::Replaced,
                ..
            }
        )),
        "matching continuity tokens must not suppress real output-identity replacement"
    );
}
