use forge_relational::facade::runtime::RelationalRuntimeApi;
use forge_signal::facade::{
    diagnostics_for_graph, mark_dirty, Aspect, AspectVersion, DependencyEdge, NodeEvaluationResult,
    RunMode, SignalGraph,
};

use crate::data::authority::DerivedTopologyReadBasis;
use crate::data::bootstrap::worth_bootstrap_schema_registry;
use crate::data::explanation::{
    explain_authority_trace, explain_derived_trace, explain_signal_trace, narrate_boundary_envelope,
};
use crate::data::seed::{seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase};
use crate::data::tracing::{
    WorthAuthorityTraceAnchor, WorthAuthorityTraceEvidence, WorthBoundaryEnvelope,
    WorthDecisionTrace, WorthDerivedTraceAnchor, WorthDerivedTraceEvidence, WorthIntegrityMarkers,
    WorthNamedCounter, WorthPerformanceAccounting, WorthSignalTraceAnchor,
    WorthSignalTraceEvidence, WorthTraceAvailability,
};

const SIGNAL_ASPECT: Aspect = Aspect::new(0);

#[test]
fn authority_trace_explanation_surfaces_commit_story_and_query_hints() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(
            worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
        )
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "authority-explanation",
        &WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 3 },
    )
    .expect("seed worth topology");
    let anchor = WorthAuthorityTraceAnchor::from_commit_results(
        verified.branch_id.clone(),
        &verified.commits,
    );
    let evidence = WorthAuthorityTraceEvidence::from_commit_results(
        verified.branch_id.clone(),
        &verified.commits,
    );

    let narrative = explain_authority_trace(&runtime, &anchor, Some(&evidence));

    assert_eq!(narrative.availability, WorthTraceAvailability::Present);
    assert!(narrative.headline.contains("Authority committed"));
    assert!(narrative.branch_head_matches_latest_commit);
    assert!(narrative.changed_record_count > 0);
    assert!(!narrative.changed_aspects.is_empty());
    assert_eq!(narrative.query_hints.len(), 2);
}

#[test]
fn derived_trace_explanation_reopens_snapshot_and_mentions_touched_worth_aspects() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(
            worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
        )
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "derived-explanation",
        &WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 3 },
    )
    .expect("seed worth topology");
    let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&verified.persisted_truth);
    let anchor = WorthDerivedTraceAnchor::from_read_basis(&read_basis);
    let evidence = WorthDerivedTraceEvidence {
        availability: WorthTraceAvailability::Present,
        invalidation_target_count: 3,
        fallback_classes: vec!["WholeViewRebuild".to_string()],
        equivalence_digest: Some("digest:test".to_string()),
    };
    let markers = WorthIntegrityMarkers::new(
        Some(read_basis.branch_id().clone()),
        read_basis.touched_aspects().clone(),
        Some(read_basis.authoritative_mutation_origin()),
        Some(read_basis.authority.truth_basis_identity.clone()),
        read_basis.precision_fallbacks.len(),
        read_basis.precision_budget_fallbacks.len(),
    );

    let narrative = explain_derived_trace(&runtime, &anchor, Some(&evidence), Some(&markers));

    assert_eq!(narrative.availability, WorthTraceAvailability::Present);
    assert!(narrative.headline.contains("Derived trace reopened"));
    assert!(narrative.entity_count > 0);
    assert!(narrative.relation_count > 0);
    assert!(!narrative.touched_aspects.is_empty());
    assert_eq!(
        narrative.fallback_classes,
        vec!["WholeViewRebuild".to_string()]
    );
    assert_eq!(narrative.equivalence_digest.as_deref(), Some("digest:test"));
}

#[test]
fn boundary_envelope_narration_uses_query_rooted_decision_trace() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(
            worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
        )
        .build();
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        "narrated-envelope",
        &WorthMilestoneOnePrimitiveCase::WireClosed { half_edge_count: 3 },
    )
    .expect("seed worth topology");
    let read_basis = DerivedTopologyReadBasis::from_persisted_truth(&verified.persisted_truth);
    let decision_trace = WorthDecisionTrace {
        authority_anchor: Some(WorthAuthorityTraceAnchor::from_commit_results(
            verified.branch_id.clone(),
            &verified.commits,
        )),
        bridge_anchor: None,
        derived_anchor: Some(WorthDerivedTraceAnchor::from_read_basis(&read_basis)),
        authority: Some(WorthAuthorityTraceEvidence::from_commit_results(
            verified.branch_id.clone(),
            &verified.commits,
        )),
        bridge: None,
        derived: Some(WorthDerivedTraceEvidence {
            availability: WorthTraceAvailability::Present,
            invalidation_target_count: 2,
            fallback_classes: Vec::new(),
            equivalence_digest: None,
        }),
        signal_anchor: None,
        signal: None,
    };
    let envelope = WorthBoundaryEnvelope::success(
        "ok",
        Vec::new(),
        decision_trace,
        WorthIntegrityMarkers::new(
            Some(read_basis.branch_id().clone()),
            read_basis.touched_aspects().clone(),
            Some(read_basis.authoritative_mutation_origin()),
            Some(read_basis.authority.truth_basis_identity.clone()),
            read_basis.precision_fallbacks.len(),
            read_basis.precision_budget_fallbacks.len(),
        ),
        WorthPerformanceAccounting::new([WorthNamedCounter::new("test.counter", 1)]),
    );

    let narrative = narrate_boundary_envelope(&runtime, None, None, &envelope);

    assert!(narrative.headline.contains("Authority committed"));
    assert!(narrative
        .causal_story
        .iter()
        .any(|line| line.contains("Truth Basis")));
    assert!(narrative.query_hints.len() >= 2);
}

#[test]
fn signal_trace_explanation_queries_node_replay_lineage_and_artifacts() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .set_dependencies(dependent, [DependencyEdge::new(source, SIGNAL_ASPECT)])
        .expect("signal dependency");

    let bootstrap = graph
        .build_evaluation_plan(&[source, dependent], RunMode::ForceOnDemand)
        .expect("bootstrap plan");
    graph
        .execute_prepared_plan(&bootstrap, &(), &|view| {
            let result = if view.node() == source {
                view.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(SIGNAL_ASPECT, 1u64)]),
                ))
            } else {
                let version = view.read_aspect_version(source, SIGNAL_ASPECT)?;
                view.finish(NodeEvaluationResult::from_version(version))
            };
            Ok(result)
        })
        .expect("bootstrap execution");

    mark_dirty(&mut graph, source, SIGNAL_ASPECT).expect("mark source dirty");
    let refresh = graph
        .build_evaluation_plan(&[source, dependent], RunMode::ForceOnDemand)
        .expect("refresh plan");
    graph
        .execute_prepared_plan(&refresh, &(), &|view| {
            let result = if view.node() == source {
                view.finish(NodeEvaluationResult::from_version(
                    AspectVersion::from_updates([(SIGNAL_ASPECT, 2u64)]),
                ))
            } else {
                let version = view.read_aspect_version(source, SIGNAL_ASPECT)?;
                view.finish(NodeEvaluationResult::from_version(version))
            };
            Ok(result)
        })
        .expect("refresh execution");

    let _diagnostics = diagnostics_for_graph(&graph);
    let anchor = WorthSignalTraceAnchor::from_graph(&graph, dependent).expect("signal anchor");
    let evidence =
        WorthSignalTraceEvidence::from_graph(&graph, dependent).expect("signal evidence");

    let narrative =
        explain_signal_trace(&graph, &anchor, Some(&evidence)).expect("signal narrative");

    assert_eq!(narrative.availability, WorthTraceAvailability::Present);
    assert!(narrative.headline.contains("Signal tracked node"));
    assert!(narrative.replay_event_count > 0);
    assert!(narrative.execution_record_id.is_some());
    assert!(narrative.lineage_artifact_id.is_some());
    assert!(narrative
        .story_lines
        .iter()
        .any(|line| line.heading == "Replay"));
    assert_eq!(narrative.query_hints.len(), 2);
}
