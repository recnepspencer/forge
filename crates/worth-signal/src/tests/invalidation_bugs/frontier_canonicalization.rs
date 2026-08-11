use crate::facade::*;
use crate::tests::support::*;

#[test]
fn duplicate_dirty_entries_canonicalize_into_one_frontier_seed() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    mark_dirty_batch(
        &mut graph,
        &DirtyBatch::new([
            DirtyBatchEntry::new(
                source,
                ASPECT_A,
                vec![ChangedRegion::new("wing").with_detail("rib-12")],
            ),
            DirtyBatchEntry::new(
                source,
                ASPECT_A,
                vec![ChangedRegion::new("wing").with_detail("rib-13")],
            ),
        ]),
    )
    .unwrap();

    let summary = graph
        .observe()
        .latest_frontier_execution_summary()
        .expect("frontier execution summary should be retained");
    assert_eq!(summary.seed_count, 1);
    assert_eq!(summary.counters.frontier_seed_count, 1);
    assert_eq!(summary.direct_waves.len(), 1);
}

#[test]
fn disjoint_aspect_batches_produce_disjoint_frontier_waves() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dep_a = graph.node().build();
    let dep_b = graph.node().build();
    graph.append_dependency(dep_a, source, ASPECT_A).unwrap();
    graph.append_dependency(dep_b, source, ASPECT_B).unwrap();

    mark_dirty_batch(
        &mut graph,
        &DirtyBatch::from_sources([(source, ASPECT_A), (source, ASPECT_B)]),
    )
    .unwrap();

    let summary = graph
        .observe()
        .latest_frontier_execution_summary()
        .expect("frontier execution summary should be retained");
    assert_eq!(summary.direct_waves.len(), 2);
    let wave_a = summary
        .direct_waves
        .iter()
        .find(|wave| wave.aspect == ASPECT_A)
        .expect("aspect A wave should exist");
    let wave_b = summary
        .direct_waves
        .iter()
        .find(|wave| wave.aspect == ASPECT_B)
        .expect("aspect B wave should exist");
    assert_eq!(
        wave_a
            .entries
            .iter()
            .map(|entry| entry.node)
            .collect::<Vec<_>>(),
        vec![dep_a]
    );
    assert_eq!(
        wave_b
            .entries
            .iter()
            .map(|entry| entry.node)
            .collect::<Vec<_>>(),
        vec![dep_b]
    );
}

#[test]
fn reachable_cycle_detection_fails_before_false_frontier_commit() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().build();
    let a = graph.node().build();
    let b = graph.node().build();

    graph.append_dependency(a, source, ASPECT_A).unwrap();
    graph.append_dependency(b, a, ASPECT_A).unwrap();
    graph.append_dependency(a, b, ASPECT_A).unwrap();
    let source_state_before = graph.get_state(source).unwrap();
    let a_state_before = graph.get_state(a).unwrap();
    let b_state_before = graph.get_state(b).unwrap();

    let err = mark_dirty(&mut graph, source, ASPECT_A).expect_err("reachable cycle should fail");
    match err {
        SignalError::CycleDetected { .. } => {}
        other => panic!("expected cycle-detected error, got {other:?}"),
    }

    assert_eq!(graph.get_state(source).unwrap(), source_state_before);
    assert_eq!(graph.get_state(a).unwrap(), a_state_before);
    assert_eq!(graph.get_state(b).unwrap(), b_state_before);
    assert!(
        graph
            .observe()
            .latest_frontier_execution_summary()
            .is_none(),
        "failed frontier preflight must not leave behind a committed frontier summary"
    );
}

#[test]
fn one_node_with_multiple_justifications_collapses_to_stable_canonical_entry() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();

    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let summary = graph
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("frontier execution summary should be retained");
    let wave = summary
        .direct_waves
        .iter()
        .find(|wave| wave.aspect == ASPECT_A)
        .expect("expected aspect wave");

    assert_eq!(wave.entries.len(), 1);
    let entry = &wave.entries[0];
    assert_eq!(entry.node, dependent);
    assert!(matches!(
        entry.classification,
        FrontierEntryClassification::DirectDirty
    ));
    assert!(matches!(
        entry.inclusion_basis,
        FrontierInclusionBasis::DirectSubscriptionMatch
    ));
}

#[test]
fn repeated_identical_inputs_produce_deterministic_frontier_summary() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let whole = graph.node().build();
    let detail = graph.node().build();
    let leaf = graph.node().build();

    graph
        .append_partition_dependency(whole, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .append_partition_detail_dependency(detail, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph.append_dependency(leaf, whole, ASPECT_A).unwrap();
    graph.append_dependency(leaf, detail, ASPECT_A).unwrap();

    let changed = [ChangedRegion::new("wing").with_detail("rib-12")];
    let mut summaries = Vec::new();
    for _ in 0..2 {
        mark_dirty_with_regions(&mut graph, source, ASPECT_A, &changed).unwrap();
        summaries.push(
            graph
                .observe()
                .latest_frontier_execution_summary()
                .cloned()
                .expect("frontier execution summary should be retained"),
        );

        evaluate(&mut graph, source, &mut |_id, _graph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
        })
        .unwrap();
        for node in [whole, detail, leaf] {
            evaluate(&mut graph, node, &mut |_id, graph| {
                Ok(NodeEvaluationResult::from_version(
                    graph.get_entry(source).unwrap().get_aspect_version(),
                ))
            })
            .unwrap();
        }
    }

    assert_eq!(summaries[0], summaries[1]);
}

#[test]
fn transitive_wave_contains_only_nodes_reachable_from_planned_roots() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let direct_dirty = graph.node().build();
    let maybe_stale = graph.node().build();
    let transitive_from_dirty = graph.node().build();
    let transitive_from_stale = graph.node().build();
    let unrelated_root = graph.node().build();
    let unrelated_leaf = graph.node().build();

    graph
        .append_partition_detail_dependency(direct_dirty, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(maybe_stale, source, ASPECT_A, "wing", "rib-13")
        .unwrap();
    graph
        .append_dependency(transitive_from_dirty, direct_dirty, ASPECT_A)
        .unwrap();
    graph
        .append_dependency(transitive_from_stale, maybe_stale, ASPECT_A)
        .unwrap();
    graph
        .append_dependency(unrelated_leaf, unrelated_root, ASPECT_A)
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let summary = graph
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("frontier execution summary should be retained");
    let transitive_wave = summary
        .transitive_waves
        .iter()
        .find(|wave| wave.aspect == ASPECT_A)
        .expect("expected aspect transitive wave");
    let transitive_nodes = transitive_wave
        .entries
        .iter()
        .map(|entry| entry.node)
        .collect::<Vec<_>>();

    assert!(transitive_nodes.contains(&transitive_from_dirty));
    assert!(transitive_nodes.contains(&transitive_from_stale));
    assert!(!transitive_nodes.contains(&unrelated_root));
    assert!(!transitive_nodes.contains(&unrelated_leaf));
    assert!(transitive_wave.entries.iter().all(|entry| matches!(
        entry.classification,
        FrontierEntryClassification::MaybeStale
    )));
    assert!(transitive_wave.entries.iter().all(|entry| matches!(
        entry.inclusion_basis,
        FrontierInclusionBasis::TransitiveReachability
    )));
}
