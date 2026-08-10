use super::admission::{resolve_effect_reuse_boundary, retains_reuse_boundary_detail};
use super::telemetry::record_reuse_rejection_telemetry;
use crate::data::comparator::DefaultComparatorPolicyResolver;
use crate::data::graph::SignalGraph;
use crate::data::output::NodeEvaluationResult;
use crate::data::proof::PartitionScopeSet;
use crate::data::reuse::{
    ArtifactSemanticBoundary, PersistentCorrespondenceEvidence, ReuseBoundaryFailure, ReuseStrategy,
};
use crate::diagnostics::policy::ArtifactRetentionPolicy;
use crate::logic::prepared::PreparedKeyedContext;
use crate::tests::support::version_ab;

#[test]
fn typed_reuse_rejection_telemetry_maps_to_canonical_counters() {
    let mut graph = SignalGraph::new();

    record_reuse_rejection_telemetry(
        &mut graph,
        &ReuseBoundaryFailure::PersistentCorrespondenceEvidenceMissing,
    );
    record_reuse_rejection_telemetry(
        &mut graph,
        &ReuseBoundaryFailure::CompositionRegionLegalityFailure,
    );
    record_reuse_rejection_telemetry(
        &mut graph,
        &ReuseBoundaryFailure::BoundaryContextUnavailable(ArtifactSemanticBoundary::TopologyRegime),
    );

    let evaluation = &graph.telemetry().evaluation;
    assert_eq!(
        evaluation.reuse_rejected_persistent_correspondence_missing_count,
        1
    );
    assert_eq!(evaluation.reuse_rejected_composition_region_count, 1);
    assert_eq!(evaluation.reuse_rejected_missing_prior_context_count, 1);
}

#[test]
fn memoized_reuse_resolves_authority_without_rich_detail() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph.set_runtime_policy(
        crate::facade::SignalRuntimePolicy::development()
            .with_explanation_retention(ArtifactRetentionPolicy::Retain)
            .with_provenance_retention(ArtifactRetentionPolicy::Retain),
    );

    let comparator_resolver = DefaultComparatorPolicyResolver::default();
    let (authority, detail) = resolve_effect_reuse_boundary(
        &graph,
        node,
        &comparator_resolver,
        Some(&NodeEvaluationResult::from_version(version_ab(1, 0))),
        None,
        Some(ReuseStrategy::MemoizedArtifactReuse),
        None,
    )
    .expect("authority-only resolution should succeed");

    assert!(detail.is_none());
    assert_eq!(authority.topology_regime, 0);
}

#[test]
fn cross_identity_reuse_retains_rich_boundary_detail_when_policy_requires_it() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    graph.set_runtime_policy(crate::facade::SignalRuntimePolicy::development());
    let comparator_resolver = DefaultComparatorPolicyResolver::default();
    let keyed = PreparedKeyedContext {
        persistent_correspondence: Some(PersistentCorrespondenceEvidence::lineage_backed_mapping(
            "lineage-map:left->right",
        )),
        composition_regions: PartitionScopeSet::default(),
        ..PreparedKeyedContext::default()
    };

    let (authority, detail) = resolve_effect_reuse_boundary(
        &graph,
        node,
        &comparator_resolver,
        Some(&NodeEvaluationResult::from_version(version_ab(1, 0))),
        Some(&keyed),
        Some(ReuseStrategy::CrossIdentityPersistentMatch),
        None,
    )
    .expect("cross-identity boundary resolution should succeed");

    assert!(detail.is_some());
    assert!(authority.persistent_correspondence_kind().is_some());
}

#[test]
fn reuse_boundary_detail_retention_is_strategy_and_policy_gated() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(
        crate::facade::SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit)
            .with_provenance_retention(ArtifactRetentionPolicy::Omit),
    );

    assert!(!retains_reuse_boundary_detail(
        &graph,
        Some(ReuseStrategy::CrossIdentityPersistentMatch)
    ));

    graph.set_runtime_policy(crate::facade::SignalRuntimePolicy::development());
    assert!(retains_reuse_boundary_detail(
        &graph,
        Some(ReuseStrategy::PartialArtifactSplicing)
    ));
    assert!(!retains_reuse_boundary_detail(
        &graph,
        Some(ReuseStrategy::MemoizedArtifactReuse)
    ));
}
