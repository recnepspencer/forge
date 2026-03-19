use crate::data::dependency::DependencySnapshot;
use crate::facade::*;
use crate::logic::transaction::BranchMergeResolutionRequirement;
use crate::tests::support::*;
use crate::data::graph::BranchStructuralDelta;

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
        ConflictMergePolicy::RejectSharedStateConflict
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
        runtime.graph().observe().lineage_records().iter().any(|record| matches!(
            record.kind,
            LineageRecordKind::BranchMerge { .. }
        )),
        "merge should emit branch merge lineage"
    );
    assert!(
        runtime.graph().observe().lineage_records().iter().any(|record| matches!(
            record.kind,
            LineageRecordKind::ArtifactMerge { .. }
        )),
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
    assert!(filtered.records[0].structural_deltas.iter().any(|delta| matches!(
        delta,
        BranchStructuralDelta::RuntimeArtifactChanged(_)
    )));
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
        let entry = graph.get_entry_mut(source_only).unwrap();
        let mut runtime_artifact = entry
            .get_runtime_artifact_state()
            .cloned()
            .expect("source-only node should have runtime artifact state");
        runtime_artifact.merge_authority = ArtifactMergeAuthority {
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
    let result = runtime.merge_branch(feature, runtime.observe().current_branch()).unwrap();

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
        !result.counters.branch_wide_scan_performed,
        "tracked branch-local mutation scope should suppress whole-live merge breadth in this case"
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
        matches!(result.candidate_scope, MergeCandidateScope::CandidateNodeSet(_)),
        "tracked branch-local mutations should lower an explicit candidate node set"
    );
    assert!(
        !result.counters.branch_wide_scan_performed,
        "candidate-node merge should not report a whole-live branch scan"
    );
    assert!(
        result.records.len() < runtime.graph().active_node_count(),
        "narrow candidate scope should plan fewer nodes than the full live authority surface"
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
    let second_merge = runtime.merge_branch(feature, runtime.observe().current_branch()).unwrap();

    assert!(
        matches!(second_merge.candidate_scope, MergeCandidateScope::CandidateNodeSet(_)),
        "repeated merge should continue using the source ledger candidate set"
    );
    assert!(
        second_merge.records.iter().all(|record| record.source_node != first),
        "source ledger should advance past already-merged nodes"
    );
    assert!(
        second_merge.records.iter().any(|record| record.source_node == second),
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
        let entry = graph.get_entry_mut(source_only).unwrap();
        entry.set_retained_diagnostic_artifact(Some(RetainedDiagnosticArtifact {
            changed_regions: CanonicalChangedRegions::new([]),
            labels: vec!["retained-only-label".to_string()],
            keyed_family: None,
            keyed_key: None,
            reuse_certification: None,
        }));
        graph.record_branch_mutation_retained_artifact(source_only);
    }

    runtime.switch_branch(main).unwrap();
    let result = runtime.merge_branch(feature, runtime.observe().current_branch()).unwrap();

    assert!(
        matches!(result.candidate_scope, MergeCandidateScope::CandidateNodeSet(ref nodes) if nodes.is_empty()),
        "retained-only churn should produce an explicit empty merge candidate set, not a whole-branch fallback"
    );
    assert!(
        result.records.is_empty(),
        "diagnostics-only retained churn should not create merge reconciliation work"
    );
    assert!(
        !result.counters.branch_wide_scan_performed,
        "diagnostics-only retained churn must not report a broad branch scan"
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
            let evidence = evidence.expect("conflict evidence should be present");
            assert_eq!(evidence.divergence, BranchMergeDivergence::SharedStateConflict);
            assert_eq!(
                evidence.reconciliation_policy.conflict,
                ConflictMergePolicy::RejectSharedStateConflict
            );
            assert_eq!(evidence.summary.total_conflict_count, 1);
            assert_eq!(evidence.summary.comparable_mismatch_count, 1);
            assert_eq!(
                evidence.summary.primary_conflict_kind,
                Some(BranchMergeConflictKind::RuntimeArtifactMismatch)
            );
            assert!(
                evidence
                    .summary
                    .required_resolution
                    .contains(&BranchMergeResolutionRequirement::ReconcileComparableState)
            );
            assert!(
                evidence
                    .summary
                    .required_resolution
                    .contains(&BranchMergeResolutionRequirement::ReconcileRuntimeArtifactState)
            );
            let failure = runtime
                .observe()
                .latest_failure_diagnostics()
                .expect("failed merge should record failure diagnostics");
            assert!(
                failure.message.contains("primary=Some(RuntimeArtifactMismatch)"),
                "failure diagnostics should surface the primary conflict class"
            );
            assert!(
                failure
                    .message
                    .contains("ReconcileRuntimeArtifactState"),
                "failure diagnostics should surface required merge resolution"
            );
            assert!(
                runtime.graph().replay_events().iter().any(|event| {
                    event.kind == ReplayEventKind::FailureRecorded
                        && event
                            .detail
                            .as_deref()
                            .map(|detail| detail.contains("ReconcileRuntimeArtifactState"))
                            .unwrap_or(false)
                }),
                "failed merge should emit a failure replay detail with required resolution"
            );
            assert!(
                !runtime.graph().replay_events().iter().any(|event| {
                    event.kind == ReplayEventKind::BranchMerged
                }),
                "failed merge must not emit a false branch-merged replay boundary"
            );
            assert_eq!(evidence.records.len(), 1);
            assert_eq!(evidence.records[0].source_node, shared);
            assert!(
                evidence.records[0]
                    .conflict_kinds
                    .contains(&BranchMergeConflictKind::ComparableMismatch)
            );
            assert!(
                evidence.records[0]
                    .conflict_kinds
                    .contains(&BranchMergeConflictKind::RuntimeArtifactMismatch)
            );
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
            let evidence = evidence.expect("topology conflict evidence should be present");
            assert_eq!(evidence.divergence, BranchMergeDivergence::SharedStateConflict);
            assert_eq!(
                evidence.summary.primary_conflict_kind,
                Some(BranchMergeConflictKind::DependencyTopologyMismatch)
            );
            assert!(
                evidence
                    .summary
                    .required_resolution
                    .contains(&BranchMergeResolutionRequirement::ReconcileDependencyTopology)
            );
            assert_eq!(evidence.records.len(), 1);
            assert!(
                evidence.records[0]
                    .conflict_kinds
                    .contains(&BranchMergeConflictKind::DependencyTopologyMismatch)
            );
        }
        other => panic!("expected topology conflict failure, got {other:?}"),
    }
}

#[test]
fn merge_branch_dependency_snapshot_conflict_surfaces_structural_requirement() {
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

    let err = runtime.merge_branch(feature, main).unwrap_err();
    match err {
        SignalError::BranchMergeFailed { kind, evidence, .. } => {
            assert_eq!(
                kind,
                BranchMergeFailureKind::DivergenceRequiresConflictResolution
            );
            let evidence = evidence.expect("snapshot conflict evidence should be present");
            assert_eq!(evidence.divergence, BranchMergeDivergence::SharedStateConflict);
            assert_eq!(
                evidence.summary.primary_conflict_kind,
                Some(BranchMergeConflictKind::DependencySnapshotMismatch)
            );
            assert!(
                evidence
                    .summary
                    .required_resolution
                    .contains(&BranchMergeResolutionRequirement::ReconcileDependencySnapshot)
            );
            assert_eq!(evidence.records.len(), 1);
            assert!(
                evidence.records[0]
                    .conflict_kinds
                    .contains(&BranchMergeConflictKind::DependencySnapshotMismatch)
            );
        }
        other => panic!("expected dependency snapshot conflict failure, got {other:?}"),
    }
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
    assert!(matches!(
        result.candidate_scope,
        MergeCandidateScope::CandidateNodeSet(_)
    ));
    assert!(
        result
            .records
            .iter()
            .any(|record| record.action == ArtifactMergeAction::IntroducedIntoTarget)
    );
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
    assert!(
        result
            .records
            .iter()
            .any(|record| record.action == ArtifactMergeAction::IntroducedIntoTarget)
    );
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

    let base_snapshot = runtime.capture_snapshot();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-active-restore-fast-forward").unwrap();

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
fn repeated_merge_after_target_restore_stays_bounded_and_history_honest() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-repeated-restore-merge").unwrap();
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
        first_merge.records.iter().any(|record| record.source_node == first),
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
        matches!(second_merge.candidate_scope, MergeCandidateScope::CandidateNodeSet(_)),
        "repeated merge after restore should remain bounded to the branch mutation candidate set"
    );
    assert!(
        !second_merge.counters.branch_wide_scan_performed,
        "repeated merge after restore must not fall back to a whole-live branch scan"
    );
    assert!(
        second_merge.records.iter().all(|record| record.source_node != first),
        "already-merged source nodes must stay retired after target restore"
    );
    assert!(
        second_merge.records.iter().any(|record| record.source_node == second),
        "new source-side work should remain merge-visible after target restore"
    );
}
