use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::graph::BranchStructuralDelta;
use crate::facade::*;
use crate::logic::transaction::{BranchMergeResolutionRequirement, ConflictResolutionStrategy};
use crate::schema::data::{
    SignalSchemaDescriptor, SignalSchemaId, SignalSchemaName, SignalSchemaRegistration,
    SignalSchemaRegistry, SignalSchemaVersion,
};
use crate::tests::support::*;

fn merge_schema_registry(
    default_strategy_name: &str,
    default_conflict_policy_name: Option<&str>,
    default_identity_matcher_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(41),
            SignalSchemaName::new("signal.demo.merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(default_strategy_name)),
            default_conflict_policy_name.map(ConflictPolicyName::new),
            default_identity_matcher_name.map(IdentityMatcherName::new),
            None,
            None,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

fn cross_identity_merge_schema_registry(
    default_identity_matcher_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(42),
            SignalSchemaName::new("signal.demo.cross-identity-merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard().with_cross_identity_persistent_matching(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            default_identity_matcher_name.map(IdentityMatcherName::new),
            None,
            None,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

fn source_only_merge_schema_registry(
    default_source_only_policy_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(43),
            SignalSchemaName::new("signal.demo.source-only-merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            None,
            default_source_only_policy_name.map(SourceOnlyPolicyName::new),
            None,
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

fn deletion_merge_schema_registry(
    default_deletion_policy_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics(
            SignalSchemaId(44),
            SignalSchemaName::new("signal.demo.deletion-merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            None,
            None,
            default_deletion_policy_name.map(DeletionPolicyName::new),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

fn aspect_policy_merge_schema_registry(
    default_aspect_policy_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics_and_aspects(
            SignalSchemaId(44),
            SignalSchemaName::new("signal.demo.aspect-merge-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            None,
            None,
            None,
            None,
            default_aspect_policy_name
                .map(|name| {
                    vec![AspectMergePolicyBinding::new(
                        ASPECT_A,
                        AspectMergePolicyName::new(name),
                    )]
                })
                .unwrap_or_default(),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

fn conflict_isolation_merge_schema_registry(
    default_conflict_isolation_policy_name: Option<&str>,
) -> SignalSchemaRegistry {
    SignalSchemaRegistry::from_registrations(vec![SignalSchemaRegistration::new(
        SignalSchemaDescriptor::new_with_merge_semantics_and_isolation(
            SignalSchemaId(45),
            SignalSchemaName::new("signal.demo.conflict-isolation-owned"),
            SignalSchemaVersion::new(1, 0),
            NodeContract::wildcard(),
            Some(MergeStrategyName::new(
                "signal.merge.rebase-source-onto-target",
            )),
            Some(ConflictPolicyName::new(
                "signal.conflict.resolve-source-when-structure-matches",
            )),
            None,
            None,
            None,
            default_conflict_isolation_policy_name.map(ConflictIsolationPolicyName::new),
        ),
    )
    .expect("valid schema registration")])
    .expect("valid schema registry")
}

#[test]
fn merge_branch_introduces_source_only_node_with_new_target_id_and_merge_traces() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("shared-main"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-introduce").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    let source_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .append_dependency(source_only, shared, ASPECT_A)
        .unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(upstream)
                        .with_output_identity("feature-only"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let source_artifact_id = runtime
        .observe()
        .current_lineage_artifact(source_only)
        .expect("source-only node should materialize a lineage artifact");

    runtime.switch_branch(main.clone()).unwrap();
    let main_node_count_before = runtime.graph().active_node_count();

    let result = runtime.merge_branch(feature.clone(), main.clone()).unwrap();
    assert_eq!(
        result.reconciliation_policy.conflict,
        ConflictMergePolicy::ResolveSourceStateWhenStructureMatches
    );
    let introduced = result
        .records
        .iter()
        .find(|record| {
            record.source_node == source_only
                && matches!(record.action, ArtifactMergeAction::IntroducedIntoTarget)
        })
        .expect("merge should report the introduced source-only node");
    let introduced_target = introduced
        .target_node
        .expect("introduced source-only node should allocate a target node");

    assert_ne!(
        introduced_target, source_only,
        "source-only adoption must allocate a distinct target node id"
    );
    assert!(
        runtime.graph().is_alive(introduced_target),
        "introduced target node should exist on the merged target branch"
    );
    assert_eq!(
        runtime.graph().active_node_count(),
        main_node_count_before + 1,
        "merging a source-only node into the active target should increase target node count"
    );
    assert_eq!(
        runtime
            .graph()
            .dependencies_of(introduced_target)
            .unwrap()
            .iter()
            .map(|edge| edge.source())
            .collect::<Vec<_>>(),
        vec![shared],
        "introduced target node dependencies should be remapped to target authority ids"
    );
    assert_ne!(
        introduced.target_artifact_id_after,
        Some(source_artifact_id),
        "introduced target nodes must not reuse the source branch artifact lineage id by default"
    );
    assert!(
        runtime
            .graph()
            .replay_events()
            .iter()
            .any(|event| event.kind == ReplayEventKind::BranchMerged),
        "merge should emit a branch-level replay boundary"
    );
    assert!(
        runtime
            .graph()
            .observe()
            .lineage_records()
            .iter()
            .any(|record| matches!(record.kind, LineageRecordKind::BranchMerge { .. })),
        "merge should emit branch merge lineage"
    );
    assert!(
        runtime
            .graph()
            .observe()
            .lineage_records()
            .iter()
            .any(|record| matches!(record.kind, LineageRecordKind::ArtifactMerge { .. })),
        "merge should emit artifact merge lineage"
    );
}

#[test]
fn branch_mutation_journal_captures_structural_dependency_snapshot_and_artifact_deltas() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let upstream = runtime.graph_mut().node().output_identity().build();
    let downstream = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .append_dependency(downstream, upstream, ASPECT_A)
        .unwrap();

    let mut runtime_ctx = ();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(downstream, &|view| {
                let result = if view.node() == upstream {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(70, 0))
                            .with_output_identity("journal-upstream"),
                    )
                } else {
                    let version = view.read_aspect_version(upstream, ASPECT_A)?;
                    view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("journal-downstream"),
                    )
                };
                Ok(result)
            })?;
            Ok(())
        })
        .unwrap();

    let record = runtime
        .graph()
        .branch_mutation_records()
        .into_iter()
        .find(|(node, _)| *node == downstream)
        .map(|(_, record)| record)
        .expect("downstream node should have structural mutation journal");

    assert!(record.structural_deltas.iter().any(|delta| matches!(
        delta,
        BranchStructuralDelta::DependencyTopologyChanged(topology)
            if topology.added_edges.len() == 1 && topology.removed_edges.is_empty()
    )));
    assert!(record.structural_deltas.iter().any(|delta| matches!(
        delta,
        BranchStructuralDelta::DependencySnapshotChanged(snapshot)
            if snapshot.next_entry_count >= 1 && snapshot.changed_entry_count >= 1
    )));
    assert!(record.structural_deltas.iter().any(|delta| matches!(
        delta,
        BranchStructuralDelta::RuntimeArtifactChanged(artifact)
            if artifact.next_output_hash.is_some() && artifact.next_reuse_basis.is_some()
    )));
}

#[test]
fn branch_mutation_journal_slice_preserves_structural_records_for_overlap_filtering() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().output_identity().build();
    let other = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(80, 0))
                        .with_output_identity("journal-source"),
                ))
            })?;
            tx.read(other, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(81, 0))
                        .with_output_identity("journal-other"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let mut ledger = BranchMutationLedger::default();
    ledger.absorb_records(runtime.graph().branch_mutation_records());
    let slice = ledger.structural_merge_journal();
    let filtered = BranchMutationJournalSlice {
        records: slice
            .records
            .iter()
            .filter(|record| record.node == source)
            .cloned()
            .collect(),
    };

    assert_eq!(slice.candidate_nodes().len(), 2);
    assert!(filtered.contains_node(source));
    assert!(!filtered.contains_node(other));
    assert!(filtered.records[0]
        .structural_deltas
        .iter()
        .any(|delta| matches!(delta, BranchStructuralDelta::RuntimeArtifactChanged(_))));
}

#[test]
fn merge_branch_introduces_multiple_source_only_nodes_with_internal_dependencies() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-chain").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    let upstream = runtime.graph_mut().node().output_identity().build();
    let downstream = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .append_dependency(downstream, upstream, ASPECT_A)
        .unwrap();

    let mut runtime_ctx = ();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(downstream, &|view| {
                let result = if view.node() == upstream {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(3, 0))
                            .with_output_identity("feature-upstream"),
                    )
                } else {
                    let version = view.read_aspect_version(upstream, ASPECT_A)?;
                    view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("feature-downstream"),
                    )
                };
                Ok(result)
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let result = runtime.merge_branch(feature, main).unwrap();

    let introduced_upstream = result
        .records
        .iter()
        .find(|record| record.source_node == upstream)
        .and_then(|record| record.target_node)
        .expect("upstream node should be introduced into target");
    let introduced_downstream = result
        .records
        .iter()
        .find(|record| record.source_node == downstream)
        .and_then(|record| record.target_node)
        .expect("downstream node should be introduced into target");

    assert_eq!(
        runtime
            .graph()
            .dependencies_of(introduced_downstream)
            .unwrap()
            .iter()
            .map(|edge| edge.source())
            .collect::<Vec<_>>(),
        vec![introduced_upstream],
        "introduced internal dependencies must remap to introduced target node ids"
    );
}

#[test]
fn merge_branch_skips_non_adoptable_source_only_nodes() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-non-adoptable").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    let source_only = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(5, 0))
                        .with_output_identity("non-adoptable"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    {
        let mut graph = runtime.graph_mut();
        let mut entry = graph.get_entry_mut(source_only).unwrap();
        let mut runtime_artifact = entry
            .get_runtime_artifact_state()
            .cloned()
            .expect("source-only node should have runtime artifact state");
        runtime_artifact.warm_mut().merge_authority = ArtifactMergeAuthority {
            authority_class: ArtifactAuthorityClass::BranchLocalSpeculative,
            adoptability: MergeAdoptability::NonAdoptableBranchLocal,
        };
        entry.set_runtime_artifact_state(Some(runtime_artifact));
    }

    runtime.switch_branch(main.clone()).unwrap();
    let main_node_count_before = runtime.graph().active_node_count();
    let result = runtime.merge_branch(feature, main).unwrap();
    let skipped = result
        .records
        .iter()
        .find(|record| record.source_node == source_only)
        .expect("merge should still report the skipped source-only node");

    assert!(
        matches!(skipped.action, ArtifactMergeAction::SkippedNonAdoptable),
        "non-adoptable source-only nodes should be skipped explicitly"
    );
    assert_eq!(
        skipped.target_node, None,
        "skipped source-only nodes must not allocate a target identity"
    );
    assert_eq!(
        runtime.graph().active_node_count(),
        main_node_count_before,
        "skipped non-adoptable nodes must not change target authority breadth"
    );
}

#[test]
fn merge_branch_counters_and_summary_surface_match_introduced_work() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-counters").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    let source_only = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(7, 0))
                        .with_output_identity("counter-node"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main).unwrap();
    let result = runtime
        .merge_branch(feature, runtime.observe().current_branch())
        .unwrap();

    assert_eq!(
        result.counters.introduced_node_count, 1,
        "merge counters should expose introduced-node adoption work"
    );
    assert_eq!(
        result.counters.replay_event_count, 1,
        "merge counters should reflect the branch-level replay boundary"
    );
    assert_eq!(
        result.counters.merge_lineage_record_count,
        (result.records.len() + 1) as u64,
        "merge counters should account for one branch merge record plus per-node artifact merge records"
    );
    assert!(
        matches!(
            result.boundary_witness.kind,
            MergeBoundaryWitnessKind::MutationJournalBoundary
        ),
        "tracked branch-local mutation scope should surface a bounded merge witness"
    );
}

#[test]
fn merge_branch_uses_branch_local_mutation_scope_instead_of_whole_live_scan() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared_a = runtime.graph_mut().node().output_identity().build();
    let shared_b = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared_a, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(11, 0))
                        .with_output_identity("shared-a"),
                ))
            })?;
            tx.read(shared_b, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(12, 0))
                        .with_output_identity("shared-b"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-narrow-scope").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    let source_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .append_dependency(source_only, shared_a, ASPECT_A)
        .unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source_only, &|view| {
                let upstream = view.read_aspect_version(shared_a, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(upstream)
                        .with_output_identity("feature-narrow"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let result = runtime.merge_branch(feature, main).unwrap();

    assert!(
        result.counters.final_candidate_breadth == result.planned_candidates.nodes.len() as u64,
        "tracked branch-local mutations should lower an explicit planned candidate set"
    );
    assert!(
        result.counters.source_slice_breadth >= result.counters.final_candidate_breadth,
        "candidate-node merge should stay bounded by the source journal slice"
    );
    assert!(
        result.records.len() < runtime.graph().active_node_count(),
        "narrow candidate scope should plan fewer nodes than the full live authority surface"
    );
}

#[test]
fn proof_minimal_overlap_and_conservative_expansion_remain_distinct_and_bounded() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(13, 0))
                        .with_output_identity("proof-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-proof-overlap").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    let source_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .append_dependency(source_only, shared, ASPECT_A)
        .unwrap();

    runtime.switch_branch(main).unwrap();
    let result = runtime
        .merge_branch(feature, runtime.observe().current_branch())
        .unwrap();

    assert!(
        result.proof_minimal_overlap.shared_nodes.is_empty(),
        "source-only introductions should not fabricate shared-node minimal overlap"
    );
    assert_eq!(
        result.planned_candidates.nodes,
        vec![source_only],
        "final merge candidates should remain bounded to the source mutation set"
    );
    assert!(
        result.conservative_overlap.support_nodes.contains(&shared),
        "conservative overlap should expand only to the target support surface needed for remaps"
    );
    assert!(
        result.counters.proof_minimal_overlap_breadth
            < result.counters.conservative_overlap_expansion_breadth,
        "conservative expansion should be measurably broader than proof-minimal overlap here"
    );
    assert!(
        result.counters.final_candidate_breadth
            < result.counters.conservative_overlap_expansion_breadth,
        "support expansion should remain distinct from the final candidate set"
    );
}

#[test]
fn merge_candidate_construction_is_identical_with_and_without_convenience_branch_indexes() {
    let prepare_runtime = || {
        let mut runtime = SignalRuntime::builder(SignalGraph::new())
            .with_kernel_defaults()
            .build();
        let shared = runtime.graph_mut().node().output_identity().build();
        let mut runtime_ctx = ();

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.read(shared, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(14, 0))
                            .with_output_identity("index-shared"),
                    ))
                })?;
                Ok(())
            })
            .unwrap();

        let main = runtime.observe().current_branch();
        let feature = runtime.create_branch("feature-index-stability").unwrap();
        runtime.switch_branch(feature.clone()).unwrap();

        let source_only = runtime.graph_mut().node().output_identity().build();
        runtime
            .graph_mut()
            .append_dependency(source_only, shared, ASPECT_A)
            .unwrap();

        runtime.switch_branch(main.clone()).unwrap();
        (runtime, feature, main)
    };

    let (mut baseline_runtime, baseline_feature, baseline_main) = prepare_runtime();
    let initial = baseline_runtime
        .inspect_branch_merge_plan_for_test(baseline_feature, baseline_main)
        .unwrap();

    let (mut rebuilt_runtime, rebuilt_feature, rebuilt_main) = prepare_runtime();
    rebuilt_runtime
        .graph_mut()
        .rebuild_subscriber_index_from_dependencies()
        .unwrap();
    let rebuilt = rebuilt_runtime
        .inspect_branch_merge_plan_for_test(rebuilt_feature, rebuilt_main)
        .unwrap();

    assert_eq!(
        initial.planned_candidates(),
        rebuilt.planned_candidates(),
        "convenience index rebuilds must not change planned merge candidates"
    );
    assert_eq!(
        initial.proof_minimal_overlap(),
        rebuilt.proof_minimal_overlap(),
        "convenience index rebuilds must not widen proof-minimal overlap"
    );
    assert_eq!(
        initial.conservative_overlap(),
        rebuilt.conservative_overlap(),
        "convenience index rebuilds must not change bounded conservative expansion"
    );
    assert_eq!(
        initial.target_overlap_journal(),
        rebuilt.target_overlap_journal(),
        "convenience index rebuilds must not perturb target overlap classification"
    );
}

#[test]
fn repeated_merge_advances_source_branch_ledger_boundary() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-repeat-merge").unwrap();
    let mut runtime_ctx = ();

    runtime.switch_branch(feature.clone()).unwrap();
    let first = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(first, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(21, 0))
                        .with_output_identity("first-merge"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let first_merge = runtime.merge_branch(feature.clone(), main.clone()).unwrap();
    assert_eq!(
        first_merge.records.len(),
        1,
        "first merge should only report the initial source-only node"
    );

    runtime.switch_branch(feature.clone()).unwrap();
    let second = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(second, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(22, 0))
                        .with_output_identity("second-merge"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main).unwrap();
    let second_merge = runtime
        .merge_branch(feature, runtime.observe().current_branch())
        .unwrap();

    assert!(
        second_merge.counters.source_slice_breadth
            == second_merge.planned_candidates.nodes.len() as u64,
        "repeated merge should continue using the source ledger candidate set"
    );
    assert!(
        second_merge
            .records
            .iter()
            .all(|record| record.source_node != first),
        "source ledger should advance past already-merged nodes"
    );
    assert!(
        second_merge
            .records
            .iter()
            .any(|record| record.source_node == second),
        "new source mutations should remain merge-visible"
    );
}

#[test]
fn retained_only_branch_churn_does_not_force_merge_replanning() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-retained-only").unwrap();
    let mut runtime_ctx = ();

    runtime.switch_branch(feature.clone()).unwrap();
    let source_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(31, 0))
                        .with_output_identity("retained-only"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let _ = runtime.merge_branch(feature.clone(), main.clone()).unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    {
        let mut graph = runtime.graph_mut();
        let mut entry = graph.get_entry_mut(source_only).unwrap();
        entry.set_retained_diagnostic_artifact(Some(RetainedDiagnosticArtifact {
            changed_regions: CanonicalChangedRegions::new([]),
            labels: vec!["retained-only-label".to_string()],
            keyed_family: None,
            keyed_key: None,
            reuse_certification: None,
            reuse_boundary_context: None,
        }));
        drop(entry);
        graph.record_branch_mutation_retained_artifact(source_only);
    }

    runtime.switch_branch(main).unwrap();
    let result = runtime
        .merge_branch(feature, runtime.observe().current_branch())
        .unwrap();

    assert!(
        result.planned_candidates.nodes.is_empty(),
        "retained-only churn should produce an explicit empty merge candidate set, not a whole-branch fallback"
    );
    assert!(
        result.records.is_empty(),
        "diagnostics-only retained churn should not create merge reconciliation work"
    );
    assert!(
        result.counters.final_candidate_breadth == 0,
        "diagnostics-only retained churn must report zero final candidate breadth"
    );
}

#[test]
fn merge_branch_equivalent_runtime_state_ignores_retained_artifact_richness() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(35, 0))
                        .with_output_identity("retained-agnostic"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    {
        let mut graph = runtime.graph_mut();
        let mut entry = graph.get_entry_mut(shared).unwrap();
        entry.set_retained_diagnostic_artifact(Some(RetainedDiagnosticArtifact {
            changed_regions: CanonicalChangedRegions::new([]),
            labels: vec!["main-label".to_string()],
            keyed_family: None,
            keyed_key: None,
            reuse_certification: None,
            reuse_boundary_context: None,
        }));
    }

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-runtime-equivalent").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(35, 0))
                        .with_output_identity("retained-agnostic"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    {
        let mut graph = runtime.graph_mut();
        let mut entry = graph.get_entry_mut(shared).unwrap();
        entry.set_retained_diagnostic_artifact(Some(RetainedDiagnosticArtifact {
            changed_regions: CanonicalChangedRegions::new([]),
            labels: vec!["feature-label".to_string()],
            keyed_family: Some("family".to_string()),
            keyed_key: Some("key".to_string()),
            reuse_certification: None,
            reuse_boundary_context: None,
        }));
        drop(entry);
        graph.record_branch_mutation_retained_artifact(shared);
    }

    runtime.switch_branch(main).unwrap();
    let result = runtime
        .merge_branch(feature, runtime.observe().current_branch())
        .unwrap();
    let shared_record = result
        .records
        .iter()
        .find(|record| record.source_node == shared)
        .expect("shared node should still be part of the merge summary");

    assert!(
        matches!(
            shared_record.action,
            ArtifactMergeAction::EquivalentUnchanged
        ),
        "merge comparability should be driven by runtime state, not retained richness"
    );
}

#[test]
fn merge_branch_self_merge_surfaces_typed_failure() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let branch = runtime.observe().current_branch();

    let err = runtime.merge_branch(branch.clone(), branch).unwrap_err();
    match err {
        SignalError::BranchMergeFailed { kind, .. } => {
            assert_eq!(kind, BranchMergeFailureKind::SelfMergeRejected);
        }
        other => panic!("expected typed branch merge failure, got {other:?}"),
    }
}

#[test]
fn merge_branch_divergent_shared_node_requires_typed_conflict_surface() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(41, 0))
                        .with_output_identity("base-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-conflict").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(42, 0))
                        .with_output_identity("feature-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    {
        let mut graph = runtime.graph_mut();
        let mut entry = graph.get_entry_mut(shared).unwrap();
        let mut runtime_artifact = entry
            .get_runtime_artifact_state()
            .cloned()
            .expect("feature shared node should have runtime artifact state");
        runtime_artifact.warm_mut().merge_authority = ArtifactMergeAuthority {
            authority_class: ArtifactAuthorityClass::BranchLocalSpeculative,
            adoptability: MergeAdoptability::NonAdoptableBranchLocal,
        };
        entry.set_runtime_artifact_state(Some(runtime_artifact));
    }

    runtime.switch_branch(main.clone()).unwrap();
    let main_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(main_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(43, 0))
                        .with_output_identity("main-only"),
                ))
            })?;
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(44, 0))
                        .with_output_identity("main-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let err = runtime.merge_branch(feature, main).unwrap_err();
    match err {
        SignalError::BranchMergeFailed { kind, evidence, .. } => {
            assert_eq!(
                kind,
                BranchMergeFailureKind::DivergenceRequiresConflictResolution
            );
            let evidence = match evidence.expect("conflict evidence should be present") {
                BranchMergeFailureEvidence::Conflict(evidence) => evidence,
                other => panic!("expected conflict evidence, got {other:?}"),
            };
            assert_eq!(
                evidence.divergence,
                BranchMergeDivergence::SharedStateConflict
            );
            assert_eq!(
                evidence.reconciliation_policy.conflict,
                ConflictMergePolicy::ResolveSourceStateWhenStructureMatches
            );
            assert_eq!(evidence.summary.total_conflict_count, 1);
            assert_eq!(evidence.summary.comparable_mismatch_count, 1);
            assert_eq!(
                evidence.summary.primary_conflict_kind,
                Some(BranchMergeConflictKind::MergeAuthorityMismatch)
            );
            assert!(evidence
                .summary
                .required_resolution
                .contains(&BranchMergeResolutionRequirement::ReconcileComparableState));
            assert!(evidence
                .summary
                .required_resolution
                .contains(&BranchMergeResolutionRequirement::ReconcileMergeAuthority));
            assert!(evidence
                .summary
                .required_resolution
                .contains(&BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState));
            assert_eq!(evidence.resolution_plan.records.len(), 1);
            assert_eq!(evidence.resolution_plan.records[0].source_node, shared);
            assert!(evidence.resolution_plan.records[0]
                .required_resolution
                .contains(&BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState));
            assert!(evidence.resolution_plan.records[0]
                .supported_strategies
                .contains(&ConflictResolutionStrategy::AdoptSourceRuntimeArtifactState));
            assert!(evidence.resolution_plan.records[0]
                .supported_strategies
                .contains(&ConflictResolutionStrategy::PreserveTargetRuntimeArtifactState));
            let failure = runtime
                .observe()
                .latest_failure_diagnostics()
                .expect("failed merge should record failure diagnostics");
            assert!(
                failure
                    .message
                    .contains("primary=Some(MergeAuthorityMismatch)"),
                "failure diagnostics should surface the primary conflict class"
            );
            assert!(
                failure.message.contains("ReconcileRuntimeArtifactState"),
                "failure diagnostics should surface required merge resolution"
            );
            assert!(
                runtime.graph().replay_events().iter().any(|event| {
                    event.kind == ReplayEventKind::FailureRecorded
                        && event
                            .detail
                            .as_ref()
                            .and_then(|detail| detail.as_message())
                            .map(|detail| detail.contains("ReconcileRuntimeArtifactState"))
                            .unwrap_or(false)
                }),
                "failed merge should emit a failure replay detail with required resolution"
            );
            assert!(
                !runtime
                    .graph()
                    .replay_events()
                    .iter()
                    .any(|event| { event.kind == ReplayEventKind::BranchMerged }),
                "failed merge must not emit a false branch-merged replay boundary"
            );
            assert_eq!(evidence.records.len(), 1);
            assert_eq!(evidence.records[0].source_node, shared);
            assert!(evidence.records[0]
                .conflict_kinds
                .contains(&BranchMergeConflictKind::ComparableMismatch));
            assert!(evidence.records[0]
                .conflict_kinds
                .contains(&BranchMergeConflictKind::RuntimeArtifactMismatch));
            assert_eq!(
                evidence.records[0]
                    .source_comparable
                    .as_ref()
                    .and_then(|comparable| comparable.output_identity.as_ref())
                    .map(|identity| identity.as_str()),
                Some("feature-shared")
            );
            assert_eq!(
                evidence.records[0]
                    .target_comparable
                    .as_ref()
                    .and_then(|comparable| comparable.output_identity.as_ref())
                    .map(|identity| identity.as_str()),
                Some("main-shared")
            );
        }
        other => panic!("expected typed divergence failure, got {other:?}"),
    }
}

#[test]
fn merge_branch_runtime_artifact_conflict_can_resolve_by_adopting_source() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(141, 0))
                        .with_output_identity("base-runtime-conflict"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-runtime-conflict-resolve")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(142, 0))
                        .with_output_identity("feature-runtime-conflict"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(143, 0))
                        .with_output_identity("main-runtime-conflict"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let result = runtime.merge_branch(feature, main).unwrap();

    assert!(
        matches!(result.merge_kind, BranchMergeKind::Applied | BranchMergeKind::ConflictResolved),
        "stable-shape snapshot reconciliation may classify as applied or resolved conflict, but it should still use the narrow snapshot delta path"
    );
    assert_eq!(
        result.divergence,
        BranchMergeDivergence::SharedStateConflict
    );
    assert_eq!(
        result.reconciliation_policy.conflict,
        ConflictMergePolicy::ResolveSourceStateWhenStructureMatches
    );
    assert!(result.resolution_plan.is_some());
    let merged_record = result
        .records
        .iter()
        .find(|record| record.source_node == shared)
        .expect("shared node should be part of the resolved merge");
    assert!(
        matches!(merged_record.action, ArtifactMergeAction::Adopted),
        "runtime-artifact conflict resolution should adopt source authority"
    );
    assert!(merged_record
        .resolved_conflict_kinds
        .contains(&BranchMergeConflictKind::RuntimeArtifactMismatch));
    assert_eq!(
        merged_record
            .target_comparable
            .as_ref()
            .and_then(|comparable| comparable.output_identity.as_ref())
            .map(|identity| identity.as_str()),
        Some("feature-runtime-conflict"),
        "resolved runtime-artifact conflict should adopt source runtime state"
    );
    assert!(
        runtime
            .graph()
            .observe()
            .lineage_records()
            .iter()
            .any(|record| matches!(
                record.kind,
                LineageRecordKind::BranchMerge {
                    merge_kind: BranchMergeKind::ConflictResolved,
                    resolution_plan: Some(_),
                    ..
                }
            )),
        "resolved conflicts should emit real conflict-resolved lineage"
    );
}

#[test]
fn merge_branch_dependency_topology_conflict_surfaces_structural_requirement() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let source_a = runtime.graph_mut().node().output_identity().build();
    let source_b = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .append_dependency(shared, source_a, ASPECT_A)
        .unwrap();

    let mut runtime_ctx = ();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                let result = match view.node() {
                    node if node == source_a => view.finish(
                        NodeEvaluationResult::from_version(version_ab(91, 0))
                            .with_output_identity("topology-source-a"),
                    ),
                    node if node == source_b => view.finish(
                        NodeEvaluationResult::from_version(version_ab(92, 0))
                            .with_output_identity("topology-source-b"),
                    ),
                    _ => view.finish(
                        NodeEvaluationResult::from_version(version_ab(93, 0))
                            .with_output_identity("topology-shared"),
                    ),
                };
                Ok(result)
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-topology-conflict").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .graph_mut()
        .set_dependencies(shared, [DependencyEdge::new(source_b, ASPECT_A)])
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .graph_mut()
        .set_dependencies(
            shared,
            [
                DependencyEdge::new(source_a, ASPECT_A),
                DependencyEdge::new(source_b, ASPECT_A),
            ],
        )
        .unwrap();

    let err = runtime.merge_branch(feature, main).unwrap_err();
    match err {
        SignalError::BranchMergeFailed { kind, evidence, .. } => {
            assert_eq!(
                kind,
                BranchMergeFailureKind::DivergenceRequiresConflictResolution
            );
            let evidence = match evidence.expect("topology conflict evidence should be present") {
                BranchMergeFailureEvidence::Conflict(evidence) => evidence,
                other => panic!("expected conflict evidence, got {other:?}"),
            };
            assert_eq!(
                evidence.divergence,
                BranchMergeDivergence::SharedStateConflict
            );
            assert_eq!(
                evidence.summary.primary_conflict_kind,
                Some(BranchMergeConflictKind::DependencyTopologyMismatch)
            );
            assert!(evidence
                .summary
                .required_resolution
                .contains(&BranchMergeResolutionRequirement::ReconcileDependencyTopology));
            assert_eq!(evidence.records.len(), 1);
            assert!(evidence.records[0]
                .conflict_kinds
                .contains(&BranchMergeConflictKind::DependencyTopologyMismatch));
        }
        other => panic!("expected topology conflict failure, got {other:?}"),
    }
}

#[test]
fn merge_branch_dependency_snapshot_conflict_can_resolve_by_adopting_source_snapshot() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let source = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .append_dependency(shared, source, ASPECT_A)
        .unwrap();

    let mut runtime_ctx = ();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                let result = if view.node() == source {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(101, 0))
                            .with_output_identity("snapshot-source"),
                    )
                } else {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(102, 0))
                            .with_output_identity("snapshot-shared"),
                    )
                };
                Ok(result)
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-snapshot-conflict").unwrap();

    let mut feature_snapshot = DependencySnapshot::empty();
    feature_snapshot.record(source, ASPECT_A, 3, None);
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .graph_mut()
        .set_dep_snapshot(shared, feature_snapshot)
        .unwrap();

    let mut main_snapshot = DependencySnapshot::empty();
    main_snapshot.record(source, ASPECT_A, 5, None);
    main_snapshot.record(source, ASPECT_B, 7, None);
    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .graph_mut()
        .set_dep_snapshot(shared, main_snapshot)
        .unwrap();

    let result = runtime.merge_branch(feature, main).unwrap();
    let merged_record = result
        .records
        .iter()
        .find(|record| record.source_node == shared)
        .expect("shared node should be part of the resolved snapshot merge");

    assert!(
        matches!(result.merge_kind, BranchMergeKind::Applied | BranchMergeKind::ConflictResolved),
        "stable-shape snapshot reconciliation may classify as applied or resolved conflict, but it should still use the narrow snapshot delta path"
    );
    assert_eq!(
        result.divergence,
        BranchMergeDivergence::SharedStateConflict
    );
    assert_eq!(
        result.reconciliation_policy.conflict,
        ConflictMergePolicy::ResolveSourceStateWhenStructureMatches
    );
    assert!(result.resolution_plan.is_some());
    assert!(matches!(
        merged_record.action,
        ArtifactMergeAction::EquivalentUnchanged
    ));
    assert!(merged_record
        .resolved_conflict_kinds
        .contains(&BranchMergeConflictKind::DependencySnapshotMismatch));
    let merged_snapshot = runtime.graph().get_dep_snapshot(shared).unwrap();
    assert_eq!(
        merged_snapshot.entries().len(),
        1,
        "resolved dependency snapshot conflict should adopt the source snapshot shape"
    );
    assert_eq!(merged_snapshot.entries()[0].cached_version, 3);
    assert_eq!(merged_snapshot.entries()[0].aspect, ASPECT_A);
}

#[test]
fn merge_branch_conflict_resolved_emits_resolution_traceability() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(150, 0))
                        .with_output_identity("main-conflict-trace"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-trace").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(151, 0))
                        .with_output_identity("feature-conflict-trace"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(152, 0))
                        .with_output_identity("main-conflict-trace-advanced"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let result = runtime.merge_branch(feature, main).unwrap();
    assert_eq!(result.merge_kind, BranchMergeKind::ConflictResolved);
    let resolution_plan = result
        .resolution_plan
        .as_ref()
        .expect("conflict-resolved merge should retain resolution plan");
    assert_eq!(resolution_plan.records.len(), 1);
    assert!(resolution_plan.records[0]
        .required_resolution
        .contains(&BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState));

    let replay_detail = runtime
        .graph()
        .observe()
        .replay_events()
        .iter()
        .rev()
        .find(|event| matches!(event.kind, ReplayEventKind::BranchMerged))
        .and_then(|event| event.detail.as_ref().and_then(|detail| detail.as_message()))
        .expect("conflict-resolved merge should emit branch merge replay detail");
    assert!(
        replay_detail.contains("resolved_requirements"),
        "conflict-resolved replay should expose resolved requirements"
    );

    let branch_merge_lineage = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .rev()
        .find(|record| {
            matches!(
                record.kind,
                LineageRecordKind::BranchMerge {
                    merge_kind: BranchMergeKind::ConflictResolved,
                    ..
                }
            )
        })
        .expect("conflict-resolved merge should emit branch merge lineage");
    match &branch_merge_lineage.kind {
        LineageRecordKind::BranchMerge {
            resolution_plan: Some(plan),
            ..
        } => {
            assert_eq!(plan.records.len(), 1);
            assert!(plan.records[0]
                .required_resolution
                .contains(&BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState));
        }
        other => panic!("expected conflict-resolved branch merge lineage, got {other:?}"),
    }

    let artifact_merge_lineage = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .rev()
        .find(|record| {
            matches!(
                record.kind,
                LineageRecordKind::ArtifactMerge {
                    source_node,
                    merge_kind: BranchMergeKind::ConflictResolved,
                    ..
                } if source_node == shared
            )
        })
        .expect("conflict-resolved merge should emit artifact merge lineage");
    match &artifact_merge_lineage.kind {
        LineageRecordKind::ArtifactMerge {
            resolved_conflict_kinds,
            ..
        } => {
            assert!(
                resolved_conflict_kinds.contains(&BranchMergeConflictKind::RuntimeArtifactMismatch)
            );
        }
        other => panic!("expected conflict-resolved artifact merge lineage, got {other:?}"),
    }
}

#[test]
fn runtime_merge_request_named_conflict_policy_changes_merge_outcome() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(201, 0))
                        .with_output_identity("base-request-policy"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let reject_feature = runtime
        .create_branch("feature-request-conflict-policy-reject")
        .unwrap();

    runtime.switch_branch(reject_feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(202, 0))
                        .with_output_identity("feature-request-policy"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(203, 0))
                        .with_output_identity("main-request-policy"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let reject_err = runtime
        .merge()
        .from(reject_feature.clone())
        .into(main.clone())
        .conflict_policy_named("signal.conflict.reject-shared-state")
        .run()
        .expect_err("reject policy should fail closed on shared-state conflict");

    match reject_err {
        SignalError::BranchMergeFailed { kind, evidence, .. } => {
            assert_eq!(
                kind,
                BranchMergeFailureKind::DivergenceRequiresConflictResolution
            );
            let evidence =
                match evidence.expect("reject policy failure should expose conflict evidence") {
                    BranchMergeFailureEvidence::Conflict(evidence) => evidence,
                    other => panic!("expected conflict evidence, got {other:?}"),
                };
            assert_eq!(
                evidence.reconciliation_policy.conflict,
                ConflictMergePolicy::RejectSharedStateConflict
            );
            assert_eq!(
                evidence.summary.primary_conflict_kind,
                Some(BranchMergeConflictKind::RuntimeArtifactMismatch)
            );
        }
        other => panic!("expected typed merge failure, got {other:?}"),
    }

    let resolve_feature = runtime
        .create_branch("feature-request-conflict-policy-resolve")
        .unwrap();
    runtime.switch_branch(resolve_feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(204, 0))
                        .with_output_identity("feature-request-policy-resolve"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(205, 0))
                        .with_output_identity("main-request-policy-resolve"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let resolve_result = runtime
        .merge()
        .from(resolve_feature)
        .into(main)
        .conflict_policy_named("signal.conflict.resolve-source-when-structure-matches")
        .run()
        .expect("resolve policy should permit bounded auto-resolution");

    assert_eq!(
        resolve_result.selected_conflict_policy_name.as_str(),
        "signal.conflict.resolve-source-when-structure-matches"
    );
    assert_eq!(
        resolve_result.selected_conflict_policy_basis,
        ConflictPolicySelectionBasis::RequestNamed
    );
    assert_eq!(
        resolve_result.reconciliation_policy.conflict,
        ConflictMergePolicy::ResolveSourceStateWhenStructureMatches
    );
    assert_eq!(resolve_result.merge_kind, BranchMergeKind::ConflictResolved);
}

#[test]
fn merge_branch_target_advanced_without_shared_conflict_surfaces_applied_divergence() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(51, 0))
                        .with_output_identity("base-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-applied").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(51, 0))
                        .with_output_identity("base-shared"),
                ))
            })?;
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(52, 0))
                        .with_output_identity("feature-only"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let main_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(51, 0))
                        .with_output_identity("base-shared"),
                ))
            })?;
            tx.read(main_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(53, 0))
                        .with_output_identity("main-only"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let result = runtime.merge_branch(feature, main).unwrap();
    assert_eq!(result.merge_kind, BranchMergeKind::Applied);
    assert_eq!(result.divergence, BranchMergeDivergence::TargetAdvanced);
    assert!(result.counters.final_candidate_breadth > 0);
    assert!(result
        .records
        .iter()
        .any(|record| record.action == ArtifactMergeAction::IntroducedIntoTarget));
}

#[test]
fn merge_branch_unrelated_target_only_pending_work_does_not_degrade_fast_forward() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(61, 0))
                        .with_output_identity("base-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-fast-forward").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(62, 0))
                        .with_output_identity("feature-only"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let unrelated_main_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(unrelated_main_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(63, 0))
                        .with_output_identity("main-unrelated"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let result = runtime.merge_branch(feature, main).unwrap();
    assert_eq!(result.merge_kind, BranchMergeKind::FastForward);
    assert_eq!(result.divergence, BranchMergeDivergence::None);
    assert!(result
        .records
        .iter()
        .any(|record| record.action == ArtifactMergeAction::IntroducedIntoTarget));
}

#[test]
fn restore_branch_snapshot_after_merge_preserves_introduced_nodes_and_remapped_dependencies() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(90, 0))
                        .with_output_identity("restore-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-restore-merge").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    let upstream = runtime.graph_mut().node().output_identity().build();
    let downstream = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .append_dependency(upstream, shared, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .append_dependency(downstream, upstream, ASPECT_A)
        .unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(downstream, &|view| {
                let result = if view.node() == upstream {
                    let version = view.read_aspect_version(shared, ASPECT_A)?;
                    view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("restore-upstream"),
                    )
                } else {
                    let version = view.read_aspect_version(upstream, ASPECT_A)?;
                    view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("restore-downstream"),
                    )
                };
                Ok(result)
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let merge = runtime.merge_branch(feature, main.clone()).unwrap();
    let introduced_upstream = merge
        .records
        .iter()
        .find(|record| record.source_node == upstream)
        .and_then(|record| record.target_node)
        .expect("merged upstream node should be introduced into target");
    let introduced_downstream = merge
        .records
        .iter()
        .find(|record| record.source_node == downstream)
        .and_then(|record| record.target_node)
        .expect("merged downstream node should be introduced into target");

    let merged_snapshot = runtime.capture_branch_snapshot(main.clone()).unwrap();

    let unrelated = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(unrelated, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(91, 0))
                        .with_output_identity("post-merge-unrelated"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .restore_branch_snapshot(main.clone(), &merged_snapshot)
        .unwrap();

    assert!(
        runtime.graph().is_alive(introduced_upstream),
        "restoring the merged branch snapshot should preserve introduced upstream nodes"
    );
    assert!(
        runtime.graph().is_alive(introduced_downstream),
        "restoring the merged branch snapshot should preserve introduced downstream nodes"
    );
    assert!(!runtime.graph().is_alive(unrelated));
    assert_eq!(
        runtime
            .graph()
            .dependencies_of(introduced_downstream)
            .unwrap()
            .iter()
            .map(|edge| edge.source())
            .collect::<Vec<_>>(),
        vec![introduced_upstream],
        "restored merged topology must retain remapped target dependencies"
    );
}

#[test]
fn restore_after_merge_does_not_emit_false_branch_merge_history() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(92, 0))
                        .with_output_identity("history-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-history-restore").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(93, 0))
                        .with_output_identity("history-feature-only"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime.merge_branch(feature, main.clone()).unwrap();
    let merged_snapshot = runtime.capture_branch_snapshot(main.clone()).unwrap();

    let branch_merge_replay_before = runtime
        .graph()
        .replay_events()
        .iter()
        .filter(|event| event.kind == ReplayEventKind::BranchMerged)
        .count();
    let branch_merge_lineage_before = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .filter(|record| matches!(record.kind, LineageRecordKind::BranchMerge { .. }))
        .count();

    runtime
        .restore_branch_snapshot(main, &merged_snapshot)
        .unwrap();

    let branch_merge_replay_after = runtime
        .graph()
        .replay_events()
        .iter()
        .filter(|event| event.kind == ReplayEventKind::BranchMerged)
        .count();
    let branch_merge_lineage_after = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .filter(|record| matches!(record.kind, LineageRecordKind::BranchMerge { .. }))
        .count();

    assert_eq!(
        branch_merge_replay_after, branch_merge_replay_before,
        "snapshot restore after merge must not fabricate extra BranchMerged replay events"
    );
    assert_eq!(
        branch_merge_lineage_after, branch_merge_lineage_before,
        "snapshot restore after merge must not fabricate extra BranchMerge lineage records"
    );
    assert!(
        runtime.graph().replay_events().iter().any(|event| {
            event.kind == ReplayEventKind::SnapshotRestored
                && event.snapshot_id == Some(merged_snapshot.meta.snapshot_id)
        }),
        "restore should still emit its own snapshot restore replay boundary"
    );
}

#[test]
fn active_restore_reinstates_branch_merge_ledger_boundary_for_later_fast_forward_merge() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(94, 0))
                        .with_output_identity("restore-base-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let base_snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-active-restore-fast-forward")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(94, 0))
                        .with_output_identity("restore-base-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(95, 0))
                        .with_output_identity("restore-main-advanced"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.restore_snapshot(&base_snapshot).unwrap();

    let result = runtime.merge_branch(feature, main).unwrap();
    assert_eq!(
        result.merge_kind,
        BranchMergeKind::FastForward,
        "restoring the active branch snapshot must reinstate the captured merge boundary and avoid stale target divergence"
    );
    assert_eq!(
        result.divergence,
        BranchMergeDivergence::None,
        "active restore should clear stale target overlap from the restored branch ledger"
    );
}

#[test]
fn runtime_merge_strategy_hint_selects_registered_descriptor() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-strategy-hint").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .strategy_hint(BranchMergeStrategy::ReplaySourceDeltaOntoTarget)
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().merge_strategy(),
        BranchMergeStrategy::ReplaySourceDeltaOntoTarget
    );
    assert_eq!(
        planned.plan().selected_strategy_name().as_str(),
        "signal.merge.replay-source-delta"
    );
}

#[test]
fn runtime_merge_strategy_named_selects_registered_descriptor() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-strategy-name").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .strategy_named("signal.merge.rebase-source-onto-target")
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().merge_strategy(),
        BranchMergeStrategy::RebaseSourceOntoTarget
    );
    assert_eq!(
        planned.plan().selected_strategy_name().as_str(),
        "signal.merge.rebase-source-onto-target"
    );
}

#[test]
fn runtime_merge_base_named_selects_registered_descriptor_and_lowers_plan() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-merge-base-name").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .merge_base_named("signal.merge-base.fork-point")
        .plan()
        .unwrap();

    let lowered = planned
        .plan()
        .lowered_merge_base()
        .expect("lowered merge-base plan");
    assert_eq!(
        lowered.selected_merge_base_name.as_str(),
        "signal.merge-base.fork-point"
    );
    assert_eq!(
        lowered.resolved_base.forked_from_snapshot_id,
        planned
            .plan()
            .merge_base()
            .and_then(|base| base.forked_from_snapshot_id)
    );
    let proof = merge_plan_proof_report(planned.plan(), planned.plan().registry_bundle_digest());
    assert_eq!(
        proof.selected_merge_base_digest,
        lowered.selected_merge_base_digest
    );
    assert_eq!(
        planned.plan().selected_semantics().merge_base_name.as_str(),
        "signal.merge-base.fork-point"
    );
}

#[test]
fn runtime_proof_report_carries_merge_base_registry_digest() {
    let runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let report = runtime_proof_report(
        runtime.schema_registry().registry_digest(),
        runtime.merge_strategy_registry().registry_digest(),
        runtime.merge_base_strategy_registry().registry_digest(),
        runtime.aspect_merge_policy_registry().registry_digest(),
        runtime.conflict_isolation_registry().registry_digest(),
        runtime.conflict_policy_registry().registry_digest(),
        runtime.identity_matcher_registry().registry_digest(),
        runtime.source_only_policy_registry().registry_digest(),
        runtime.deletion_policy_registry().registry_digest(),
    );

    assert_eq!(
        report.merge_base_strategy_registry_digest,
        runtime.merge_base_strategy_registry().registry_digest()
    );
}

#[test]
fn runtime_merge_uses_schema_default_strategy_when_request_is_silent() {
    let graph = SignalGraph::new().with_schema_registry(merge_schema_registry(
        "signal.merge.rebase-source-onto-target",
        None,
        None,
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-schema-default-strategy")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.merge-owned")
        .expect("known schema")
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().merge_strategy(),
        BranchMergeStrategy::RebaseSourceOntoTarget
    );
    assert_eq!(
        planned.plan().selected_strategy_basis(),
        MergeStrategySelectionBasis::SchemaDefault
    );
}

#[test]
fn runtime_merge_lowers_schema_default_aspect_policy_for_affected_aspect() {
    let graph = SignalGraph::new().with_schema_registry(aspect_policy_merge_schema_registry(Some(
        "signal.aspect.prefer-source",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-aspect-schema-default")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.aspect-merge-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A])
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    let aspect_plan = planned.plan().aspect_policy_plan();
    assert_eq!(aspect_plan.records.len(), 1);
    assert_eq!(aspect_plan.records[0].aspect, ASPECT_A);
    assert_eq!(
        aspect_plan.records[0].selected_policy_name.as_str(),
        "signal.aspect.prefer-source"
    );
    assert_eq!(
        aspect_plan.records[0].selected_policy_basis,
        AspectMergePolicySelectionBasis::SchemaDefault
    );
}

#[test]
fn runtime_merge_node_aspect_policy_override_precedes_schema_default() {
    let graph = SignalGraph::new().with_schema_registry(aspect_policy_merge_schema_registry(Some(
        "signal.aspect.prefer-target",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-aspect-node-override")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.aspect-merge-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A])
        .aspect_merge_policy_name(ASPECT_A, "signal.aspect.prefer-source")
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    let aspect_plan = planned.plan().aspect_policy_plan();
    assert_eq!(aspect_plan.records.len(), 1);
    assert_eq!(
        aspect_plan.records[0].selected_policy_name.as_str(),
        "signal.aspect.prefer-source"
    );
    assert_eq!(
        aspect_plan.records[0].selected_policy_basis,
        AspectMergePolicySelectionBasis::NodeOverride
    );
}

#[test]
fn runtime_merge_request_named_aspect_policy_precedes_schema_and_node_defaults() {
    let graph = SignalGraph::new().with_schema_registry(aspect_policy_merge_schema_registry(Some(
        "signal.aspect.prefer-target",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(4, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-aspect-request-override")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.aspect-merge-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A])
        .aspect_merge_policy_name(ASPECT_A, "signal.aspect.prefer-target")
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .aspect_policy_named(ASPECT_A, "signal.aspect.prefer-source")
        .plan()
        .unwrap();

    let aspect_plan = planned.plan().aspect_policy_plan();
    assert_eq!(aspect_plan.records.len(), 1);
    assert_eq!(
        aspect_plan.records[0].selected_policy_name.as_str(),
        "signal.aspect.prefer-source"
    );
    assert_eq!(
        aspect_plan.records[0].selected_policy_basis,
        AspectMergePolicySelectionBasis::RequestNamed
    );
}

#[test]
fn runtime_merge_lowers_aspect_decision_records_for_affected_nodes() {
    let graph = SignalGraph::new().with_schema_registry(aspect_policy_merge_schema_registry(Some(
        "signal.aspect.prefer-source",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-aspect-decision-lowering")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.aspect-merge-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A])
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    let aspect_decisions = &planned.plan().aspect_decision_plan().records;
    assert_eq!(aspect_decisions.len(), 1);
    assert_eq!(aspect_decisions[0].aspect, ASPECT_A);
    assert_eq!(aspect_decisions[0].source_node, feature_only);
    assert_eq!(
        aspect_decisions[0].selected_policy_name.as_str(),
        "signal.aspect.prefer-source"
    );
    assert_eq!(
        aspect_decisions[0].outcome,
        AspectMergeDecisionOutcome::SourceIntroducedIntoTarget
    );
}

#[test]
fn runtime_merge_lowers_conflict_isolation_records_for_conflicted_nodes() {
    let graph =
        SignalGraph::new().with_schema_registry(conflict_isolation_merge_schema_registry(None));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.conflict-isolation-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A])
        .build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-conflict-isolation-schema-default")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(2, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(3, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_conflict_isolation_name().as_str(),
        "signal.conflict-isolation.per-node"
    );
    assert_eq!(
        planned.plan().selected_conflict_isolation_basis(),
        ConflictIsolationSelectionBasis::BuiltInDefault
    );
    assert_eq!(
        planned.plan().conflict_isolation_plan().expansion_breadth,
        0
    );
}

#[test]
fn runtime_merge_request_named_conflict_isolation_precedes_schema_and_node_defaults() {
    let graph = SignalGraph::new().with_schema_registry(conflict_isolation_merge_schema_registry(
        Some("signal.conflict-isolation.per-node"),
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.conflict-isolation-owned")
        .expect("known schema")
        .conflict_isolation_policy_name("signal.conflict-isolation.per-node")
        .produces_aspects([ASPECT_A, ASPECT_B])
        .build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(401, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-request-conflict-isolation-precedence")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(402, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(403, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .conflict_isolation_policy_named("signal.conflict-isolation.per-aspect")
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_conflict_isolation_name().as_str(),
        "signal.conflict-isolation.per-aspect"
    );
    assert_eq!(
        planned.plan().selected_conflict_isolation_basis(),
        ConflictIsolationSelectionBasis::RequestNamed
    );
    assert_eq!(planned.plan().conflict_isolation_plan().records.len(), 1);
    assert_eq!(
        planned.plan().conflict_isolation_plan().records[0].granularity,
        ConflictIsolationGranularity::PerAspect
    );
    assert_eq!(
        planned.plan().conflict_isolation_plan().records[0].isolated_aspects,
        vec![ASPECT_A, ASPECT_B]
    );
}

#[test]
fn runtime_merge_node_conflict_isolation_override_precedes_schema_default() {
    let graph = SignalGraph::new().with_schema_registry(conflict_isolation_merge_schema_registry(
        Some("signal.conflict-isolation.per-node"),
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.conflict-isolation-owned")
        .expect("known schema")
        .conflict_isolation_policy_name("signal.conflict-isolation.per-aspect")
        .produces_aspects([ASPECT_A, ASPECT_B])
        .build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(411, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-node-conflict-isolation-precedence")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(412, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(413, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_conflict_isolation_name().as_str(),
        "signal.conflict-isolation.per-aspect"
    );
    assert_eq!(
        planned.plan().selected_conflict_isolation_basis(),
        ConflictIsolationSelectionBasis::NodeOverride
    );
    assert_eq!(
        planned.plan().conflict_isolation_plan().records[0].granularity,
        ConflictIsolationGranularity::PerAspect
    );
}

#[test]
fn runtime_merge_schema_default_conflict_isolation_applies_to_structural_conflicts() {
    let graph = SignalGraph::new().with_schema_registry(conflict_isolation_merge_schema_registry(
        Some("signal.conflict-isolation.per-aspect"),
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.conflict-isolation-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A, ASPECT_B])
        .build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(421, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-schema-conflict-isolation-precedence")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(422, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(423, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_conflict_isolation_name().as_str(),
        "signal.conflict-isolation.per-aspect"
    );
    assert_eq!(
        planned.plan().selected_conflict_isolation_basis(),
        ConflictIsolationSelectionBasis::SchemaDefault
    );
    assert_eq!(planned.plan().conflict_isolation_plan().records.len(), 1);
    assert_eq!(
        planned.plan().conflict_isolation_plan().records[0].isolated_aspects,
        vec![ASPECT_A, ASPECT_B]
    );
}

#[test]
fn runtime_merge_conflict_isolation_selection_flows_into_execution_counters() {
    let graph = SignalGraph::new().with_schema_registry(conflict_isolation_merge_schema_registry(
        Some("signal.conflict-isolation.per-node"),
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.conflict-isolation-owned")
        .expect("known schema")
        .produces_aspects([ASPECT_A, ASPECT_B])
        .build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(431, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-conflict-isolation-execution-counters")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(432, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(433, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .conflict_isolation_policy_named("signal.conflict-isolation.per-aspect")
        .run()
        .unwrap();

    assert_eq!(
        result.selected_conflict_isolation_name.as_str(),
        "signal.conflict-isolation.per-aspect"
    );
    assert_eq!(
        result.selected_conflict_isolation_basis,
        ConflictIsolationSelectionBasis::RequestNamed
    );
    assert_eq!(result.conflict_isolation_plan.records.len(), 1);
    assert_eq!(
        result.conflict_isolation_plan.records[0].granularity,
        ConflictIsolationGranularity::PerAspect
    );
    assert_eq!(
        result.conflict_isolation_plan.records[0].isolated_aspects,
        vec![ASPECT_A, ASPECT_B]
    );
    assert_eq!(result.counters.conflict_isolation_record_count, 1);
    assert_eq!(result.counters.conflict_isolation_expansion_breadth, 0);
}

#[test]
fn runtime_merge_node_override_precedes_schema_default_strategy() {
    let graph = SignalGraph::new().with_schema_registry(merge_schema_registry(
        "signal.merge.rebase-source-onto-target",
        None,
        None,
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-node-override-strategy")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.merge-owned")
        .expect("known schema")
        .merge_strategy_name("signal.merge.replay-source-delta")
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().merge_strategy(),
        BranchMergeStrategy::ReplaySourceDeltaOntoTarget
    );
    assert_eq!(
        planned.plan().selected_strategy_basis(),
        MergeStrategySelectionBasis::NodeOverride
    );
}

#[test]
fn runtime_merge_request_named_conflict_policy_selects_registered_descriptor() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-request-named-conflict-policy")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .conflict_policy_named("signal.conflict.reject-shared-state")
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_conflict_policy_name().as_str(),
        "signal.conflict.reject-shared-state"
    );
    assert_eq!(
        planned.plan().selected_conflict_policy_basis(),
        ConflictPolicySelectionBasis::RequestNamed
    );
    assert_eq!(
        planned.plan().reconciliation_policy().conflict,
        ConflictMergePolicy::RejectSharedStateConflict
    );
}

#[test]
fn runtime_merge_uses_schema_default_conflict_policy_when_request_is_silent() {
    let graph = SignalGraph::new().with_schema_registry(merge_schema_registry(
        "signal.merge.rebase-source-onto-target",
        Some("signal.conflict.reject-shared-state"),
        None,
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-schema-default-conflict-policy")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.merge-owned")
        .expect("known schema")
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_conflict_policy_name().as_str(),
        "signal.conflict.reject-shared-state"
    );
    assert_eq!(
        planned.plan().selected_conflict_policy_basis(),
        ConflictPolicySelectionBasis::SchemaDefault
    );
    assert_eq!(
        planned.plan().reconciliation_policy().conflict,
        ConflictMergePolicy::RejectSharedStateConflict
    );
}

#[test]
fn runtime_merge_node_conflict_policy_override_precedes_schema_default() {
    let graph = SignalGraph::new().with_schema_registry(merge_schema_registry(
        "signal.merge.rebase-source-onto-target",
        Some("signal.conflict.resolve-source-when-structure-matches"),
        None,
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-node-override-conflict-policy")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.merge-owned")
        .expect("known schema")
        .conflict_policy_name("signal.conflict.reject-shared-state")
        .build();
    runtime
        .graph_mut()
        .append_dependency(feature_only, shared, ASPECT_A)
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                let upstream = view.read_aspect_version(shared, ASPECT_A)?;
                Ok(view.finish(NodeEvaluationResult::from_version(upstream)))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_conflict_policy_name().as_str(),
        "signal.conflict.reject-shared-state"
    );
    assert_eq!(
        planned.plan().selected_conflict_policy_basis(),
        ConflictPolicySelectionBasis::NodeOverride
    );
    assert_eq!(
        planned.plan().reconciliation_policy().conflict,
        ConflictMergePolicy::RejectSharedStateConflict
    );
}

#[test]
fn runtime_merge_request_named_identity_matcher_selects_registered_descriptor() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-request-named-identity-matcher")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("identity-matcher-request"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .identity_matcher_named("signal.identity.output-identity-in-target-journal")
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_identity_matcher_name().as_str(),
        "signal.identity.output-identity-in-target-journal"
    );
    assert_eq!(
        planned.plan().selected_identity_matcher_basis(),
        IdentityMatcherSelectionBasis::RequestNamed
    );
    assert_eq!(
        planned
            .plan()
            .selected_semantics()
            .identity_matcher_name
            .as_str(),
        "signal.identity.output-identity-in-target-journal"
    );
}

#[test]
fn runtime_merge_uses_schema_default_identity_matcher_when_request_is_silent() {
    let graph = SignalGraph::new().with_schema_registry(merge_schema_registry(
        "signal.merge.rebase-source-onto-target",
        None,
        Some("signal.identity.output-identity-in-target-journal"),
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-schema-default-identity-matcher")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("identity-matcher-schema"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_identity_matcher_name().as_str(),
        "signal.identity.output-identity-in-target-journal"
    );
    assert_eq!(
        planned.plan().selected_identity_matcher_basis(),
        IdentityMatcherSelectionBasis::SchemaDefault
    );
}

#[test]
fn runtime_merge_node_identity_matcher_override_precedes_schema_default() {
    let graph = SignalGraph::new().with_schema_registry(merge_schema_registry(
        "signal.merge.rebase-source-onto-target",
        None,
        Some("signal.identity.exact-node-id"),
    ));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-node-override-identity-matcher")
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.merge-owned")
        .expect("known schema")
        .identity_matcher_name("signal.identity.output-identity-in-target-journal")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("identity-matcher-node"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_identity_matcher_name().as_str(),
        "signal.identity.output-identity-in-target-journal"
    );
    assert_eq!(
        planned.plan().selected_identity_matcher_basis(),
        IdentityMatcherSelectionBasis::NodeOverride
    );
}

#[test]
fn runtime_merge_identity_matcher_changes_source_only_correspondence_behavior() {
    let graph = SignalGraph::new().with_schema_registry(cross_identity_merge_schema_registry(None));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-identity-matcher-correspondence")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(10, 0))
                        .with_output_identity("gear-tooth-correspondence"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let target_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(target_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(11, 0))
                        .with_output_identity("gear-tooth-correspondence"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let target_node_count_before_exact = runtime.graph().active_node_count();

    let exact_result = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .identity_matcher_named("signal.identity.exact-node-id")
        .run()
        .unwrap();

    assert_eq!(
        exact_result.selected_identity_matcher_name.as_str(),
        "signal.identity.exact-node-id"
    );
    assert_eq!(
        exact_result.selected_identity_matcher_basis,
        IdentityMatcherSelectionBasis::RequestNamed
    );
    assert_eq!(
        runtime.graph().active_node_count(),
        target_node_count_before_exact + 1
    );
    assert!(exact_result
        .records
        .iter()
        .any(|record| record.source_node == feature_only
            && matches!(record.action, ArtifactMergeAction::IntroducedIntoTarget)));

    let graph = SignalGraph::new().with_schema_registry(cross_identity_merge_schema_registry(None));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-identity-matcher-correspondence")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(10, 0))
                        .with_output_identity("gear-tooth-correspondence"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let target_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(target_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(11, 0))
                        .with_output_identity("gear-tooth-correspondence"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let target_node_count_before_output_identity = runtime.graph().active_node_count();

    let matched_result = runtime
        .merge()
        .from(feature)
        .into(main)
        .identity_matcher_named("signal.identity.output-identity-in-target-journal")
        .run()
        .unwrap();

    assert_eq!(
        matched_result.selected_identity_matcher_name.as_str(),
        "signal.identity.output-identity-in-target-journal"
    );
    assert_eq!(
        matched_result.selected_identity_matcher_basis,
        IdentityMatcherSelectionBasis::RequestNamed
    );
    assert_eq!(
        matched_result
            .identity_correspondence
            .rejected_admissibility_count,
        0
    );
    assert_eq!(
        runtime.graph().active_node_count(),
        target_node_count_before_output_identity
    );
    let matched_record = matched_result
        .records
        .iter()
        .find(|record| record.source_node == feature_only)
        .expect("identity matcher should reconcile the source-only node onto the target node");
    assert_eq!(matched_record.target_node, Some(target_only));
    assert_eq!(
        matched_record.identity_basis,
        Some(IdentityCorrespondenceBasis::OutputIdentityTargetJournal)
    );
    assert_eq!(
        matched_record.identity_status,
        Some(IdentityCorrespondenceStatus::Matched)
    );
    assert_eq!(matched_record.identity_candidate_count, 1);
    assert!(!matches!(
        matched_record.action,
        ArtifactMergeAction::IntroducedIntoTarget
    ));
}

#[test]
fn runtime_merge_output_identity_matcher_fails_closed_without_explicit_admissibility() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-identity-matcher-not-admitted")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(20, 0))
                        .with_output_identity("not-admitted-correspondence"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let target_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(target_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(21, 0))
                        .with_output_identity("not-admitted-correspondence"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let target_node_count_before = runtime.graph().active_node_count();

    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .identity_matcher_named("signal.identity.output-identity-in-target-journal")
        .run()
        .unwrap();

    assert_eq!(
        runtime.graph().active_node_count(),
        target_node_count_before + 1
    );
    assert_eq!(
        result.identity_correspondence.rejected_admissibility_count,
        1
    );
    let correspondence = result
        .identity_correspondence
        .records
        .iter()
        .find(|record| record.source_node == feature_only)
        .expect("source node should have identity correspondence record");
    assert_eq!(
        correspondence.status,
        IdentityCorrespondenceStatus::UnmatchedRejectedAdmissibility
    );
    assert!(correspondence.admissibility_rejection.is_some());
    assert!(result
        .records
        .iter()
        .any(|record| record.source_node == feature_only
            && matches!(record.action, ArtifactMergeAction::IntroducedIntoTarget)));
}

#[test]
fn runtime_merge_request_named_source_only_policy_selects_registered_descriptor() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-request-named-source-only-policy")
        .unwrap();
    let mut runtime_ctx = ();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(30, 0))
                        .with_output_identity("request-named-source-only-policy"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = match runtime
        .merge()
        .from(feature)
        .into(main)
        .source_only_policy_named("signal.source-only.reject-introduction")
        .plan()
    {
        Ok(_) => panic!(
            "reject-introduction policy must fail closed when source-only adoption is required"
        ),
        Err(err) => err,
    };

    assert!(planned
        .to_string()
        .contains("signal.source-only.reject-introduction"));
}

#[test]
fn runtime_merge_uses_schema_default_source_only_policy_when_request_is_silent() {
    let graph = SignalGraph::new().with_schema_registry(source_only_merge_schema_registry(Some(
        "signal.source-only.reject-introduction",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-schema-default-source-only-policy")
        .unwrap();
    let mut runtime_ctx = ();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.source-only-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(31, 0))
                        .with_output_identity("schema-default-source-only-policy"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let err = match runtime.merge().from(feature).into(main).plan() {
        Ok(_) => panic!("schema default reject-introduction should fail closed"),
        Err(err) => err,
    };

    assert!(err
        .to_string()
        .contains("signal.source-only.reject-introduction"));
}

#[test]
fn runtime_merge_node_source_only_policy_override_precedes_schema_default() {
    let graph = SignalGraph::new().with_schema_registry(source_only_merge_schema_registry(Some(
        "signal.source-only.reject-introduction",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-node-source-only-policy-override")
        .unwrap();
    let mut runtime_ctx = ();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.source-only-merge-owned")
        .expect("known schema")
        .source_only_policy_name("signal.source-only.introduce-adoptable-skip-non-adoptable")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(32, 0))
                        .with_output_identity("node-source-only-policy-override"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_source_only_policy_name().as_str(),
        "signal.source-only.introduce-adoptable-skip-non-adoptable"
    );
    assert_eq!(
        planned.plan().selected_source_only_policy_basis(),
        SourceOnlyPolicySelectionBasis::NodeOverride
    );

    let result = planned.execute().unwrap();
    assert_eq!(
        result.selected_source_only_policy_name.as_str(),
        "signal.source-only.introduce-adoptable-skip-non-adoptable"
    );
    assert_eq!(
        result.selected_source_only_policy_basis,
        SourceOnlyPolicySelectionBasis::NodeOverride
    );
    assert!(result.records.iter().any(|record| {
        record.source_node == feature_only
            && matches!(record.action, ArtifactMergeAction::IntroducedIntoTarget)
    }));
}

#[test]
fn runtime_merge_reject_source_only_policy_blocks_introduction_and_preserves_target_breadth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-reject-source-only-policy")
        .unwrap();
    let mut runtime_ctx = ();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(33, 0))
                        .with_output_identity("reject-source-only-policy"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let target_node_count_before = runtime.graph().active_node_count();
    let err = runtime
        .merge()
        .from(feature)
        .into(main)
        .source_only_policy_named("signal.source-only.reject-introduction")
        .run()
        .expect_err("reject-introduction policy must block source-only adoption");

    assert!(err
        .to_string()
        .contains("rejects introducing source-only node"));
    assert_eq!(
        runtime.graph().active_node_count(),
        target_node_count_before
    );
}

#[test]
fn runtime_merge_uses_schema_default_deletion_policy_when_request_is_silent() {
    let graph = SignalGraph::new().with_schema_registry(deletion_merge_schema_registry(Some(
        "signal.deletion.reject-target-only-conflict",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(34, 0))
                        .with_output_identity("deletion-shared-base"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-schema-default-deletion-policy")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.deletion-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(34, 0))
                        .with_output_identity("deletion-shared-base"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let main_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(main_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(35, 0))
                        .with_output_identity("schema-default-deletion-policy"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let err = runtime
        .merge()
        .from(feature)
        .into(main)
        .run()
        .expect_err("schema default reject-target-only-conflict should fail closed");

    assert!(err
        .to_string()
        .contains("signal.deletion.reject-target-only-conflict"));
}

#[test]
fn runtime_merge_node_deletion_policy_override_precedes_schema_default() {
    let graph = SignalGraph::new().with_schema_registry(deletion_merge_schema_registry(Some(
        "signal.deletion.reject-target-only-conflict",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(36, 0))
                        .with_output_identity("deletion-override-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-node-deletion-policy-override")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.deletion-merge-owned")
        .expect("known schema")
        .deletion_policy_name("signal.deletion.preserve-target-only")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(36, 0))
                        .with_output_identity("deletion-override-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let main_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(main_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(37, 0))
                        .with_output_identity("node-deletion-policy-override"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let planned = runtime
        .merge()
        .from(feature.clone())
        .into(main.clone())
        .plan()
        .unwrap();

    assert_eq!(
        planned.plan().selected_deletion_policy_name().as_str(),
        "signal.deletion.preserve-target-only"
    );
    assert_eq!(
        planned.plan().selected_deletion_policy_basis(),
        DeletionPolicySelectionBasis::NodeOverride
    );

    let result = planned.execute().unwrap();
    assert_eq!(
        result.selected_deletion_policy_name.as_str(),
        "signal.deletion.preserve-target-only"
    );
    assert_eq!(
        result.selected_deletion_policy_basis,
        DeletionPolicySelectionBasis::NodeOverride
    );
    assert_eq!(result.deletion_plan.target_only_count, 1);
}

#[test]
fn runtime_merge_request_named_deletion_policy_precedes_schema_and_node_defaults() {
    let graph = SignalGraph::new().with_schema_registry(deletion_merge_schema_registry(Some(
        "signal.deletion.reject-target-only-conflict",
    )));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let shared = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(136, 0))
                        .with_output_identity("deletion-request-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-request-deletion-policy")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.deletion-merge-owned")
        .expect("known schema")
        .deletion_policy_name("signal.deletion.reject-target-only-conflict")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(shared, ASPECT_A)?;
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(136, 0))
                        .with_output_identity("deletion-request-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let main_only = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(main_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(137, 0))
                        .with_output_identity("request-deletion-policy"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .deletion_policy_named("signal.deletion.preserve-target-only")
        .run()
        .expect("request deletion policy should override schema and node defaults");

    assert_eq!(
        result.selected_deletion_policy_name.as_str(),
        "signal.deletion.preserve-target-only"
    );
    assert_eq!(
        result.selected_deletion_policy_basis,
        DeletionPolicySelectionBasis::RequestNamed
    );
}

#[test]
fn merge_budget_identity_counters_track_bounded_target_journal_scope() {
    let graph = SignalGraph::new().with_schema_registry(cross_identity_merge_schema_registry(None));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-identity-budget-counters")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(34, 0))
                        .with_output_identity("identity-budget-shared"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let matched_target = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    let unrelated_target = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(matched_target, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(35, 0))
                        .with_output_identity("identity-budget-shared"),
                ))
            })?;
            tx.read(unrelated_target, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(36, 0))
                        .with_output_identity("identity-budget-unrelated"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let result = runtime
        .merge()
        .from(feature)
        .into(main)
        .identity_matcher_named("signal.identity.output-identity-in-target-journal")
        .run()
        .unwrap();

    assert_eq!(result.identity_correspondence.target_candidate_count, 2);
    assert_eq!(result.counters.identity_target_candidates_indexed, 2);
    assert_eq!(result.identity_correspondence.source_lookup_count, 1);
    assert_eq!(result.counters.identity_source_lookups, 1);
    assert_eq!(result.identity_correspondence.ambiguous_match_count, 0);
    assert_eq!(result.counters.identity_ambiguous_match_count, 0);
    assert_eq!(
        result.identity_correspondence.rejected_admissibility_count,
        0
    );
    assert_eq!(result.counters.identity_rejected_admissibility_count, 0);
    let correspondence = result
        .identity_correspondence
        .records
        .iter()
        .find(|record| record.source_node == feature_only)
        .expect("identity correspondence record should be present");
    assert_eq!(correspondence.target_node, Some(matched_target));
    assert_eq!(correspondence.candidate_count, 1);
}

#[test]
fn runtime_merge_output_identity_matcher_rejects_ambiguous_target_journal_candidates() {
    let graph = SignalGraph::new().with_schema_registry(cross_identity_merge_schema_registry(None));
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-identity-ambiguous-candidates")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_only = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(feature_only, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(37, 0))
                        .with_output_identity("identity-ambiguous"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let target_a = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    let target_b = runtime
        .graph_mut()
        .node()
        .schema_name("signal.demo.cross-identity-merge-owned")
        .expect("known schema")
        .output_identity()
        .build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(target_a, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(38, 0))
                        .with_output_identity("identity-ambiguous"),
                ))
            })?;
            tx.read(target_b, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(39, 0))
                        .with_output_identity("identity-ambiguous"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let err = runtime
        .merge()
        .from(feature)
        .into(main)
        .identity_matcher_named("signal.identity.output-identity-in-target-journal")
        .run()
        .expect_err("ambiguous output-identity candidates must fail explicitly");

    match err {
        SignalError::BranchMergeFailed { kind, message, .. } => {
            assert_eq!(kind, BranchMergeFailureKind::UnsupportedMergeStrategy);
            assert!(message.contains("ambiguous target journal correspondence"));
            assert!(message.contains(&feature_only.to_string()));
        }
        other => panic!("unexpected error kind: {other:?}"),
    }
}

#[test]
fn merge_branch_without_established_journal_boundary_fails_explicitly() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let shared = runtime.graph_mut().node().output_identity().build();
    let feature = runtime
        .create_branch("feature-missing-merge-boundary")
        .unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(shared, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(140, 0))
                        .with_output_identity("missing-boundary-feature"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .clear_branch_merge_boundary_for_test(feature.id)
        .expect("feature branch state should be present");

    runtime.switch_branch(main.clone()).unwrap();
    let err = runtime
        .merge_branch(feature.clone(), main.clone())
        .expect_err("merge must fail explicitly when no bounded journal boundary exists");

    assert!(matches!(
        err,
        SignalError::BranchMergeFailed {
            kind: BranchMergeFailureKind::UnsupportedMergeStrategy,
            evidence: None,
            ..
        }
    ));
    assert!(
        err.to_string().contains("mutation-journal boundary"),
        "failure message should explain that bounded journal proof is required"
    );
}

#[test]
fn repeated_merge_after_target_restore_stays_bounded_and_history_honest() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("feature-repeated-restore-merge")
        .unwrap();
    let mut runtime_ctx = ();

    runtime.switch_branch(feature.clone()).unwrap();
    let first = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(first, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(96, 0))
                        .with_output_identity("restore-cycle-first"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let first_merge = runtime.merge_branch(feature.clone(), main.clone()).unwrap();
    assert!(
        first_merge
            .records
            .iter()
            .any(|record| record.source_node == first),
        "first merge should include the initial source-only node"
    );
    let merged_snapshot = runtime.capture_branch_snapshot(main.clone()).unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let second = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(second, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(97, 0))
                        .with_output_identity("restore-cycle-second"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let unrelated = runtime.graph_mut().node().output_identity().build();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(unrelated, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(98, 0))
                        .with_output_identity("restore-cycle-unrelated"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let branch_merge_replay_before_restore = runtime
        .graph()
        .replay_events()
        .iter()
        .filter(|event| event.kind == ReplayEventKind::BranchMerged)
        .count();
    let branch_merge_lineage_before_restore = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .filter(|record| matches!(record.kind, LineageRecordKind::BranchMerge { .. }))
        .count();

    runtime
        .restore_branch_snapshot(main.clone(), &merged_snapshot)
        .unwrap();

    assert_eq!(
        runtime
            .graph()
            .replay_events()
            .iter()
            .filter(|event| event.kind == ReplayEventKind::BranchMerged)
            .count(),
        branch_merge_replay_before_restore,
        "target restore between merge cycles must not fabricate extra branch merge replay events"
    );
    assert_eq!(
        runtime
            .graph()
            .observe()
            .lineage_records()
            .iter()
            .filter(|record| matches!(record.kind, LineageRecordKind::BranchMerge { .. }))
            .count(),
        branch_merge_lineage_before_restore,
        "target restore between merge cycles must not fabricate extra branch merge lineage"
    );

    let second_merge = runtime.merge_branch(feature, main).unwrap();
    assert!(
        second_merge.counters.final_candidate_breadth
            == second_merge.planned_candidates.nodes.len() as u64,
        "repeated merge after restore should remain bounded to the branch mutation candidate set"
    );
    assert!(
        matches!(
            second_merge.boundary_witness.kind,
            MergeBoundaryWitnessKind::MutationJournalBoundary
        ),
        "repeated merge after restore must remain anchored to the mutation-journal witness"
    );
    assert!(
        second_merge
            .records
            .iter()
            .all(|record| record.source_node != first),
        "already-merged source nodes must stay retired after target restore"
    );
    assert!(
        second_merge
            .records
            .iter()
            .any(|record| record.source_node == second),
        "new source-side work should remain merge-visible after target restore"
    );
}
