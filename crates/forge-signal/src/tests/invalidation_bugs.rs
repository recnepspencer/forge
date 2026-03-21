use crate::facade::*;
use crate::tests::support::*;

#[test]
fn partition_scoped_dependencies_on_same_source_check_all_matching_edges() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-13")
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12"))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")))
    })
    .unwrap();
    evaluate(&mut graph, dependent, &mut |_id, graph| {
        let version = graph.get_entry(source).unwrap().get_aspect_version();
        Ok(NodeEvaluationResult::from_version(version))
    })
    .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .unwrap();

    assert_eq!(
        graph.get_state(dependent).unwrap(),
        NodeState::Dirty,
        "later partition-scoped dependency edges on the same source/aspect must still be checked"
    );
}

#[test]
fn repeated_partition_invalidations_union_dirty_scopes_until_evaluation() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-13")
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12"))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")))
    })
    .unwrap();
    evaluate(&mut graph, dependent, &mut |_id, graph| {
        let version = graph.get_entry(source).unwrap().get_aspect_version();
        Ok(NodeEvaluationResult::from_version(version))
    })
    .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();
    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .unwrap();

    let entry = graph.get_entry(dependent).unwrap();
    let scopes = entry.get_dirty_partition_scopes();
    assert_eq!(entry.get_state(), &NodeState::Dirty);
    assert!(
        scopes.iter().any(|scope| {
            scope.partition.0.as_str() == "wing" && scope.detail.as_deref() == Some("rib-12")
        }),
        "the first invalidation scope should not be erased by a later wave"
    );
    assert!(
        scopes.iter().any(|scope| {
            scope.partition.0.as_str() == "wing" && scope.detail.as_deref() == Some("rib-13")
        }),
        "the second invalidation scope should be merged with earlier scopes"
    );
}

#[test]
fn whole_aspect_invalidation_does_not_erase_other_aspects_partition_precision() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_B, "tail", "panel-7")
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 1))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12"))
            .with_changed_region(ChangedRegion::new("tail").with_detail("panel-7")))
    })
    .unwrap();
    evaluate(&mut graph, dependent, &mut |_id, graph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    })
    .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_B,
        &[ChangedRegion::new("tail").with_detail("panel-7")],
    )
    .unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();

    let entry = graph.get_entry(dependent).unwrap();
    let scopes = entry.get_dirty_partition_scopes();
    assert!(
        scopes.iter().any(|scope| {
            scope.partition.0.as_str() == "tail" && scope.detail.as_deref() == Some("panel-7")
        }),
        "whole-aspect invalidation on aspect A must not erase scoped dirtiness retained for aspect B"
    );
}

#[test]
fn reconverging_frontier_does_not_revisit_already_visited_nodes() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();
    let d = graph.node().build();
    let e = graph.node().build();

    graph.append_dependency(b, source, ASPECT_A).unwrap();
    graph.append_dependency(c, source, ASPECT_A).unwrap();
    graph.append_dependency(e, source, ASPECT_A).unwrap();
    graph.append_dependency(d, b, ASPECT_A).unwrap();
    graph.append_dependency(d, c, ASPECT_A).unwrap();
    graph.append_dependency(e, d, ASPECT_A).unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)))
    })
    .unwrap();
    for node in [b, c, d, e] {
        evaluate(&mut graph, node, &mut |_id, _graph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0)))
        })
        .unwrap();
    }

    graph.reset_telemetry();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();

    assert_eq!(
        graph.telemetry().invalidation.invalidation_nodes_visited,
        4,
        "reconverging downstream nodes should only count once during transitive invalidation"
    );
}

#[test]
fn deep_invalidation_chain_completes_without_recursive_cycle_detection() {
    let mut graph = SignalGraph::new();
    let root = graph.node().build();
    let mut previous = root;
    for _ in 0..20_000 {
        let next = graph.node().build();
        graph.append_dependency(next, previous, ASPECT_A).unwrap();
        previous = next;
    }

    let result = mark_dirty(&mut graph, root, ASPECT_A);
    assert!(
        result.is_ok(),
        "deep invalidation chains should not overflow recursive cycle detection: {result:?}"
    );
}

#[test]
fn batch_invalidation_reuses_transitive_wave_for_same_aspect_sources() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let shared = graph.node().build();
    let leaf = graph.node().build();

    graph.append_dependency(shared, source_a, ASPECT_A).unwrap();
    graph.append_dependency(shared, source_b, ASPECT_A).unwrap();
    graph.append_dependency(leaf, shared, ASPECT_A).unwrap();

    for node in [source_a, source_b, shared, leaf] {
        evaluate(&mut graph, node, &mut |_id, _graph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0)))
        })
        .unwrap();
    }

    let mut scalar = graph.clone();
    scalar.reset_telemetry();
    mark_dirty(&mut scalar, source_a, ASPECT_A).unwrap();
    mark_dirty(&mut scalar, source_b, ASPECT_A).unwrap();

    let mut batched = graph;
    batched.reset_telemetry();
    mark_dirty_batch(
        &mut batched,
        &DirtyBatch::from_sources([(source_a, ASPECT_A), (source_b, ASPECT_A)]),
    )
    .unwrap();

    assert_eq!(batched.get_state(shared).unwrap(), NodeState::Dirty);
    assert_eq!(batched.get_state(leaf).unwrap(), NodeState::MaybeStale);
    assert!(
        batched.telemetry().invalidation.invalidation_nodes_visited
            < scalar.telemetry().invalidation.invalidation_nodes_visited,
        "batched invalidation should reuse the downstream transitive wave for same-aspect sources"
    );
}

#[test]
fn unscoped_dependency_removal_removes_partition_scoped_edges() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    graph.drop_dependency(dependent, source, ASPECT_A).unwrap();

    assert!(
        graph.dependencies_of(dependent).unwrap().is_empty(),
        "unscoped dependency removal should remove matching scoped edges too"
    );
}

#[test]
fn whole_partition_invalidates_partition_detail_subscribers() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    })
    .unwrap();
    evaluate(&mut graph, dependent, &mut |_id, graph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    })
    .unwrap();

    mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")]).unwrap();

    assert_eq!(
        graph.get_state(dependent).unwrap(),
        NodeState::Dirty,
        "whole-partition changes must invalidate detail subscribers on the same partition"
    );
}

#[test]
fn frontier_execution_summary_exposes_direct_dirty_and_maybe_stale_entries() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let direct_dirty = graph.node().build();
    let maybe_stale = graph.node().build();

    graph
        .append_partition_detail_dependency(direct_dirty, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(maybe_stale, source, ASPECT_A, "wing", "rib-13")
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

    assert!(wave.entries.iter().any(|entry| {
        entry.node == direct_dirty
            && matches!(entry.classification, FrontierEntryClassification::DirectDirty)
            && matches!(entry.inclusion_basis, FrontierInclusionBasis::DetailScopeOverlap)
    }));
    assert!(wave.entries.iter().any(|entry| {
        entry.node == maybe_stale
            && matches!(entry.classification, FrontierEntryClassification::MaybeStale)
            && matches!(entry.inclusion_basis, FrontierInclusionBasis::DirectSubscriptionMatch)
    }));
}

#[test]
fn frontier_runtime_counters_are_derived_from_execution_summary() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let whole = graph.node().build();
    let detail = graph.node().build();

    graph
        .append_partition_dependency(whole, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .append_partition_detail_dependency(detail, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing")],
    )
    .unwrap();

    let summary = graph
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("frontier execution summary should be retained");
    let metrics = graph.observe().metrics();
    assert_eq!(
        metrics.invalidation.frontier_seed_count,
        summary.counters.frontier_seed_count
    );
    assert_eq!(
        metrics.invalidation.frontier_direct_wave_count,
        summary.counters.frontier_direct_wave_count
    );
    assert_eq!(
        metrics.invalidation.frontier_transitive_wave_count,
        summary.counters.frontier_transitive_wave_count
    );
    assert_eq!(
        metrics.invalidation.frontier_cycle_check_candidate_count,
        summary.counters.frontier_cycle_check_candidate_count
    );
}

#[test]
fn frontier_transitive_wave_count_stays_zero_when_no_transitive_entries_realize() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let direct = graph.node().build();

    graph
        .append_partition_detail_dependency(direct, source, ASPECT_A, "wing", "rib-12")
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
    let metrics = graph.observe().metrics();

    assert!(summary.transitive_waves.iter().all(|wave| wave.entries.is_empty()));
    assert_eq!(summary.counters.frontier_transitive_wave_count, 0);
    assert_eq!(metrics.invalidation.frontier_transitive_wave_count, 0);
}

#[test]
fn frontier_tracing_policy_changes_retained_richness_not_invalidation_truth() {
    let mut operational = SignalGraph::new();
    operational.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = operational.node().partitioned_output().build();
    let dependent = operational.node().build();
    operational
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let mut development = operational.clone();
    development.set_runtime_policy(SignalRuntimePolicy::development());

    let changed = &[ChangedRegion::new("wing").with_detail("rib-12")];
    mark_dirty_with_regions(&mut operational, source, ASPECT_A, changed).unwrap();
    mark_dirty_with_regions(&mut development, source, ASPECT_A, changed).unwrap();

    let operational_summary = operational
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("operational summary should exist");
    let development_summary = development
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("development summary should exist");

    assert_eq!(operational_summary.seed_count, development_summary.seed_count);
    assert_eq!(operational_summary.direct_waves, development_summary.direct_waves);
    assert_eq!(
        operational_summary.transitive_waves,
        development_summary.transitive_waves
    );
    assert_eq!(
        operational_summary.touched_scope_summary,
        development_summary.touched_scope_summary
    );
    assert_eq!(operational_summary.counters.frontier_trace_retained_count, 0);
    assert!(
        development_summary.counters.frontier_trace_retained_count
            > operational_summary.counters.frontier_trace_retained_count
    );
    assert!(
        operational
            .observe()
            .latest_invalidation_trace_records()
            .is_empty()
    );
    assert!(
        !development
            .observe()
            .latest_invalidation_trace_records()
            .is_empty()
    );
}

#[test]
fn duplicate_dirty_entries_canonicalize_into_one_frontier_seed() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph.append_dependency(dependent, source, ASPECT_A).unwrap();

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
    assert_eq!(wave_a.entries.iter().map(|entry| entry.node).collect::<Vec<_>>(), vec![dep_a]);
    assert_eq!(wave_b.entries.iter().map(|entry| entry.node).collect::<Vec<_>>(), vec![dep_b]);
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
        graph.observe().latest_frontier_execution_summary().is_none(),
        "failed frontier preflight must not leave behind a committed frontier summary"
    );
}

#[test]
fn one_node_with_multiple_justifications_collapses_to_stable_canonical_entry() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();

    graph.append_dependency(dependent, source, ASPECT_A).unwrap();
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
            graph.observe()
                .latest_frontier_execution_summary()
                .cloned()
                .expect("frontier execution summary should be retained"),
        );

        evaluate(&mut graph, source, &mut |_id, _graph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_changed_region(
                ChangedRegion::new("wing").with_detail("rib-12"),
            ))
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
