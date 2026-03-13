use crate::facade::*;
use crate::tests::support::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Domain {
    Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Impact {
    One,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ev {
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    Slow,
}

fn build_runtime(graph: SignalGraph) -> SignalRuntime<Domain, Impact, Ev, (), Tier> {
    let _ = Domain::Cache;
    let _ = Impact::One;
    SignalRuntime::builder(graph)
        .with_kernel_defaults()
        .with_domains::<Domain>()
        .with_impacts::<Impact>()
        .with_events::<Ev>()
        .with_tiers::<Tier>()
        .checkpoint_barrier(CheckpointBarrier::PerOperation)
        .build()
}

#[test]
fn explain_reports_changed_upstream() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut source_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut source_v2).unwrap();

    let explanation = graph.observe().explain(dependent).unwrap();
    assert_eq!(explanation.node, dependent);
    assert!(matches!(
        explanation.upstream.as_slice(),
        [UpstreamCause::Changed { source: changed, aspect, cached_version: 1, current_version: 2, .. }]
        if *changed == source && *aspect == ASPECT_A
    ));
}

#[test]
fn explain_reports_clean_upstream_when_snapshot_matches() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    let explanation = graph.observe().explain(dependent).unwrap();
    assert!(matches!(
        explanation.upstream.as_slice(),
        [UpstreamCause::Clean { source: clean, aspect, cached_version: 1, current_version: 1, .. }]
        if *clean == source && *aspect == ASPECT_A
    ));
}

#[test]
fn explain_reports_skipped_by_comparator_via_runtime_policy() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let middle = graph.node().build();
    let dependent = graph.node().build();
    graph.append_dependency(middle, source, ASPECT_A).unwrap();
    graph
        .append_dependency(dependent, middle, ASPECT_A)
        .unwrap();

    let mut runtime = build_runtime(graph);
    runtime.set_node_tier(dependent, Tier::Slow);
    runtime.set_tier_policy(
        TierPolicy::new(
            Tier::Slow,
            DependencyMode::AutoDiscovered,
            DirtyPropagation::Immediate,
            EvaluationTrigger::LazyPull,
        )
        .with_default_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 }),
    );

    let mut source_v10 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    let mut source_v12 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(12, 0));
    let mut middle_v100 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(100, 0));
    let mut middle_v102 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(102, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1_000, 0));

    evaluate(runtime.graph_mut(), source, &mut source_v10).unwrap();
    evaluate(runtime.graph_mut(), middle, &mut middle_v100).unwrap();
    evaluate(runtime.graph_mut(), dependent, &mut dependent_compute).unwrap();
    mark_dirty(runtime.graph_mut(), source, ASPECT_A).unwrap();
    evaluate(runtime.graph_mut(), source, &mut source_v12).unwrap();
    evaluate(runtime.graph_mut(), middle, &mut middle_v102).unwrap();

    let explanation = runtime.observe().explain(dependent).unwrap();
    assert!(explanation.upstream.iter().any(|cause| matches!(
        cause,
        UpstreamCause::SkippedByComparator {
            source: skipped,
            aspect,
            cached_version: 100,
            current_version: 102,
            ..
        } if *skipped == middle && *aspect == ASPECT_A
    )));
}

#[test]
fn explicit_omit_policy_surfaces_unavailable_artifacts() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    graph.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit)
            .with_provenance_retention(ArtifactRetentionPolicy::Omit),
    );

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();

    let (explanation, explanation_mode) = graph.explain_artifact(dependent).unwrap();
    let (provenance, provenance_mode) = graph.provenance_artifact(dependent).unwrap();

    assert!(explanation.is_none());
    assert!(provenance.is_none());
    assert_eq!(explanation_mode, ArtifactMaterializationMode::Unavailable);
    assert_eq!(provenance_mode, ArtifactMaterializationMode::Unavailable);
}

#[test]
fn explicit_retained_and_reconstructed_artifact_apis_match_policy() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let bootstrap = graph
        .build_evaluation_plan(&[source, dependent], EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan_with_precompute(&bootstrap, &|node, view| {
            let result = if node == source {
                view.finish(version_ab(1, 0))
            } else {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(version))
            };
            Ok(result)
        })
        .unwrap();
    assert!(graph
        .observe()
        .retained_explanation_artifact(dependent)
        .is_some());
    assert!(graph
        .observe()
        .retained_provenance_artifact(dependent)
        .is_some());
    assert_eq!(
        graph
            .observe()
            .retained_explanation_artifact(dependent)
            .unwrap()
            .materialization_mode,
        ArtifactMaterializationMode::Retained
    );
    assert_eq!(
        graph
            .observe()
            .retained_provenance_artifact(dependent)
            .unwrap()
            .materialization_mode,
        ArtifactMaterializationMode::Retained
    );

    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    assert!(graph
        .observe()
        .retained_explanation_artifact(dependent)
        .is_none());
    assert!(graph
        .observe()
        .retained_provenance_artifact(dependent)
        .is_none());
    let reconstructed_explanation = graph
        .observe()
        .reconstruct_explanation_artifact(dependent)
        .unwrap();
    let reconstructed_provenance = graph
        .observe()
        .reconstruct_provenance_artifact(dependent)
        .unwrap();
    assert_eq!(
        reconstructed_explanation.materialization_mode,
        ArtifactMaterializationMode::Reconstructed
    );
    assert_eq!(
        reconstructed_provenance.materialization_mode,
        ArtifactMaterializationMode::Reconstructed
    );
    assert!(!reconstructed_explanation.upstream.is_empty());
    assert!(reconstructed_provenance
        .vertices
        .iter()
        .any(|vertex| vertex.node == dependent));
    assert_eq!(
        reconstructed_provenance.causal_links,
        reconstructed_explanation.causal_links
    );
}

#[test]
fn market_runtime_policy_presets_expose_distinct_operational_shapes() {
    let kernel = SignalRuntimePolicy::kernel();
    let fintech = SignalRuntimePolicy::fintech();
    let game = SignalRuntimePolicy::game_engine();
    let web = SignalRuntimePolicy::web_development();
    let fintech_plan = SignalDeploymentPreset::Fintech.recommended();

    assert_eq!(kernel.profile, DiagnosticsProfile::Forensic);
    assert_eq!(fintech.profile, DiagnosticsProfile::Development);
    assert_eq!(game.profile, DiagnosticsProfile::Operational);
    assert_eq!(web.profile, DiagnosticsProfile::Operational);
    assert!(
        kernel.parallel_admission.full_parallel_min_tasks
            >= fintech.parallel_admission.full_parallel_min_tasks
    );
    assert!(fintech.retain_flow_explanation);
    assert!(!game.retains_explanation_facts());
    assert_eq!(fintech_plan.runtime_policy, fintech);
}

#[test]
fn explain_reports_condition_deferred_for_on_demand_nodes() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().on_demand().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut source_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate_on_demand(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut source_v2).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    let explanation = graph.observe().explain(dependent).unwrap();
    assert!(explanation
        .upstream
        .iter()
        .any(|cause| matches!(cause, UpstreamCause::ConditionDeferred { source: deferred, aspect, condition: EvaluationCondition::OnDemand, decision: ConditionDecision::Deferred, .. } if *deferred == source && *aspect == ASPECT_A)));
}

#[test]
fn explain_reports_missing_snapshot_and_dependency_removed() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    let mut source_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut source_compute).unwrap();

    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let missing_snapshot = graph.observe().explain(dependent).unwrap();
    assert!(missing_snapshot
        .upstream
        .iter()
        .any(|cause| matches!(cause, UpstreamCause::MissingSnapshot { source: missing, aspect, current_version: Some(1), .. } if *missing == source && *aspect == ASPECT_A)));
    assert!(missing_snapshot.causal_links.iter().any(|link| {
        matches!(link.disposition, CausalDisposition::Conservative)
            && link.kind == "MissingSnapshot"
    }));

    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    graph.drop_dependency(dependent, source, ASPECT_A).unwrap();

    let removed = graph.observe().explain(dependent).unwrap();
    assert!(removed
        .upstream
        .iter()
        .any(|cause| matches!(cause, UpstreamCause::DependencyRemoved { source: removed_source, aspect, cached_version: 1, .. } if *removed_source == source && *aspect == ASPECT_A)));
}

#[test]
fn explanation_surfaces_causality_and_trace_summary() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph
        .set_causality(
            node,
            Some(CausalityMetadata {
                kind: "bridge".to_string(),
                fields: [("commit".to_string(), "42".to_string())]
                    .into_iter()
                    .collect(),
            }),
        )
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, node, &mut compute).unwrap();

    let explanation = graph.observe().explain(node).unwrap();
    assert_eq!(explanation.causality.as_ref().unwrap().kind, "bridge");
    assert!(explanation.historical_artifact_record.is_some());
    assert!(format!("{explanation}").contains("Causality: bridge"));
}

#[test]
fn explanation_surfaces_retained_reuse_certification() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let entry = graph.get_entry_mut(node).unwrap();
    entry.set_runtime_artifact_state(Some(RuntimeArtifactState {
        recomputed: false,
        memoized_origin: MemoizedResultOrigin::MemoizedFromCache,
        reuse_basis: ReuseBasis::Reused {
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::None,
        },
        reuse_boundary_context: Some(ReuseBoundaryContext {
            topology_regime: 1,
            tolerance_regime: VersionComparatorPolicy::Exact,
            semantic_region: ReuseSemanticRegionIdentity::new(
                node,
                false,
                Vec::new(),
                ContextRequirement::None,
            ),
            authority_policy: AuthorityPolicy::SpeculativeThenReconcile,
        }),
        ..RuntimeArtifactState::default()
    }));
    entry.set_retained_diagnostic_artifact(Some(RetainedDiagnosticArtifact {
        changed_regions: CanonicalChangedRegions::default(),
        labels: Vec::new(),
        keyed_family: None,
        keyed_key: None,
        reuse_certification: Some(ReuseCertificationRecord {
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::None,
            proofs: vec![ReuseBoundaryProof {
                boundary: ArtifactSemanticBoundary::TopologyRegime,
                satisfied: true,
            }],
        }),
    }));

    let explanation = graph.observe().explain(node).unwrap();
    assert_eq!(
        explanation.reuse_basis,
        Some(ReuseBasis::Reused {
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::None,
        })
    );
    assert_eq!(
        explanation
            .reuse_certification
            .as_ref()
            .map(|record| record.proofs.len()),
        Some(1)
    );
    assert!(format!("{explanation}").contains("Reuse certification proofs: 1"));
}

#[test]
fn dependency_inspection_apis_are_deterministic() {
    let mut graph = SignalGraph::new();
    let root = graph.node().build();
    let middle = graph.node().build();
    let target = graph.node().build();
    graph.append_dependency(middle, root, ASPECT_A).unwrap();
    graph.append_dependency(target, middle, ASPECT_B).unwrap();

    assert_eq!(graph.dependencies_of(target).unwrap().len(), 1);
    assert_eq!(graph.subscribers_of(root).unwrap(), &[middle]);
    assert!(graph.depends_on(target, middle, ASPECT_B).unwrap());
    assert_eq!(
        graph.observe().dependency_chain_to(root, target).unwrap(),
        Some(vec![root, middle, target])
    );
}

#[test]
fn dot_export_contains_state_color_and_edge_labels() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().on_demand().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();

    let dot = graph.observe().to_dot();
    assert!(dot.contains(&format!("\"{}\"", source)));
    assert!(dot.contains("fillcolor=green"));
    assert!(dot.contains("aspect:0"));
    assert!(dot.contains("scope:"));
}

#[test]
fn metrics_snapshots_reflect_runtime_activity() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let mut runtime = build_runtime(graph);

    let outcome = runtime
        .transaction(&mut (), |transaction| {
            transaction.mark_dirty(source, ASPECT_A)?;
            transaction.emit_event(Ev::Tick);
            transaction.flush_events(CheckpointBarrier::PerOperation)?;
            Ok(())
        })
        .unwrap();

    assert_eq!(outcome.outcome, TransactionOutcome::Committed);
    assert!(
        runtime
            .observe()
            .metrics()
            .transaction
            .transaction_begin_count
            >= 1
    );
    assert!(
        runtime
            .observe()
            .metrics()
            .transaction
            .transaction_commit_count
            >= 1
    );
    assert!(runtime.observe().metrics().checkpoint.event_flushes >= 1);
    assert!(
        runtime
            .graph()
            .observe()
            .metrics()
            .invalidation
            .invalidation_nodes_visited
            >= 1
    );
}

#[test]
fn explanation_is_deterministic_with_multiple_upstreams_and_mixed_states() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().build();
    let source_b = graph.node().build();
    let source_c = graph.node().build();
    let dependent = graph.node().on_demand().build();
    let mut dependencies = DependencyBatchBuilder::new(&mut graph);
    dependencies
        .append_dependency(dependent, source_b, ASPECT_B)
        .unwrap()
        .append_dependency(dependent, source_a, ASPECT_A)
        .unwrap()
        .append_dependency(dependent, source_c, ASPECT_A)
        .unwrap();
    dependencies.commit().unwrap();

    let mut source_a_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut source_a_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut source_b_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut source_c_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(3, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source_a, &mut source_a_v1).unwrap();
    evaluate(&mut graph, source_b, &mut source_b_v1).unwrap();
    evaluate(&mut graph, source_c, &mut source_c_v1).unwrap();
    evaluate_on_demand(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty(&mut graph, source_a, ASPECT_A).unwrap();
    evaluate(&mut graph, source_a, &mut source_a_v2).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    let explanation = graph.observe().explain(dependent).unwrap();
    let rendered = format!("{explanation}");
    assert!(matches!(
        explanation.upstream.first(),
        Some(UpstreamCause::ConditionDeferred { source, .. }) if *source == source_a
    ));
    assert!(explanation.upstream.iter().any(|cause| matches!(
        cause,
        UpstreamCause::Clean { source, aspect, cached_version: 1, current_version: 1, .. }
        if *source == source_b && *aspect == ASPECT_B
    )));
    assert!(rendered.contains("condition OnDemand/Deferred"));
}

#[test]
fn rollback_preserves_committed_explanation_and_increments_rollback_metric() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let mut runtime = build_runtime(graph);

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut dependent_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(runtime.graph_mut(), source, &mut source_v1).unwrap();
    evaluate(runtime.graph_mut(), dependent, &mut dependent_v1).unwrap();
    let before = runtime.observe().explain(dependent).unwrap();
    let rollback_before = runtime
        .observe()
        .metrics()
        .transaction
        .transaction_rollback_count;

    let err = runtime.transaction(&mut (), |tx| {
        tx.mark_dirty(source, ASPECT_A)?;
        tx.evaluate_with_plan(
            dependent,
            &|view| Ok(view.finish(version_ab(99, 0))),
            EvaluationRequestMode::Default,
        )?;
        Err(SignalError::invalid_input("rollback for test"))
    });
    assert!(err.is_err());

    let after = runtime.observe().explain(dependent).unwrap();
    assert_eq!(
        before.historical_artifact_record,
        after.historical_artifact_record
    );
    assert_eq!(before.upstream, after.upstream);
    assert_eq!(
        runtime
            .observe()
            .metrics()
            .transaction
            .transaction_rollback_count,
        rollback_before + 1
    );
}

#[test]
fn flow_diagnostics_attach_event_epochs_after_successful_commit() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let mut runtime = build_runtime(graph);

    runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                source,
                &|view| Ok(view.finish(version_ab(1, 0))),
                EvaluationRequestMode::Default,
            )?;
            tx.evaluate_with_plan(
                dependent,
                &|view| {
                    let version = view.read_aspect_version(source, ASPECT_A)?;
                    Ok(view.finish(NodeEvaluationResult::from_version(version)))
                },
                EvaluationRequestMode::Default,
            )?;
            tx.emit_event(Ev::Tick);
            tx.flush_events(CheckpointBarrier::PerOperation)?;
            Ok(())
        })
        .unwrap();

    let flow = runtime.observe().latest_flow_diagnostics().unwrap();
    assert_eq!(flow.event_epochs.len(), 1);
    assert_eq!(flow.event_epochs[0].outcome, EventEpochOutcome::Committed);
    assert_eq!(
        flow.event_epochs[0].barrier,
        CheckpointBarrier::PerOperation
    );
    assert_eq!(flow.event_epochs[0].committed_subscriber_count, 0);
    assert_eq!(flow.event_epochs[0].failed_subscriber_position, None);
}

#[test]
fn fillet_style_explanation_stays_local_to_the_changed_partition_scope() {
    let mut graph = SignalGraph::new();
    let feature_edit = graph.node().partitioned_output().build();
    let unrelated_region = graph.node().partitioned_output().build();
    let fillet = graph.node().build();
    graph
        .append_partition_detail_dependency(
            fillet,
            feature_edit,
            ASPECT_A,
            "surface",
            "fillet-band",
        )
        .unwrap();

    let bootstrap = graph
        .build_evaluation_plan(
            &[feature_edit, unrelated_region, fillet],
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    graph
        .execute_prepared_plan_with_precompute(&bootstrap, &|node, view| {
            let result = if node == fillet {
                let version = view.read_partitioned_aspect_version(
                    feature_edit,
                    ASPECT_A,
                    PartitionSubscription::partition_and_detail("surface", "fillet-band"),
                )?;
                view.finish(NodeEvaluationResult::from_version(version))
            } else {
                view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))
            };
            Ok(result)
        })
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        feature_edit,
        ASPECT_A,
        &[ChangedRegion::new("surface").with_detail("fillet-band")],
    )
    .unwrap();
    let feature_update = graph
        .build_evaluation_plan(&[feature_edit], EvaluationRequestMode::Default)
        .unwrap();
    graph
        .execute_prepared_plan_with_precompute(&feature_update, &|_node, view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(2, 0))
                    .with_changed_region(ChangedRegion::new("surface").with_detail("fillet-band")),
            ))
        })
        .unwrap();

    let explanation = graph.observe().explain(fillet).unwrap();
    let summary = explanation.diagnostics_summary(DiagnosticsProfile::Development);
    assert!(explanation.causal_links.iter().any(|link| {
        link.source == Some(feature_edit)
            && link.scope.validation_scope.as_ref().is_some_and(|scope| {
                scope.partition.0 == "surface" && scope.detail.as_deref() == Some("fillet-band")
            })
    }));
    assert!(!explanation
        .causal_links
        .iter()
        .any(|link| link.source == Some(unrelated_region)));
    assert!(summary.triage_classes.contains(&"locality".to_string()));
    assert_eq!(summary.discarded_scope_count, 0);
    assert!(summary
        .scope_provenance_kinds
        .iter()
        .any(|kind| kind == "Direct"));
}

#[test]
fn flow_cause_samples_surface_locality_triage_without_false_rewiring() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let fillet = graph.node().build();
    graph
        .append_partition_detail_dependency(fillet, source, ASPECT_A, "surface", "fillet-band")
        .unwrap();
    let mut runtime = build_runtime(graph);

    runtime
        .transaction(&mut (), |tx| {
            tx.evaluate_with_plan(
                source,
                &|view| Ok(view.finish(version_ab(1, 0))),
                EvaluationRequestMode::Default,
            )?;
            tx.evaluate_with_plan(
                fillet,
                &|view| {
                    let version = view.read_partitioned_aspect_version(
                        source,
                        ASPECT_A,
                        PartitionSubscription::partition_and_detail("surface", "fillet-band"),
                    )?;
                    Ok(view.finish(NodeEvaluationResult::from_version(version)))
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();

    let flow = runtime.observe().latest_flow_diagnostics().unwrap();
    assert!(flow.cause_samples.iter().any(|sample| {
        sample.node == fillet
            && sample.suspect_classes.contains(&"locality".to_string())
            && !sample.suspect_classes.contains(&"rewiring".to_string())
            && sample.scope_kinds.iter().any(|kind| kind == "Direct")
    }));
}
