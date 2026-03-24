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

    let (explanation, explanation_mode) =
        graph.materialize_explanation_artifact(dependent).unwrap();
    let (provenance, provenance_mode) = graph.materialize_provenance_artifact(dependent).unwrap();

    assert!(explanation.is_none());
    assert!(provenance.is_none());
    assert_eq!(explanation_mode, DiagnosticsAvailability::OmittedByTier);
    assert_eq!(provenance_mode, DiagnosticsAvailability::OmittedByTier);
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
        .materialize()
        .retained_explanation_artifact(dependent)
        .is_some());
    assert!(graph
        .observe()
        .materialize()
        .retained_provenance_artifact(dependent)
        .is_some());
    assert_eq!(
        graph
            .observe()
            .materialize()
            .retained_explanation_artifact(dependent)
            .unwrap()
            .materialization_mode,
        DiagnosticsAvailability::RetainedAvailable
    );
    assert_eq!(
        graph
            .observe()
            .materialize()
            .retained_provenance_artifact(dependent)
            .unwrap()
            .materialization_mode,
        DiagnosticsAvailability::RetainedAvailable
    );

    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    assert!(graph
        .observe()
        .materialize()
        .retained_explanation_artifact(dependent)
        .is_none());
    assert!(graph
        .observe()
        .materialize()
        .retained_provenance_artifact(dependent)
        .is_none());
    let reconstructed_explanation = graph
        .observe()
        .materialize()
        .reconstruct_explanation_artifact(dependent)
        .unwrap();
    let reconstructed_provenance = graph
        .observe()
        .materialize()
        .reconstruct_provenance_artifact(dependent)
        .unwrap();
    assert_eq!(
        reconstructed_explanation.materialization_mode,
        DiagnosticsAvailability::ReconstructedAvailable
    );
    assert_eq!(
        reconstructed_provenance.materialization_mode,
        DiagnosticsAvailability::ReconstructedAvailable
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
    assert!(
        graph
            .observe()
            .metrics()
            .storage
            .hot_path_artifact_reconstruction_count
            >= 2
    );
}

#[test]
fn market_runtime_policy_presets_expose_distinct_operational_shapes() {
    let kernel = SignalRuntimePolicy::kernel();
    let fintech = SignalRuntimePolicy::fintech();
    let game = SignalRuntimePolicy::game_engine();
    let web = SignalRuntimePolicy::web_development();
    let fintech_plan = SignalDeploymentPreset::Fintech.recommended();

    assert_eq!(kernel.tier, DiagnosticsTier::Forensic);
    assert_eq!(fintech.tier, DiagnosticsTier::Development);
    assert_eq!(game.tier, DiagnosticsTier::Operational);
    assert_eq!(web.tier, DiagnosticsTier::Operational);
    assert!(
        kernel.parallel_admission.full_parallel_min_tasks
            >= fintech.parallel_admission.full_parallel_min_tasks
    );
    assert!(fintech.retention_budget.retain_flow_explanation);
    assert!(!game.retains_explanation_facts());
    assert_eq!(fintech_plan.runtime_policy, fintech);
}

#[test]
fn artifact_access_counters_attribute_lane_api_and_denial_reason() {
    let mut retained_graph = SignalGraph::new();
    let retained_source = retained_graph.node().build();
    let retained_dependent = retained_graph.node().build();
    retained_graph
        .append_dependency(retained_dependent, retained_source, ASPECT_A)
        .unwrap();
    retained_graph.set_runtime_policy(SignalRuntimePolicy::development());
    let retained_bootstrap = retained_graph
        .build_evaluation_plan(
            &[retained_source, retained_dependent],
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    retained_graph
        .execute_prepared_plan_with_precompute(&retained_bootstrap, &|node, view| {
            let result = if node == retained_source {
                view.finish(version_ab(1, 0))
            } else {
                let version = view.read_aspect_version(retained_source, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(version))
            };
            Ok(result)
        })
        .unwrap();
    assert!(retained_graph
        .observe()
        .materialize()
        .retained_explanation_artifact(retained_dependent)
        .is_some());
    assert!(retained_graph
        .observe()
        .materialize()
        .retained_provenance_artifact(retained_dependent)
        .is_some());
    let retained_metrics = retained_graph.observe().metrics();
    assert_eq!(retained_metrics.storage.retained_forensic_read_count, 2);
    assert_eq!(retained_metrics.storage.retained_artifact_read_count, 2);
    assert_eq!(
        retained_metrics
            .storage
            .explicit_cold_materialization_request_count,
        0
    );

    let mut reconstructed_graph = SignalGraph::new();
    let reconstructed_source = reconstructed_graph.node().build();
    let reconstructed_dependent = reconstructed_graph.node().build();
    reconstructed_graph
        .append_dependency(reconstructed_dependent, reconstructed_source, ASPECT_A)
        .unwrap();
    reconstructed_graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let mut reconstructed_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(
        &mut reconstructed_graph,
        reconstructed_source,
        &mut reconstructed_compute,
    )
    .unwrap();
    evaluate(
        &mut reconstructed_graph,
        reconstructed_dependent,
        &mut reconstructed_compute,
    )
    .unwrap();
    reconstructed_graph
        .materialize_explanation_artifact(reconstructed_dependent)
        .unwrap();
    reconstructed_graph
        .materialize_provenance_artifact(reconstructed_dependent)
        .unwrap();
    let reconstructed_metrics = reconstructed_graph.observe().metrics();
    assert_eq!(
        reconstructed_metrics
            .storage
            .explicit_cold_materialization_request_count,
        2
    );
    assert_eq!(
        reconstructed_metrics
            .storage
            .cold_explanation_reconstruction_count,
        1
    );
    assert_eq!(
        reconstructed_metrics
            .storage
            .cold_provenance_reconstruction_count,
        1
    );
    assert_eq!(
        reconstructed_metrics
            .storage
            .reconstructed_artifact_read_count,
        2
    );

    let mut omitted_graph = SignalGraph::new();
    let omitted_source = omitted_graph.node().build();
    let omitted_dependent = omitted_graph.node().build();
    omitted_graph
        .append_dependency(omitted_dependent, omitted_source, ASPECT_A)
        .unwrap();
    omitted_graph.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit)
            .with_provenance_retention(ArtifactRetentionPolicy::Omit),
    );
    let mut omitted_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut omitted_graph, omitted_source, &mut omitted_compute).unwrap();
    evaluate(&mut omitted_graph, omitted_dependent, &mut omitted_compute).unwrap();
    assert_eq!(
        omitted_graph
            .materialize_explanation_artifact(omitted_dependent)
            .unwrap()
            .1,
        DiagnosticsAvailability::OmittedByTier
    );
    assert_eq!(
        omitted_graph
            .materialize_provenance_artifact(omitted_dependent)
            .unwrap()
            .1,
        DiagnosticsAvailability::OmittedByTier
    );
    let omitted_metrics = omitted_graph.observe().metrics();
    assert_eq!(
        omitted_metrics.storage.denied_reconstruction_by_tier_count,
        2
    );
    assert_eq!(
        omitted_metrics
            .storage
            .denied_reconstruction_explanation_api_count,
        1
    );
    assert_eq!(
        omitted_metrics
            .storage
            .denied_reconstruction_provenance_api_count,
        1
    );

    let mut denied_graph = SignalGraph::new();
    let denied_source = denied_graph.node().build();
    let denied_dependent = denied_graph.node().build();
    denied_graph
        .append_dependency(denied_dependent, denied_source, ASPECT_A)
        .unwrap();
    let mut denied_policy = SignalRuntimePolicy::operational();
    denied_policy
        .reconstruction_budget
        .allow_explanation_reconstruction = false;
    denied_policy
        .reconstruction_budget
        .allow_provenance_reconstruction = false;
    denied_graph.set_runtime_policy(denied_policy);
    let mut denied_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut denied_graph, denied_source, &mut denied_compute).unwrap();
    evaluate(&mut denied_graph, denied_dependent, &mut denied_compute).unwrap();
    assert_eq!(
        denied_graph
            .materialize_explanation_artifact(denied_dependent)
            .unwrap()
            .1,
        DiagnosticsAvailability::DeniedByBudget
    );
    assert_eq!(
        denied_graph
            .materialize_provenance_artifact(denied_dependent)
            .unwrap()
            .1,
        DiagnosticsAvailability::DeniedByBudget
    );
    let denied_metrics = denied_graph.observe().metrics();
    assert_eq!(
        denied_metrics.storage.denied_reconstruction_by_budget_count,
        2
    );
    assert_eq!(
        denied_metrics
            .storage
            .explicit_cold_materialization_request_count,
        2
    );
}

#[test]
fn artifact_materialization_availability_states_are_explicit_and_non_ambiguous() {
    let mut retained_graph = SignalGraph::new();
    let retained_source = retained_graph.node().build();
    let retained_dependent = retained_graph.node().build();
    retained_graph
        .append_dependency(retained_dependent, retained_source, ASPECT_A)
        .unwrap();
    retained_graph.set_runtime_policy(SignalRuntimePolicy::development());
    let retained_bootstrap = retained_graph
        .build_evaluation_plan(
            &[retained_source, retained_dependent],
            EvaluationRequestMode::ForceOnDemand,
        )
        .unwrap();
    retained_graph
        .execute_prepared_plan_with_precompute(&retained_bootstrap, &|node, view| {
            let result = if node == retained_source {
                view.finish(version_ab(1, 0))
            } else {
                let version = view.read_aspect_version(retained_source, ASPECT_A)?;
                view.finish(NodeEvaluationResult::from_version(version))
            };
            Ok(result)
        })
        .unwrap();
    let (retained_explanation, retained_mode) = retained_graph
        .materialize_explanation_artifact(retained_dependent)
        .unwrap();

    let mut reconstructed_graph = SignalGraph::new();
    let reconstructed_source = reconstructed_graph.node().build();
    let reconstructed_dependent = reconstructed_graph.node().build();
    reconstructed_graph
        .append_dependency(reconstructed_dependent, reconstructed_source, ASPECT_A)
        .unwrap();
    reconstructed_graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let mut reconstructed_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(
        &mut reconstructed_graph,
        reconstructed_source,
        &mut reconstructed_compute,
    )
    .unwrap();
    evaluate(
        &mut reconstructed_graph,
        reconstructed_dependent,
        &mut reconstructed_compute,
    )
    .unwrap();
    let (reconstructed_explanation, reconstructed_mode) = reconstructed_graph
        .materialize_explanation_artifact(reconstructed_dependent)
        .unwrap();

    let mut omitted_graph = SignalGraph::new();
    let omitted_source = omitted_graph.node().build();
    let omitted_dependent = omitted_graph.node().build();
    omitted_graph
        .append_dependency(omitted_dependent, omitted_source, ASPECT_A)
        .unwrap();
    omitted_graph.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit),
    );
    let mut omitted_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut omitted_graph, omitted_source, &mut omitted_compute).unwrap();
    evaluate(&mut omitted_graph, omitted_dependent, &mut omitted_compute).unwrap();
    let (omitted_explanation, omitted_mode) = omitted_graph
        .materialize_explanation_artifact(omitted_dependent)
        .unwrap();

    let mut denied_graph = SignalGraph::new();
    let denied_source = denied_graph.node().build();
    let denied_dependent = denied_graph.node().build();
    denied_graph
        .append_dependency(denied_dependent, denied_source, ASPECT_A)
        .unwrap();
    let mut denied_policy = SignalRuntimePolicy::operational();
    denied_policy
        .reconstruction_budget
        .allow_explanation_reconstruction = false;
    denied_graph.set_runtime_policy(denied_policy);
    let mut denied_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut denied_graph, denied_source, &mut denied_compute).unwrap();
    evaluate(&mut denied_graph, denied_dependent, &mut denied_compute).unwrap();
    let (denied_explanation, denied_mode) = denied_graph
        .materialize_explanation_artifact(denied_dependent)
        .unwrap();

    assert!(retained_explanation.is_some());
    assert!(reconstructed_explanation.is_some());
    assert!(omitted_explanation.is_none());
    assert!(denied_explanation.is_none());

    assert_eq!(retained_mode, DiagnosticsAvailability::RetainedAvailable);
    assert_eq!(
        reconstructed_mode,
        DiagnosticsAvailability::ReconstructedAvailable
    );
    assert_eq!(omitted_mode, DiagnosticsAvailability::OmittedByTier);
    assert_eq!(denied_mode, DiagnosticsAvailability::DeniedByBudget);

    assert!(retained_mode.is_available());
    assert!(reconstructed_mode.is_available());
    assert!(!omitted_mode.is_available());
    assert!(!denied_mode.is_available());
    assert!(!retained_mode.is_reconstructed());
    assert!(reconstructed_mode.is_reconstructed());
    assert!(!omitted_mode.is_reconstructed());
    assert!(!denied_mode.is_reconstructed());

    assert_ne!(retained_mode.message(), reconstructed_mode.message());
    assert_ne!(retained_mode.message(), omitted_mode.message());
    assert_ne!(retained_mode.message(), denied_mode.message());
    assert_ne!(reconstructed_mode.message(), omitted_mode.message());
    assert_ne!(reconstructed_mode.message(), denied_mode.message());
    assert_ne!(omitted_mode.message(), denied_mode.message());
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
            && matches!(
                link.kind,
                crate::logic::explain::CausalLinkKind::MissingSnapshot
            )
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
        reuse_origin: crate::data::reuse::ReuseOrigin::MemoizedArtifactReuse,
        reuse_basis: ReuseBasis::strategy(
            crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
            ReuseSource::MemoizedArtifact,
            ReuseCrossing::None,
        ),
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
            artifact_family: None,
            structural_dependency_basis: crate::data::dependency::DependencySnapshotId::EMPTY,
            partition_region_basis: PartitionScopeSet::default(),
            strategy_detail: crate::data::reuse::ReuseStrategyBoundaryContext::None,
        }),
        ..RuntimeArtifactState::default()
    }));
    entry.set_retained_diagnostic_artifact(Some(RetainedDiagnosticArtifact {
        changed_regions: CanonicalChangedRegions::default(),
        labels: Vec::new(),
        keyed_family: None,
        keyed_key: None,
        reuse_certification: Some(ReuseCertificationRecord {
            strategy: crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
            origin: crate::data::reuse::ReuseOrigin::MemoizedArtifactReuse,
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
        Some(ReuseBasis::strategy(
            crate::data::reuse::ReuseStrategy::MemoizedArtifactReuse,
            ReuseSource::MemoizedArtifact,
            ReuseCrossing::None,
        ))
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
fn ordinary_summary_surfaces_do_not_trigger_artifact_reconstruction() {
    let mut runtime = build_runtime(SignalGraph::new());
    runtime.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = runtime.graph_mut().node().build();
    let dependent = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(runtime.graph_mut(), source, &mut compute).unwrap();
    evaluate(runtime.graph_mut(), dependent, &mut compute).unwrap();

    let before = runtime
        .observe()
        .metrics()
        .storage
        .hot_path_artifact_reconstruction_count;

    let diagnostics = runtime.observe().diagnostics();
    let _graph_summary = diagnostics.summary(DiagnosticsTier::Operational);
    let history = diagnostics.history(DiagnosticsTier::Operational);
    let _recent = diagnostics.recent_history();
    let _replay = runtime.graph().replay_events();
    let rendered = render_execution_history_summary(&history);

    let after = runtime
        .observe()
        .metrics()
        .storage
        .hot_path_artifact_reconstruction_count;
    assert_eq!(
        before, after,
        "ordinary diagnostics/history/replay reads must not trigger artifact reconstruction"
    );
    assert!(rendered.contains("ExecutionHistorySummary"));
}

#[test]
fn tier_matrix_public_observer_surfaces_preserve_truth_while_availability_changes() {
    #[derive(Clone)]
    struct TierRun {
        summary: GraphSummary,
        history: ExecutionHistorySummary,
        flow: FlowSummary,
        replay: ReplaySlice,
        lineage: Vec<LineageRecord>,
        explanation: ExplanationSummary,
        explanation_availability: DiagnosticsAvailability,
        provenance_availability: DiagnosticsAvailability,
        ordinary_cold_requests: u64,
    }

    fn run(policy: SignalRuntimePolicy) -> TierRun {
        let mut graph = SignalGraph::new();
        graph.set_runtime_policy(policy);
        let source = graph.node().output_identity().build();
        let dependent = graph.node().build();
        graph
            .append_dependency(dependent, source, ASPECT_A)
            .unwrap();

        let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_output_identity("artifact-v1"))
        };
        let mut source_v2 = |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
                .with_output_identity("artifact-v2"))
        };
        let mut dependent_compute = |_id: NodeId, graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(
                graph.get_entry(source).unwrap().get_aspect_version(),
            ))
        };

        evaluate(&mut graph, source, &mut source_v1).unwrap();
        evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
        let snapshot = graph.capture_snapshot();

        mark_dirty(&mut graph, source, ASPECT_A).unwrap();
        evaluate(&mut graph, source, &mut source_v2).unwrap();
        evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
        graph
            .restore_snapshot_with_intent(
                &snapshot,
                SnapshotRestoreIntent::restore_runtime_truth_with_active_policy(),
            )
            .unwrap();

        let before_ordinary = graph
            .observe()
            .metrics()
            .storage
            .explicit_cold_materialization_request_count;
        let summary = graph.observe().diagnostics_summary(policy.tier);
        let history = graph.observe().execution_history_summary(policy.tier);
        let flow = graph
            .observe()
            .latest_flow_diagnostics()
            .expect("flow should exist after restore")
            .clone();
        let replay = graph
            .observe()
            .replay_around_snapshot(snapshot.snapshot_id())
            .to_owned_slice();
        let lineage = graph.observe().lineage_for_node(source).to_owned_records();
        let explanation = graph
            .observe()
            .explain(dependent)
            .unwrap()
            .diagnostics_summary(policy.tier);
        let after_ordinary = graph
            .observe()
            .metrics()
            .storage
            .explicit_cold_materialization_request_count;
        let ordinary_cold_requests = after_ordinary.saturating_sub(before_ordinary);

        let explanation_availability = graph.materialize_explanation_artifact(dependent).unwrap().1;
        let provenance_availability = graph.materialize_provenance_artifact(dependent).unwrap().1;

        TierRun {
            summary,
            history,
            flow,
            replay,
            lineage,
            explanation,
            explanation_availability,
            provenance_availability,
            ordinary_cold_requests,
        }
    }

    let operational = run(SignalRuntimePolicy::operational());
    let development = run(SignalRuntimePolicy::development());
    let forensic = run(SignalRuntimePolicy::forensic()
        .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::CompactGlobal));

    for (left, right) in [
        (&operational, &development),
        (&development, &forensic),
        (&operational, &forensic),
    ] {
        assert!(
            left.summary.active_node_count == right.summary.active_node_count
                && left.summary.clean_node_count == right.summary.clean_node_count
                && left.summary.maybe_stale_node_count == right.summary.maybe_stale_node_count
                && left.summary.dirty_node_count == right.summary.dirty_node_count
                && left.summary.dependency_edge_count == right.summary.dependency_edge_count
                && left.summary.subscriber_edge_count == right.summary.subscriber_edge_count
                && left.summary.nodes_with_causality == right.summary.nodes_with_causality,
            "graph summaries should preserve the same canonical graph truth across tier changes"
        );
        assert!(
            left.history.traced_node_count == right.history.traced_node_count
                && left.history.execution_record_count == right.history.execution_record_count
                && left.history.latest_execution_record_id
                    == right.history.latest_execution_record_id
                && left.history.reuse_origin_counts == right.history.reuse_origin_counts,
            "execution history should preserve the same conclusion set across tier changes"
        );
        assert!(
            left.flow.change == right.flow.change
                && left.flow.invalidation == right.flow.invalidation
                && left.flow.planning.plan.task_count == right.flow.planning.plan.task_count
                && left.flow.planning.plan.stage_count == right.flow.planning.plan.stage_count
                && left.flow.precompute.prepared_evaluations_produced
                    == right.flow.precompute.prepared_evaluations_produced
                && left.flow.apply.prepared_evaluations_applied
                    == right.flow.apply.prepared_evaluations_applied
                && left.flow.rollback == right.flow.rollback,
            "latest flow should preserve the same semantic truth across tier changes"
        );
        assert!(
            replay_slices_equivalent(&left.replay, &right.replay),
            "replay should remain semantically equivalent across tier changes"
        );
        assert!(
            lineage_records_equivalent(&left.lineage, &right.lineage),
            "lineage should remain semantically equivalent across tier changes"
        );
        assert!(
            left.explanation.node == right.explanation.node
                && left.explanation.state == right.explanation.state
                && left.explanation.upstream_count == right.explanation.upstream_count
                && left.explanation.changed_upstream_count
                    == right.explanation.changed_upstream_count
                && left.explanation.skipped_upstream_count
                    == right.explanation.skipped_upstream_count
                && left.explanation.condition_deferred_count
                    == right.explanation.condition_deferred_count
                && left.explanation.clean_upstream_count == right.explanation.clean_upstream_count
                && left.explanation.missing_snapshot_count
                    == right.explanation.missing_snapshot_count
                && left.explanation.dependency_removed_count
                    == right.explanation.dependency_removed_count
                && left.explanation.propagation_suppressed
                    == right.explanation.propagation_suppressed
                && left.explanation.output_change == right.explanation.output_change
                && left.explanation.memoized_origin == right.explanation.memoized_origin
                && left.explanation.reuse_basis == right.explanation.reuse_basis
                && left.explanation.reuse_origin == right.explanation.reuse_origin
                && left.explanation.contract_reads_mask == right.explanation.contract_reads_mask
                && left.explanation.contract_produces_mask
                    == right.explanation.contract_produces_mask
                && left.explanation.required_context == right.explanation.required_context,
            "explanations should preserve the same semantic truth across tier changes"
        );
        assert_eq!(
            left.ordinary_cold_requests, 0,
            "ordinary observer access must not trigger cold materialization"
        );
        assert_eq!(
            right.ordinary_cold_requests, 0,
            "ordinary observer access must not trigger cold materialization"
        );
    }

    assert_eq!(
        operational.explanation_availability,
        DiagnosticsAvailability::ReconstructedAvailable
    );
    assert_eq!(
        operational.provenance_availability,
        DiagnosticsAvailability::ReconstructedAvailable
    );
    assert_eq!(development.explanation_availability.is_available(), true);
    assert_eq!(development.provenance_availability.is_available(), true);
    assert_eq!(forensic.explanation_availability.is_available(), true);
    assert_eq!(forensic.provenance_availability.is_available(), true);
}

#[test]
fn ordinary_observer_access_never_increments_cold_or_denial_counters_across_tiers() {
    for tier in [
        DiagnosticsTier::Operational,
        DiagnosticsTier::Development,
        DiagnosticsTier::Forensic,
    ] {
        let mut runtime = build_runtime(SignalGraph::new());
        runtime.set_runtime_policy(SignalRuntimePolicy::for_tier(tier));
        let source = runtime.graph_mut().node().output_identity().build();
        let mut runtime_ctx = ();

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 0))
                            .with_output_identity(format!("ordinary-tier-{}", tier.label())),
                    ))
                })?;
                Ok(())
            })
            .unwrap();

        let branch = runtime.observe().current_branch();
        let metrics_before = runtime.observe().metrics().storage;

        let diagnostics = runtime.observe().diagnostics();
        let _summary = diagnostics.summary(tier);
        let _history = diagnostics.history(tier);
        let _recent = diagnostics.recent_history();
        let _latest_flow = runtime.observe().latest_flow_diagnostics();
        let _replay = runtime.observe().replay_for_branch(branch.id);
        let _lineage = runtime.observe().lineage_chain_for_node(source);
        let _explanation = runtime.observe().explain(source).unwrap();

        let metrics_after = runtime.observe().metrics().storage;
        assert_eq!(
            metrics_before.explicit_cold_materialization_request_count,
            metrics_after.explicit_cold_materialization_request_count,
            "ordinary observer access must not request cold materialization for tier {}",
            tier.label()
        );
        assert_eq!(
            metrics_before.cold_explanation_reconstruction_count,
            metrics_after.cold_explanation_reconstruction_count,
            "ordinary observer access must not reconstruct explanation artifacts for tier {}",
            tier.label()
        );
        assert_eq!(
            metrics_before.cold_provenance_reconstruction_count,
            metrics_after.cold_provenance_reconstruction_count,
            "ordinary observer access must not reconstruct provenance artifacts for tier {}",
            tier.label()
        );
        assert_eq!(
            metrics_before.reconstructed_artifact_read_count,
            metrics_after.reconstructed_artifact_read_count,
            "ordinary observer access must not record reconstructed artifact reads for tier {}",
            tier.label()
        );
        assert_eq!(
            metrics_before.denied_reconstruction_by_budget_count,
            metrics_after.denied_reconstruction_by_budget_count,
            "ordinary observer access must not produce budget denial counts for tier {}",
            tier.label()
        );
        assert_eq!(
            metrics_before.denied_reconstruction_by_tier_count,
            metrics_after.denied_reconstruction_by_tier_count,
            "ordinary observer access must not produce tier denial counts for tier {}",
            tier.label()
        );
        assert_eq!(
            metrics_before.denied_reconstruction_explanation_api_count,
            metrics_after.denied_reconstruction_explanation_api_count,
            "ordinary observer access must not increment explanation denial attribution for tier {}",
            tier.label()
        );
        assert_eq!(
            metrics_before.denied_reconstruction_provenance_api_count,
            metrics_after.denied_reconstruction_provenance_api_count,
            "ordinary observer access must not increment provenance denial attribution for tier {}",
            tier.label()
        );
    }
}

#[test]
fn branch_and_snapshot_churn_respect_retention_budget_under_all_tiers() {
    for policy in [
        SignalRuntimePolicy::operational()
            .with_history_limit(2)
            .with_detail_limit(1)
            .with_history_details(false),
        SignalRuntimePolicy::development()
            .with_history_limit(3)
            .with_detail_limit(2)
            .with_history_details(true),
        SignalRuntimePolicy::forensic()
            .with_history_limit(4)
            .with_detail_limit(3)
            .with_history_details(true),
    ] {
        let mut runtime = build_runtime(SignalGraph::new());
        runtime.set_runtime_policy(policy);
        let source = runtime.graph_mut().node().output_identity().build();
        let mut runtime_ctx = ();

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 0))
                            .with_output_identity(format!("main-seed-{}", policy.tier.label())),
                    ))
                })?;
                Ok(())
            })
            .unwrap();

        let main = runtime.observe().current_branch();
        let feature = runtime
            .create_branch(format!("feature-retention-{}", policy.tier.label()))
            .unwrap();
        let main_snapshot = runtime.capture_snapshot();

        runtime.switch_branch(feature.clone()).unwrap();
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_A)?;
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(2, 0))
                            .with_output_identity(format!("feature-seed-{}", policy.tier.label())),
                    ))
                })?;
                Ok(())
            })
            .unwrap();
        let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

        for cycle in 0..18 {
            let (branch, snapshot, output_identity) = if cycle % 2 == 0 {
                (
                    main.clone(),
                    &main_snapshot,
                    format!("main-cycle-{}-{cycle}", policy.tier.label()),
                )
            } else {
                (
                    feature.clone(),
                    &feature_snapshot,
                    format!("feature-cycle-{}-{cycle}", policy.tier.label()),
                )
            };
            runtime.switch_branch(branch.clone()).unwrap();
            runtime
                .restore_branch_snapshot(branch.clone(), snapshot)
                .unwrap();
            runtime
                .transaction(&mut runtime_ctx, |tx| {
                    tx.mark_dirty(source, ASPECT_A)?;
                    tx.read(source, &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(cycle + 3, 0))
                                .with_output_identity(output_identity.clone()),
                        ))
                    })?;
                    Ok(())
                })
                .unwrap();

            let diagnostics = runtime.observe().diagnostics();
            assert!(
                diagnostics.recent_history().len() <= policy.retention_budget.history_limit,
                "recent history must stay within retention budget for tier {}",
                policy.tier.label()
            );
            assert!(
                runtime.graph().replay_events().len()
                    <= policy.retention_budget.history_limit.max(1) * 32,
                "replay retention must stay bounded for tier {}",
                policy.tier.label()
            );
            assert!(
                runtime.graph().observe().lineage_records().len()
                    <= policy.retention_budget.history_limit.max(1) * 32,
                "lineage retention must stay bounded for tier {}",
                policy.tier.label()
            );
        }
    }
}

#[test]
fn ordinary_summary_and_history_rendering_respect_retained_detail_limits() {
    let policy = SignalRuntimePolicy::development()
        .with_history_limit(3)
        .with_detail_limit(1)
        .with_history_details(true);
    let mut runtime = build_runtime(SignalGraph::new());
    runtime.set_runtime_policy(policy);
    let source_a = runtime.graph_mut().node().output_identity().build();
    let source_b = runtime.graph_mut().node().output_identity().build();
    let source_c = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    for (index, node) in [source_a, source_b, source_c].into_iter().enumerate() {
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.read(node, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(index as u64 + 1, 0))
                            .with_output_identity(format!("render-bounded-{index}")),
                    ))
                })?;
                Ok(())
            })
            .unwrap();
    }

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source_a, ASPECT_A)?;
            tx.mark_dirty(source_b, ASPECT_A)?;
            Ok(())
        })
        .unwrap();

    let metrics_before = runtime.observe().metrics().storage;
    let diagnostics = runtime.observe().diagnostics();
    let summary = diagnostics.summary(DiagnosticsTier::Development);
    let history = diagnostics.history(DiagnosticsTier::Development);
    let rendered_summary = render_graph_summary(&summary);
    let rendered_history = render_execution_history_summary(&history);
    let metrics_after = runtime.observe().metrics().storage;

    assert!(summary.sample_dirty_nodes.len() <= policy.retention_budget.detail_limit);
    assert!(
        summary.sample_nodes_with_execution_record.len() <= policy.retention_budget.detail_limit
    );
    assert!(history.nodes.len() <= policy.retention_budget.detail_limit);
    assert!(rendered_summary.contains("GraphSummary"));
    assert!(rendered_history.contains("ExecutionHistorySummary"));
    assert_eq!(
        metrics_before.explicit_cold_materialization_request_count,
        metrics_after.explicit_cold_materialization_request_count,
        "ordinary rendering must not request cold materialization"
    );
    assert_eq!(
        metrics_before.cold_explanation_reconstruction_count,
        metrics_after.cold_explanation_reconstruction_count,
        "ordinary rendering must not reconstruct explanation artifacts"
    );
    assert_eq!(
        metrics_before.cold_provenance_reconstruction_count,
        metrics_after.cold_provenance_reconstruction_count,
        "ordinary rendering must not reconstruct provenance artifacts"
    );
}

#[test]
fn long_session_branch_churn_with_mixed_reads_keeps_bounds_and_cold_work_honest() {
    let policy = SignalRuntimePolicy::operational()
        .with_history_limit(2)
        .with_detail_limit(1)
        .with_history_details(false);
    let mut runtime = build_runtime(SignalGraph::new());
    runtime.set_runtime_policy(policy);
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("long-main"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-long-session").unwrap();
    let main_snapshot = runtime.capture_snapshot();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("long-feature"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    for cycle in 0..24 {
        let (branch, snapshot) = if cycle % 2 == 0 {
            (main.clone(), &main_snapshot)
        } else {
            (feature.clone(), &feature_snapshot)
        };
        runtime.switch_branch(branch.clone()).unwrap();
        runtime
            .restore_branch_snapshot(branch.clone(), snapshot)
            .unwrap();

        let before_ordinary = runtime
            .observe()
            .metrics()
            .storage
            .explicit_cold_materialization_request_count;
        let diagnostics = runtime.observe().diagnostics();
        let summary = diagnostics.summary(DiagnosticsTier::Operational);
        let history = diagnostics.history(DiagnosticsTier::Operational);
        let _recent = diagnostics.recent_history();
        let _replay = runtime.observe().replay_for_branch(branch.id);
        let _lineage = runtime.observe().lineage_chain_for_node(source);
        let rendered_history = render_execution_history_summary(&history);
        let rendered_summary = render_graph_summary(&summary);
        let after_ordinary = runtime
            .observe()
            .metrics()
            .storage
            .explicit_cold_materialization_request_count;
        assert_eq!(
            before_ordinary, after_ordinary,
            "ordinary diagnostics reads must stay zero-cold under long-session churn"
        );
        assert!(rendered_history.contains("ExecutionHistorySummary"));
        assert!(rendered_summary.contains("GraphSummary"));

        if cycle % 4 == 0 {
            let before_burst = runtime.observe().metrics().storage;
            let (explanation, explanation_mode) = runtime
                .observe()
                .materialize()
                .materialize_explanation_artifact(source)
                .unwrap();
            let (provenance, provenance_mode) = runtime
                .observe()
                .materialize()
                .materialize_provenance_artifact(source)
                .unwrap();
            let after_burst = runtime.observe().metrics().storage;
            assert!(explanation.is_some());
            assert!(provenance.is_some());
            assert_eq!(
                explanation_mode,
                DiagnosticsAvailability::ReconstructedAvailable
            );
            assert_eq!(
                provenance_mode,
                DiagnosticsAvailability::ReconstructedAvailable
            );
            assert_eq!(
                after_burst.explicit_cold_materialization_request_count
                    - before_burst.explicit_cold_materialization_request_count,
                2,
                "each explicit cold burst should record exactly two requests"
            );
            assert_eq!(
                after_burst.reconstructed_artifact_read_count
                    - before_burst.reconstructed_artifact_read_count,
                2,
                "each explicit cold burst should record exactly two reconstructed reads"
            );
            assert_eq!(
                after_burst.cold_explanation_reconstruction_count
                    - before_burst.cold_explanation_reconstruction_count,
                1
            );
            assert_eq!(
                after_burst.cold_provenance_reconstruction_count
                    - before_burst.cold_provenance_reconstruction_count,
                1
            );
        }
    }

    let metrics = runtime.observe().metrics();
    assert_eq!(metrics.storage.denied_reconstruction_by_budget_count, 0);
    assert_eq!(metrics.storage.denied_reconstruction_by_tier_count, 0);
    assert!(
        runtime
            .observe()
            .recent_execution_history_diagnostics()
            .len()
            <= policy.retention_budget.history_limit
    );
    assert!(
        runtime.graph().replay_events().len() <= policy.retention_budget.history_limit.max(1) * 32
    );
    assert!(
        runtime.graph().observe().lineage_records().len()
            <= policy.retention_budget.history_limit.max(1) * 32
    );
    assert_eq!(runtime.observe().known_branches().len(), 2);
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
fn diagnostics_access_exposes_frontier_execution_and_trace_records() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
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

    let diagnostics = graph.observe().diagnostics();
    let frontier = diagnostics
        .latest_frontier_execution()
        .expect("frontier execution should be available");
    assert_eq!(frontier.seed_count, 1);
    assert_eq!(frontier.direct_waves.len(), 1);
    assert!(frontier
        .direct_waves
        .iter()
        .flat_map(|wave| wave.entries.iter())
        .any(|entry| entry.node == dependent));
    assert!(!diagnostics.latest_invalidation_trace_records().is_empty());
}

#[test]
fn pending_invalidation_summary_is_retained_and_served_without_cold_work() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let before = graph
        .observe()
        .metrics()
        .storage
        .explicit_cold_materialization_request_count;
    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let pending = graph
        .diagnostics_state()
        .pending_graph_summary()
        .cloned()
        .expect("pending invalidation should retain a graph summary");
    let summary = graph
        .observe()
        .diagnostics_summary(DiagnosticsTier::Development);
    let after = graph
        .observe()
        .metrics()
        .storage
        .explicit_cold_materialization_request_count;

    assert_eq!(summary, pending.with_profile(DiagnosticsTier::Development));
    assert!(summary.dirty_node_count >= 1);
    assert_eq!(
        before, after,
        "ordinary dirty-state summary reads must not trigger cold materialization work"
    );
}

#[test]
fn operational_diagnostics_do_not_retain_frontier_trace_records_by_default() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
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

    let diagnostics = graph.observe().diagnostics();
    assert!(diagnostics.latest_frontier_execution().is_some());
    assert!(diagnostics.latest_invalidation_trace_records().is_empty());
}

#[test]
fn observer_reads_do_not_mutate_frontier_truth_or_retain_extra_trace_records() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
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

    let summary_before = graph
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("frontier execution summary should exist");
    let traces_before = graph.observe().latest_invalidation_trace_records().to_vec();
    let metrics_before = graph
        .observe()
        .metrics()
        .invalidation
        .frontier_trace_retained_count;

    let diagnostics = graph.observe().diagnostics();
    let summary_after = diagnostics
        .latest_frontier_execution()
        .cloned()
        .expect("frontier execution summary should remain available");
    let traces_after = diagnostics.latest_invalidation_trace_records().to_vec();
    let metrics_after = graph
        .observe()
        .metrics()
        .invalidation
        .frontier_trace_retained_count;

    assert_eq!(summary_before, summary_after);
    assert_eq!(traces_before, traces_after);
    assert_eq!(metrics_before, metrics_after);
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
    let summary = explanation.diagnostics_summary(DiagnosticsTier::Development);
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
    runtime.set_runtime_policy(SignalRuntimePolicy::development());

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

#[test]
fn operational_flow_diagnostics_do_not_sample_explanations_by_default() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let fillet = graph.node().build();
    graph
        .append_partition_detail_dependency(fillet, source, ASPECT_A, "surface", "fillet-band")
        .unwrap();
    let mut runtime = build_runtime(graph);
    runtime.set_runtime_policy(SignalRuntimePolicy::operational());

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
    assert!(
        flow.cause_samples.is_empty(),
        "operational flow diagnostics should not pay sampled explanation cost by default"
    );
}
