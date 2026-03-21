use std::sync::atomic::{AtomicU32, Ordering};

use crate::facade::*;
use crate::tests::support::*;

#[test]
fn output_identity_unchanged_suppresses_downstream_propagation() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("artifact"))
    };
    let mut source_v2_same_identity = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("artifact"))
    };
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Dirty);

    evaluate(&mut graph, source, &mut source_v2_same_identity).unwrap();

    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
    let explanation = graph.observe().explain(source).unwrap();
    assert_eq!(explanation.output_change, Some(OutputChange::Unchanged));
    assert!(explanation.propagation_suppressed);
    assert_eq!(
        graph
            .observe()
            .metrics()
            .evaluation
            .suppressed_downstream_propagations,
        1
    );
}

#[test]
fn output_identity_suppression_does_not_hide_other_real_upstream_changes() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().output_identity().build();
    let source_b = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source_a, ASPECT_A)
        .unwrap();
    graph
        .append_dependency(dependent, source_b, ASPECT_B)
        .unwrap();

    let mut source_a_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("artifact-a"))
    };
    let mut source_a_v2_same_identity = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("artifact-a"))
    };
    let mut source_b_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut source_b_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 2));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 10)))
    };

    evaluate(&mut graph, source_a, &mut source_a_v1).unwrap();
    evaluate(&mut graph, source_b, &mut source_b_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty(&mut graph, source_a, ASPECT_A).unwrap();
    mark_dirty(&mut graph, source_b, ASPECT_B).unwrap();
    evaluate(&mut graph, source_a, &mut source_a_v2_same_identity).unwrap();
    evaluate(&mut graph, source_b, &mut source_b_v2).unwrap();

    assert_ne!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn continuity_token_match_does_not_hide_real_output_identity_change() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_output_identity("artifact-a")
            .with_continuity_token("stable-lineage"))
    })
    .unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_output_identity("artifact-b")
            .with_continuity_token("stable-lineage"))
    })
    .unwrap();

    let explanation = graph.observe().explain(source).unwrap();
    assert_eq!(
        explanation.output_change,
        Some(OutputChange::Replaced),
        "a continuity-token match must not erase a real output identity change"
    );
}

#[test]
fn changed_regions_flow_into_trace_and_explanation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().partitioned_output().build();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing-panel").with_detail("rib-12")))
    };

    evaluate(&mut graph, node, &mut compute).unwrap();

    let explanation = graph.observe().explain(node).unwrap();
    assert_eq!(explanation.changed_regions.len(), 1);
    assert_eq!(
        explanation
            .historical_artifact_record
            .as_ref()
            .map(|record| record.runtime.changed_partition_count)
            .unwrap(),
        1
    );
    assert_eq!(
        graph
            .observe()
            .metrics()
            .invalidation
            .partition_aware_recomputations,
        1
    );
}

#[test]
fn keyed_node_lookup_reuses_same_runtime_entry() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family = define_keyed_computation(&mut runtime, "fighter-projection", ());

    let node_a = family.keyed("left-wing").node(&mut runtime);
    let node_b = family.keyed("left-wing").node(&mut runtime);
    let node_c = family.keyed("right-wing").node(&mut runtime);

    assert_eq!(node_a, node_b);
    assert_ne!(node_a, node_c);
}

#[test]
fn defined_computation_keyed_lookup_reuses_same_runtime_entry() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let volumes = runtime
        .define_computation(ComputationSpec {
            family: "fighter-projection".into(),
            contract: NodeContract::reads([ASPECT_A]).with_produces([ASPECT_B]),
            tier: (),
            comparator: VersionComparatorPolicy::Exact,
            evaluator: |_ctx: &mut EvaluationContext<'_, ()>| {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                    NodeEvaluationResult::from_version(version_ab(1, 0)),
                ))
            },
        })
        .unwrap();

    let node_a = volumes.keyed("left-wing").node(&mut runtime);
    let node_b = volumes.keyed("left-wing").node(&mut runtime);
    let node_c = volumes.keyed("right-wing").node(&mut runtime);

    assert_eq!(node_a, node_b);
    assert_ne!(node_a, node_c);
}

#[test]
fn keyed_evaluation_can_reuse_memoized_result() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family = define_keyed_computation(&mut runtime, "projection", ());
    let keyed = family.keyed("bulkhead");
    let node = keyed.node(&mut runtime);
    let computation = keyed.memoized("shape-v1");
    let mut runtime_ctx = ();
    let compute_calls = AtomicU32::new(0);

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node, &computation, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("bulkhead-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    mark_dirty(runtime.graph_mut(), node, ASPECT_A).unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node, &computation, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(99, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 1);
    let explanation = runtime.observe().explain(node).unwrap();
    let reuse_basis = explanation.reuse_basis.expect("memoized reuse basis");
    assert_eq!(
        reuse_basis.strategy,
        Some(crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse)
    );
    assert_eq!(reuse_basis.source, ReuseSource::MemoizedArtifact);
    assert_eq!(reuse_basis.crossing, ReuseCrossing::None);
    assert!(reuse_basis.dependency_snapshot_basis.is_some());
    assert_eq!(
        explanation.memoized_origin,
        Some(MemoizedResultOrigin::MemoizedFromCache)
    );
    let metrics = runtime.observe().metrics();
    assert_eq!(metrics.invalidation.keyed_evaluation_count, 2);
    assert_eq!(metrics.evaluation.memoization_hits, 1);
    assert_eq!(metrics.evaluation.memoization_misses, 1);
}

#[test]
fn defined_computation_evaluate_memoized_reuses_cached_result() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let compute_calls = AtomicU32::new(0);
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A]).with_produces([ASPECT_B]),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("bulkhead-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let bulkhead = projection.keyed("bulkhead");
    let node = bulkhead.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            bulkhead.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();

    mark_dirty(runtime.graph_mut(), node, ASPECT_A).unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            bulkhead.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 1);
    let explanation = runtime.observe().explain(node).unwrap();
    let reuse_basis = explanation.reuse_basis.expect("memoized reuse basis");
    assert_eq!(
        reuse_basis.strategy,
        Some(crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse)
    );
    assert_eq!(reuse_basis.source, ReuseSource::MemoizedArtifact);
    assert_eq!(reuse_basis.crossing, ReuseCrossing::None);
    assert!(reuse_basis.dependency_snapshot_basis.is_some());
    assert_eq!(
        explanation.memoized_origin,
        Some(MemoizedResultOrigin::MemoizedFromCache)
    );
}

#[test]
fn defined_computation_evaluate_cross_identity_reuses_cached_result_via_public_api() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let compute_calls = AtomicU32::new(0);
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("cross-identity-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity(tx, "source", "shape-v1", "mesh-001")
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 1);
    let explanation = runtime.observe().explain(alias_node).unwrap();
    let reuse_basis = explanation.reuse_basis.expect("cross-identity reuse basis");
    assert_eq!(
        reuse_basis.strategy,
        Some(crate::data::reuse::ReuseStrategy::CrossIdentityPersistentMatch)
    );
    assert_eq!(reuse_basis.source, ReuseSource::PersistentCorrespondence);
    assert_eq!(reuse_basis.crossing, ReuseCrossing::PersistentIdentityBoundary);
    assert_eq!(
        explanation.reuse_origin,
        Some(crate::data::reuse::ReuseOrigin::CrossIdentityPersistentReuse)
    );
    let replay_event = runtime
        .graph()
        .replay_events()
        .iter()
        .rev()
        .find(|event| {
            event.kind == ReplayEventKind::TaskApplied && event.node == Some(alias_node)
        })
        .expect("cross-identity replay event");
    assert_eq!(
        replay_event.reuse_origin,
        Some(crate::data::reuse::ReuseOrigin::CrossIdentityPersistentReuse)
    );
    let history = runtime
        .observe()
        .execution_history_summary(DiagnosticsProfile::Development);
    assert_eq!(
        history
            .reuse_origin_counts
            .get("CrossIdentityPersistentReuse")
            .copied(),
        Some(1)
    );
    assert!(history.nodes.iter().any(|node| {
        node.node == alias_node
            && node.reuse_origin
                == Some(crate::data::reuse::ReuseOrigin::CrossIdentityPersistentReuse)
    }));
    assert_eq!(runtime.observe().metrics().evaluation.cross_identity_reuse_count, 1);
}

#[test]
fn cross_identity_contract_declared_basis_is_retained_in_runtime_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("contract-basis-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-contract");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_contract_basis(
                tx,
                "source",
                "shape-v1",
                "contract:mesh-family:v2",
            )
        })
        .unwrap();

    let runtime_state = runtime
        .graph()
        .observe()
        .runtime_artifact_state(alias_node)
        .unwrap()
        .cloned()
        .expect("runtime artifact state");
    assert_eq!(
        runtime_state.reuse_origin,
        crate::data::reuse::ReuseOrigin::CrossIdentityPersistentReuse
    );
    assert_eq!(
        runtime_state
            .reuse_boundary_context
            .as_ref()
            .and_then(|ctx| ctx.persistent_correspondence.as_ref()),
        Some(&crate::data::reuse::PersistentCorrespondenceEvidence::ContractDeclaredBasis(
            "contract:mesh-family:v2".to_string()
        ))
    );
}

#[test]
fn cross_identity_lineage_mapping_is_retained_in_runtime_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("lineage-map-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-lineage");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_lineage_mapping(
                tx,
                "source",
                "shape-v1",
                "lineage-map:mesh-42->mesh-77",
            )
        })
        .unwrap();

    let runtime_state = runtime
        .graph()
        .observe()
        .runtime_artifact_state(alias_node)
        .unwrap()
        .cloned()
        .expect("runtime artifact state");
    assert_eq!(
        runtime_state
            .reuse_boundary_context
            .as_ref()
            .and_then(|ctx| ctx.persistent_correspondence.as_ref()),
        Some(&crate::data::reuse::PersistentCorrespondenceEvidence::LineageBackedMapping(
            "lineage-map:mesh-42->mesh-77".to_string()
        ))
    );
}

#[test]
fn cross_identity_region_identity_basis_is_retained_in_runtime_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching()
                .with_partition_scope(PartitionSubscription::whole_partition("wing")),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("region-basis-artifact")
                        .with_output_change(OutputChange::Refreshed)
                        .with_changed_region(ChangedRegion::new("wing")),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-region");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_region_identity(
                tx,
                "source",
                "shape-v1",
                "region:wing",
            )
        })
        .unwrap();

    let runtime_state = runtime
        .graph()
        .observe()
        .runtime_artifact_state(alias_node)
        .unwrap()
        .cloned()
        .expect("runtime artifact state");
    assert_eq!(
        runtime_state
            .reuse_boundary_context
            .as_ref()
            .and_then(|ctx| ctx.persistent_correspondence.as_ref()),
        Some(&crate::data::reuse::PersistentCorrespondenceEvidence::RegionIdentityBasis(
            "region:wing".to_string()
        ))
    );
}

#[test]
fn cross_identity_changed_contract_basis_is_rejected_and_preserves_previous_correspondence() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let compute_calls = AtomicU32::new(0);
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("contract-mismatch-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-contract-mismatch");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_contract_basis(
                tx,
                "source",
                "shape-v1",
                "contract:mesh-family:v2",
            )
        })
        .unwrap();

    mark_dirty(runtime.graph_mut(), alias_node, ASPECT_A).unwrap();

    let err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_contract_basis(
                tx,
                "source",
                "shape-v1",
                "contract:mesh-family:v3",
            )
        })
        .expect_err("changed correspondence basis should be rejected");
    assert!(err.to_string().contains("reuse certification failed"));
    assert_eq!(
        compute_calls.load(Ordering::Relaxed),
        1,
        "rejected cross-identity mismatch should fail before fallback recompute"
    );

    let runtime_state = runtime
        .graph()
        .observe()
        .runtime_artifact_state(alias_node)
        .unwrap()
        .cloned()
        .expect("runtime artifact state");
    assert_eq!(
        runtime_state
            .reuse_boundary_context
            .as_ref()
            .and_then(|ctx| ctx.persistent_correspondence.as_ref()),
        Some(&crate::data::reuse::PersistentCorrespondenceEvidence::ContractDeclaredBasis(
            "contract:mesh-family:v2".to_string()
        )),
        "failed reuse admission must not overwrite prior certified correspondence"
    );
    assert_eq!(
        runtime.observe().metrics().evaluation.reuse_rejected_persistent_correspondence_invalid_count,
        1
    );
}

#[test]
fn cross_identity_evidence_family_change_is_rejected_and_not_treated_as_equivalent() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let compute_calls = AtomicU32::new(0);
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("family-mismatch-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-family-mismatch");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity(tx, "source", "shape-v1", "mesh-001")
        })
        .unwrap();

    mark_dirty(runtime.graph_mut(), alias_node, ASPECT_A).unwrap();

    let err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_lineage_mapping(
                tx,
                "source",
                "shape-v1",
                "mesh-001",
            )
        })
        .expect_err("changing correspondence evidence family should be rejected");
    assert!(err.to_string().contains("reuse certification failed"));
    assert_eq!(
        compute_calls.load(Ordering::Relaxed),
        1,
        "family mismatch should not silently degrade into recompute"
    );

    let runtime_state = runtime
        .graph()
        .observe()
        .runtime_artifact_state(alias_node)
        .unwrap()
        .cloned()
        .expect("runtime artifact state");
    assert_eq!(
        runtime_state
            .reuse_boundary_context
            .as_ref()
            .and_then(|ctx| ctx.persistent_correspondence.as_ref()),
        Some(&crate::data::reuse::PersistentCorrespondenceEvidence::HostSuppliedKey(
            "mesh-001".to_string()
        )),
        "distinct correspondence families must remain semantically distinct after rejection"
    );
    assert_eq!(
        runtime.observe().metrics().evaluation.reuse_rejected_persistent_correspondence_invalid_count,
        1
    );
}

#[test]
fn ambiguous_lineage_mapping_is_rejected_before_cross_identity_reuse_commits() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("ambiguous-lineage-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-ambiguous");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();

    let err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_lineage_mapping(
                tx,
                "source",
                "shape-v1",
                "lineage-map:mesh-42->mesh-77|mesh-99",
            )
        })
        .expect_err("ambiguous lineage mapping should be rejected");
    assert!(err.to_string().contains("reuse certification failed"));
    assert_eq!(
        runtime.observe().metrics().evaluation.reuse_rejected_persistent_correspondence_invalid_count,
        1
    );
    assert!(
        runtime
            .graph()
            .observe()
            .runtime_artifact_state(alias_node)
            .unwrap()
            .is_none(),
        "failed first admission must not commit runtime artifact state"
    );
}

#[test]
fn cross_identity_lineage_and_history_preserve_correspondence_family() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("lineage-family-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-lineage-family");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_lineage_mapping(
                tx,
                "source",
                "shape-v1",
                "lineage-map:mesh-42->mesh-77",
            )
        })
        .unwrap();

    let replay_event = runtime
        .graph()
        .replay_events()
        .iter()
        .rev()
        .find(|event| {
            event.kind == ReplayEventKind::TaskApplied && event.node == Some(alias_node)
        })
        .expect("cross-identity replay event");
    assert_eq!(
        replay_event.persistent_correspondence_kind,
        Some(crate::data::reuse::PersistentCorrespondenceKind::LineageBackedMapping)
    );

    let history = runtime
        .observe()
        .execution_history_summary(DiagnosticsProfile::Development);
    let node_summary = history
        .nodes
        .iter()
        .find(|node| node.node == alias_node)
        .expect("history summary for alias");
    assert_eq!(
        node_summary.persistent_correspondence_kind,
        Some(crate::data::reuse::PersistentCorrespondenceKind::LineageBackedMapping)
    );

    let lineage = runtime.observe().lineage_chain_for_node(alias_node);
    assert!(lineage.iter().any(|record| matches!(
        &record.kind,
        LineageRecordKind::ArtifactTransition {
            transition:
                ArtifactTransitionKind::CrossIdentityPersistentReuse {
                    correspondence_kind:
                        crate::data::reuse::PersistentCorrespondenceKind::LineageBackedMapping
                },
            ..
        }
    )));
}

#[test]
fn branch_local_cross_identity_rejection_preserves_main_correspondence_and_lineage() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("branch-correspondence-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-branch");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity(tx, "source", "shape-v1", "mesh-branch-001")
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let main_lineage_before = runtime.observe().lineage_chain_for_node(alias_node);
    let main_replay_before = runtime.observe().replay_for_branch(main.id);
    let main_artifact_before = runtime
        .observe()
        .current_lineage_artifact(alias_node)
        .expect("main branch should have a lineage artifact");

    let feature = runtime.create_branch("feature-cross-identity").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    mark_dirty(runtime.graph_mut(), alias_node, ASPECT_A).unwrap();

    let err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_lineage_mapping(
                tx,
                "source",
                "shape-v1",
                "lineage-map:mesh-branch-001->mesh-branch-777",
            )
        })
        .expect_err("feature branch should reject stale cross-identity evidence");
    assert!(err.to_string().contains("reuse certification failed"));
    assert_eq!(
        runtime.observe().current_lineage_artifact(alias_node),
        Some(main_artifact_before),
        "failed feature-branch admission must not replace the branch-local committed artifact"
    );
    assert_eq!(
        runtime
            .graph()
            .observe()
            .runtime_artifact_state(alias_node)
            .unwrap()
            .and_then(|state| state.reuse_boundary_context.as_ref())
            .and_then(|ctx| ctx.persistent_correspondence.as_ref()),
        Some(&crate::data::reuse::PersistentCorrespondenceEvidence::HostSuppliedKey(
            "mesh-branch-001".to_string()
        )),
        "feature branch should preserve the last committed certified correspondence after rejection"
    );
    let feature_replay_after = runtime.observe().replay_for_branch(feature.id);
    assert!(
        feature_replay_after
            .frames
            .iter()
            .all(|frame| frame.branch_id == feature.id),
        "feature replay must remain branch-local after stale correspondence rejection"
    );

    runtime.switch_branch(main.clone()).unwrap();
    assert_eq!(
        runtime.observe().lineage_chain_for_node(alias_node),
        main_lineage_before,
        "feature-branch rejection must not contaminate main-branch lineage"
    );
    let main_replay_after = runtime.observe().replay_for_branch(main.id);
    assert_eq!(
        main_replay_after
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::TaskApplied)
            .count(),
        main_replay_before
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::TaskApplied)
            .count(),
        "feature-branch rejection must not append task-apply replay on main"
    );
    assert_eq!(
        main_replay_after
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::TransactionCommitted)
            .count(),
        main_replay_before
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::TransactionCommitted)
            .count(),
        "feature-branch rejection must not append committed execution replay on main"
    );
    assert_eq!(
        runtime
            .graph()
            .observe()
            .runtime_artifact_state(alias_node)
            .unwrap()
            .and_then(|state| state.reuse_boundary_context.as_ref())
            .and_then(|ctx| ctx.persistent_correspondence.as_ref()),
        Some(&crate::data::reuse::PersistentCorrespondenceEvidence::HostSuppliedKey(
            "mesh-branch-001".to_string()
        )),
        "main branch should retain its original certified correspondence"
    );
}

#[test]
fn branch_local_cross_identity_history_retains_committed_family_after_rejected_evolution() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("branch-history-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias-branch-history");
    let alias_node = alias.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| source.evaluate_memoized(tx, "shape-v1"))
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_contract_basis(
                tx,
                "source",
                "shape-v1",
                "contract:mesh-family:v2",
            )
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-cross-identity-history").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    mark_dirty(runtime.graph_mut(), alias_node, ASPECT_A).unwrap();
    let _ = runtime.transaction(&mut runtime_ctx, |tx| {
        alias.evaluate_cross_identity_with_region_identity(
            tx,
            "source",
            "shape-v1",
            "region:wing",
        )
    });

    let feature_history = runtime
        .observe()
        .execution_history_summary(DiagnosticsProfile::Development);
    let feature_summary = feature_history
        .nodes
        .iter()
        .find(|node| node.node == alias_node)
        .expect("feature summary for alias");
    assert_eq!(
        feature_summary.reuse_origin,
        Some(crate::data::reuse::ReuseOrigin::CrossIdentityPersistentReuse),
        "rejected evolution must not erase the last committed advanced reuse origin"
    );
    assert_eq!(
        feature_summary.persistent_correspondence_kind,
        Some(crate::data::reuse::PersistentCorrespondenceKind::ContractDeclaredBasis),
        "history should keep the committed correspondence family after rejected branch-local evolution"
    );

    runtime.switch_branch(main.clone()).unwrap();
    let main_history = runtime
        .observe()
        .execution_history_summary(DiagnosticsProfile::Development);
    let main_summary = main_history
        .nodes
        .iter()
        .find(|node| node.node == alias_node)
        .expect("main summary for alias");
    assert_eq!(
        main_summary.persistent_correspondence_kind,
        Some(crate::data::reuse::PersistentCorrespondenceKind::ContractDeclaredBasis),
        "main branch should still report the committed correspondence family"
    );
}

#[test]
fn defined_computation_evaluate_partial_splice_uses_public_api() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let compute_calls = AtomicU32::new(0);
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_partial_artifact_splicing()
                .with_partition_scope(PartitionSubscription::whole_partition("wing")),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("splice-artifact")
                        .with_output_change(OutputChange::Refreshed)
                        .with_changed_region(ChangedRegion::new("wing")),
                ))
            },
        })
        .unwrap();
    let wing = projection.keyed("wing");
    let node = wing.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| wing.evaluate_memoized(tx, "shape-v1"))
        .unwrap();

    mark_dirty(runtime.graph_mut(), node, ASPECT_A).unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            wing.evaluate_partial_splice(
                tx,
                "shape-v1",
                [PartitionSubscription::whole_partition("wing")],
            )
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 1);
    let explanation = runtime.observe().explain(node).unwrap();
    let reuse_basis = explanation.reuse_basis.expect("partial splice reuse basis");
    assert_eq!(
        reuse_basis.strategy,
        Some(crate::data::reuse::ReuseStrategy::PartialArtifactSplicing)
    );
    assert_eq!(reuse_basis.source, ReuseSource::PartialComposition);
    assert_eq!(reuse_basis.crossing, ReuseCrossing::CompositionBoundary);
    assert_eq!(reuse_basis.partition_region_basis_count, 1);
    assert_eq!(
        explanation.reuse_origin,
        Some(crate::data::reuse::ReuseOrigin::PartialArtifactSplice)
    );
    let replay_event = runtime
        .graph()
        .replay_events()
        .iter()
        .rev()
        .find(|event| event.kind == ReplayEventKind::TaskApplied && event.node == Some(node))
        .expect("partial splice replay event");
    assert_eq!(
        replay_event.reuse_origin,
        Some(crate::data::reuse::ReuseOrigin::PartialArtifactSplice)
    );
    let history = runtime
        .observe()
        .execution_history_summary(DiagnosticsProfile::Development);
    assert_eq!(
        history
            .reuse_origin_counts
            .get("PartialArtifactSplice")
            .copied(),
        Some(1)
    );
    assert!(history.nodes.iter().any(|entry| {
        entry.node == node
            && entry.reuse_origin == Some(crate::data::reuse::ReuseOrigin::PartialArtifactSplice)
    }));
    assert_eq!(
        runtime.observe().metrics().evaluation.partial_artifact_splice_count,
        1
    );
}

#[test]
fn memoization_is_scoped_by_family() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family_a = define_keyed_computation(&mut runtime, "projection-a", ());
    let family_b = define_keyed_computation(&mut runtime, "projection-b", ());
    let keyed_a = family_a.keyed("bulkhead");
    let keyed_b = family_b.keyed("bulkhead");
    let node_a = keyed_a.node(&mut runtime);
    let node_b = keyed_b.node(&mut runtime);
    let computation_a = keyed_a.memoized("shape-v1");
    let computation_b = keyed_b.memoized("shape-v1");
    let mut runtime_ctx = ();
    let compute_calls = AtomicU32::new(0);

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node_a, &computation_a, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("a"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node_b, &computation_b, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("b"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 2);
}

#[test]
fn memoization_write_is_discarded_on_rollback() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family = define_keyed_computation(&mut runtime, "projection", ());
    let keyed = family.keyed("bulkhead");
    let node = keyed.node(&mut runtime);
    let computation = keyed.memoized("shape-v1");
    let mut runtime_ctx = ();
    let compute_calls = AtomicU32::new(0);

    let err = runtime.transaction(&mut runtime_ctx, |tx| {
        tx.evaluate_keyed(node, &computation, &|view| {
            compute_calls.fetch_add(1, Ordering::Relaxed);
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("cached"),
            ))
        })?;
        Err(SignalError::invalid_input("force rollback"))
    });
    assert!(err.is_err());

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node, &computation, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("fresh"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 2);
    let metrics = runtime.observe().metrics();
    assert_eq!(metrics.evaluation.memoization_hits, 0);
    assert_eq!(metrics.evaluation.memoization_misses, 2);
}

#[test]
fn aborted_keyed_evaluation_does_not_leak_key_registry_growth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().build();
    let family = ComputationFamily::from("fresh-family");
    let computation = KeyedComputation::new(family.clone(), "fresh-key").with_memo_key("fresh-v1");
    let before = runtime.config().test_registry_counts();
    let mut runtime_ctx = ();

    let err = runtime.transaction(&mut runtime_ctx, |tx| {
        tx.evaluate_keyed(node, &computation, &|view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("cached"),
            ))
        })?;
        Err(SignalError::invalid_input("force rollback"))
    });
    assert!(err.is_err());

    assert_eq!(
        runtime.config().test_registry_counts(),
        before,
        "aborted keyed evaluation must not leak family/key/memo registry entries"
    );
}

#[test]
fn partition_subscribers_only_dirty_on_matching_partition() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let wing_subscriber = graph.node().build();
    let tail_subscriber = graph.node().build();
    graph
        .append_partition_dependency(wing_subscriber, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .append_partition_dependency(tail_subscriber, source, ASPECT_A, "tail")
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing")))
    };
    let mut subscriber_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, wing_subscriber, &mut subscriber_compute).unwrap();
    evaluate(&mut graph, tail_subscriber, &mut subscriber_compute).unwrap();

    mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")]).unwrap();

    assert_eq!(graph.get_state(wing_subscriber).unwrap(), NodeState::Dirty);
    assert_eq!(
        graph.get_state(tail_subscriber).unwrap(),
        NodeState::MaybeStale
    );
    assert_eq!(
        graph
            .observe()
            .metrics()
            .invalidation
            .partition_match_dirty_count,
        1
    );
    assert_eq!(
        graph
            .observe()
            .metrics()
            .invalidation
            .partition_scoped_invalidation_checks,
        2
    );
}

#[test]
fn detail_sensitive_partition_subscriber_reverts_clean_when_detail_does_not_match() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let subscriber = graph.node().build();
    graph
        .append_partition_detail_dependency(subscriber, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let mut source_rib_12 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    };
    let mut source_rib_13 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")))
    };
    let mut subscriber_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_rib_12).unwrap();
    evaluate(&mut graph, subscriber, &mut subscriber_compute).unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .unwrap();
    assert_eq!(graph.get_state(subscriber).unwrap(), NodeState::MaybeStale);

    evaluate(&mut graph, source, &mut source_rib_13).unwrap();
    evaluate(&mut graph, subscriber, &mut subscriber_compute).unwrap();

    assert_eq!(graph.get_state(subscriber).unwrap(), NodeState::Clean);
    let explanation = graph.observe().explain(subscriber).unwrap();
    assert!(matches!(
        explanation.upstream.as_slice(),
        [UpstreamCause::Clean { subscription: Some(subscription), .. }]
        if subscription.partition == PartitionToken::new("wing")
            && subscription.detail.as_deref() == Some("rib-12")
    ));
    assert_eq!(
        graph
            .observe()
            .metrics()
            .invalidation
            .partition_scope_revert_clean_count,
        1
    );
}

#[test]
fn mixed_whole_aspect_and_partition_subscribers_behave_deterministically() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let whole_aspect_subscriber = graph.node().build();
    let matching_partition_subscriber = graph.node().build();
    let non_matching_partition_subscriber = graph.node().build();
    let mut dependencies = DependencyBatchBuilder::new(&mut graph);
    dependencies
        .append_dependency(whole_aspect_subscriber, source, ASPECT_A)
        .unwrap()
        .append_partition_dependency(matching_partition_subscriber, source, ASPECT_A, "wing")
        .unwrap()
        .append_partition_dependency(non_matching_partition_subscriber, source, ASPECT_A, "tail")
        .unwrap();
    dependencies.commit().unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing")))
    };
    let mut source_v2 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_changed_region(ChangedRegion::new("wing")))
    };
    let mut subscriber_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, whole_aspect_subscriber, &mut subscriber_compute).unwrap();
    evaluate(
        &mut graph,
        matching_partition_subscriber,
        &mut subscriber_compute,
    )
    .unwrap();
    evaluate(
        &mut graph,
        non_matching_partition_subscriber,
        &mut subscriber_compute,
    )
    .unwrap();

    mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")]).unwrap();

    assert_eq!(
        graph.get_state(whole_aspect_subscriber).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        graph.get_state(matching_partition_subscriber).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        graph.get_state(non_matching_partition_subscriber).unwrap(),
        NodeState::MaybeStale
    );

    evaluate(&mut graph, source, &mut source_v2).unwrap();
    evaluate(
        &mut graph,
        non_matching_partition_subscriber,
        &mut subscriber_compute,
    )
    .unwrap();
    assert_eq!(
        graph.get_state(non_matching_partition_subscriber).unwrap(),
        NodeState::Clean
    );
}

#[test]
fn partition_scoped_cleanup_does_not_hide_other_dirty_upstreams() {
    let mut graph = SignalGraph::new();
    let source_partitioned = graph.node().partitioned_output().build();
    let source_other = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(
            dependent,
            source_partitioned,
            ASPECT_A,
            "wing",
            "rib-12",
        )
        .unwrap();
    graph
        .append_dependency(dependent, source_other, ASPECT_B)
        .unwrap();

    let mut partitioned_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    };
    let mut partitioned_v2_other_detail = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")))
    };
    let mut other_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 10)))
    };

    evaluate(&mut graph, source_partitioned, &mut partitioned_v1).unwrap();
    evaluate(&mut graph, source_other, &mut other_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source_partitioned,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .unwrap();
    mark_dirty(&mut graph, source_other, ASPECT_B).unwrap();
    evaluate(
        &mut graph,
        source_partitioned,
        &mut partitioned_v2_other_detail,
    )
    .unwrap();

    assert_ne!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn transaction_mark_dirty_with_regions_routes_partition_matches() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().partitioned_output().build();
    let matching = runtime.graph_mut().node().build();
    let non_matching = runtime.graph_mut().node().build();
    let mut dependencies = DependencyBatchBuilder::new(runtime.graph_mut());
    dependencies
        .append_partition_dependency(matching, source, ASPECT_A, "wing")
        .unwrap()
        .append_partition_dependency(non_matching, source, ASPECT_A, "tail")
        .unwrap();
    dependencies.commit().unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.mark_dirty_with_regions(source, ASPECT_A, &[ChangedRegion::new("wing")])?;
            Ok(())
        })
        .unwrap();

    assert_eq!(
        runtime.graph().get_state(matching).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        runtime.graph().get_state(non_matching).unwrap(),
        NodeState::MaybeStale
    );
}

#[test]
fn partition_scoped_runtime_reads_do_not_widen_captured_dependencies() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().partitioned_output().build();
    let matching = runtime.graph_mut().node().build();
    let non_matching = runtime.graph_mut().node().build();
    let mut dependencies = DependencyBatchBuilder::new(runtime.graph_mut());
    dependencies
        .append_partition_dependency(matching, source, ASPECT_A, "wing")
        .unwrap()
        .append_partition_dependency(non_matching, source, ASPECT_A, "tail")
        .unwrap();
    dependencies.commit().unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")),
                ))
            })?;
            tx.read(matching, &|view| {
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::whole_partition("wing"),
                )?;
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(10, 0))))
            })?;
            tx.read(non_matching, &|view| {
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::whole_partition("tail"),
                )?;
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(20, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.mark_dirty_with_regions(
                source,
                ASPECT_A,
                &[ChangedRegion::new("wing").with_detail("rib-12")],
            )?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(
        runtime.graph().get_state(matching).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        runtime.graph().get_state(non_matching).unwrap(),
        NodeState::MaybeStale
    );
}

#[test]
fn transaction_rollback_after_partition_local_evaluation_restores_clean_states() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().partitioned_output().build();
    let matching = runtime.graph_mut().node().build();
    let non_matching = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_partition_detail_dependency(matching, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    runtime
        .graph_mut()
        .append_partition_dependency(non_matching, source, ASPECT_A, "tail")
        .unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")),
                ))
            })?;
            tx.read(matching, &|view| {
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::partition_and_detail("wing", "rib-12"),
                )?;
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(10, 0))))
            })?;
            tx.read(non_matching, &|view| {
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::whole_partition("tail"),
                )?;
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(20, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let err = runtime.transaction(&mut (), |tx| {
        tx.mark_dirty_with_regions(
            source,
            ASPECT_A,
            &[ChangedRegion::new("wing").with_detail("rib-12")],
        )?;
        tx.read(source, &|view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(2, 0))
                    .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")),
            ))
        })?;
        tx.read(matching, &|view| {
            let _ = view.read_partitioned_aspect_version(
                source,
                ASPECT_A,
                PartitionSubscription::partition_and_detail("wing", "rib-12"),
            )?;
            Ok(view.finish(NodeEvaluationResult::from_version(version_ab(11, 0))))
        })?;
        Err(SignalError::invalid_input("rollback localized wave"))
    });
    assert!(err.is_err());

    assert_eq!(runtime.graph().get_state(source).unwrap(), NodeState::Clean);
    assert_eq!(
        runtime.graph().get_state(matching).unwrap(),
        NodeState::Clean
    );
    assert_eq!(
        runtime.graph().get_state(non_matching).unwrap(),
        NodeState::Clean
    );
}

#[test]
fn committed_partition_local_evaluation_preserves_changed_region_explanation_and_metrics() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().partitioned_output().build();
    let matching = runtime.graph_mut().node().build();
    let non_matching = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_partition_detail_dependency(matching, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    runtime
        .graph_mut()
        .append_partition_dependency(non_matching, source, ASPECT_A, "tail")
        .unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")),
                ))
            })?;
            tx.read(matching, &|view| {
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::partition_and_detail("wing", "rib-12"),
                )?;
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(10, 0))))
            })?;
            tx.read(non_matching, &|view| {
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::whole_partition("tail"),
                )?;
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(20, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.mark_dirty_with_regions(
                source,
                ASPECT_A,
                &[ChangedRegion::new("wing").with_detail("rib-12")],
            )?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")),
                ))
            })?;
            tx.read(matching, &|view| {
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::partition_and_detail("wing", "rib-12"),
                )?;
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(11, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(
        runtime.graph().get_state(matching).unwrap(),
        NodeState::Clean
    );
    assert_eq!(
        runtime.graph().get_state(non_matching).unwrap(),
        NodeState::MaybeStale
    );
    let explanation = runtime.observe().explain(source).unwrap();
    assert!(explanation.changed_regions.iter().any(|region| {
        region.partition == PartitionToken::new("wing")
            && region.detail.as_deref() == Some("rib-12")
    }));
}

#[test]
fn transaction_partition_invalidations_union_dirty_scopes_until_runtime_evaluation() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().partitioned_output().build();
    let dependent = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    runtime
        .graph_mut()
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-13")
        .unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12"))
                        .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")),
                ))
            })?;
            tx.read(dependent, &|view| {
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::partition_and_detail("wing", "rib-12"),
                )?;
                let _ = view.read_partitioned_aspect_version(
                    source,
                    ASPECT_A,
                    PartitionSubscription::partition_and_detail("wing", "rib-13"),
                )?;
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(10, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.mark_dirty_with_regions(
                source,
                ASPECT_A,
                &[ChangedRegion::new("wing").with_detail("rib-12")],
            )?;
            tx.mark_dirty_with_regions(
                source,
                ASPECT_A,
                &[ChangedRegion::new("wing").with_detail("rib-13")],
            )?;
            Ok(())
        })
        .unwrap();

    let entry = runtime.graph().get_entry(dependent).unwrap();
    let scopes = entry.get_dirty_partition_scopes();
    assert_eq!(entry.get_state(), &NodeState::Dirty);
    assert!(scopes
        .iter()
        .any(|scope| scope.detail.as_deref() == Some("rib-12")));
    assert!(scopes
        .iter()
        .any(|scope| scope.detail.as_deref() == Some("rib-13")));
}

#[test]
fn sparse_partition_fanout_keeps_most_subscribers_out_of_dirty_state() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let mut subscribers = Vec::new();
    for index in 0..128 {
        let subscriber = graph.node().build();
        graph
            .append_partition_dependency(subscriber, source, ASPECT_A, format!("partition-{index}"))
            .unwrap();
        subscribers.push(subscriber);
    }

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("partition-7")))
    };
    let mut subscriber_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    for &subscriber in &subscribers {
        evaluate(&mut graph, subscriber, &mut subscriber_compute).unwrap();
    }

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("partition-7")],
    )
    .unwrap();

    let dirty_count = subscribers
        .iter()
        .filter(|&&subscriber| graph.get_state(subscriber).unwrap() == NodeState::Dirty)
        .count();
    let maybe_stale_count = subscribers
        .iter()
        .filter(|&&subscriber| graph.get_state(subscriber).unwrap() == NodeState::MaybeStale)
        .count();

    assert_eq!(dirty_count, 1);
    assert_eq!(maybe_stale_count, 127);
    assert_eq!(
        graph
            .observe()
            .metrics()
            .invalidation
            .partition_scoped_invalidation_checks,
        128
    );
    assert_eq!(
        graph
            .observe()
            .metrics()
            .invalidation
            .partition_match_dirty_count,
        1
    );
}
