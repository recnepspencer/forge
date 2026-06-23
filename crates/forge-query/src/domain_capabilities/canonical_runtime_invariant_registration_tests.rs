use forge_proof::TransitionOutcome;
use forge_relational::facade::runtime::{InvariantCatalog, InvariantRegistration, InvariantRule};

use super::test_support::{
    declaration_target, lower_runtime_target, ready, ready_payload, success,
};
use super::{
    materialize_graph_composition_capability_support_row,
    materialize_graph_composition_domain_invariant_denial,
    materialize_query_invariant_catalog_registration_artifact,
    ForgeQueryGraphCapabilityRuntimeSemantics, ForgeQueryGraphInvariantDenialRuntimeSemantics,
    ForgeQueryInvariantCapabilityContributionAuthoring,
    ForgeQueryInvariantCapabilityContributionPayload,
    ForgeQueryInvariantCapabilityContributionPosture,
    ForgeQueryInvariantRegistrationRuntimeSemantics,
};
use crate::runtime::ForgeQueryGraphCompositionCapabilityClass;

#[test]
fn graph_capability_runtime_materializer_preserves_capability_semantics() {
    let row = success(materialize_graph_composition_capability_support_row(
        ready_invariant_capability(
            ForgeQueryInvariantCapabilityContributionAuthoring::graph_capability_gap(
                "graph.face_inner_loop_insertion",
                ForgeQueryGraphCompositionCapabilityClass::TargetCombination,
                "graph.face_inner_loop_insertion",
                "topology substrate is unavailable",
            ),
        ),
    ));

    assert_eq!(row.capability_family(), "graph.face_inner_loop_insertion");
    assert_eq!(
        row.capability_class(),
        ForgeQueryGraphCompositionCapabilityClass::TargetCombination
    );
}

#[test]
fn graph_capability_runtime_materializer_denies_missing_or_unsupported_semantics() {
    let missing = materialize_graph_composition_capability_support_row(ready_invariant_capability(
        ForgeQueryInvariantCapabilityContributionAuthoring::capability_gap(
            "graph.face_inner_loop_insertion",
            "topology substrate is unavailable",
        ),
    ));
    let unsupported =
        materialize_graph_composition_capability_support_row(ready_invariant_payload(
            ForgeQueryInvariantCapabilityContributionPayload::with_graph_capability(
                ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial,
                "spatial.non_manifold_edge_split",
                "registered as a runtime invariant",
                Some(ForgeQueryGraphCapabilityRuntimeSemantics::new(
                    "graph.face_inner_loop_insertion",
                    ForgeQueryGraphCompositionCapabilityClass::TargetCombination,
                )),
            ),
        ));

    assert!(matches!(
        missing,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
    assert!(matches!(
        unsupported,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture
    ));
}

#[test]
fn graph_invariant_denial_runtime_materializer_builds_query_denial() {
    let denial = success(materialize_graph_composition_domain_invariant_denial(
        ready_invariant_capability(
            ForgeQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
                "spatial.non_manifold_edge_split",
                ["edges", "faces"],
                ["edge:12"],
                ["mixed_existing_and_symbolic_entity_identity_edges"],
                ["mixed_existing_target_followup_mutation"],
                "program-graph-1",
                "breadth-graph-1",
                "components=3;symbolic_entities=1;symbolic_relations=0;declared_collections=2;declared_symbols=1;target_combinations=1;lifecycle_families=1",
                "spatial.non_manifold_edge_split",
                "result would introduce non-manifold topology",
            ),
        ),
    ));

    assert_eq!(denial.invariant_family(), "spatial.non_manifold_edge_split");
    assert_eq!(
        denial.domain_invariant_summary().declared_collections(),
        &["edges".to_string(), "faces".to_string()]
    );
    assert_eq!(
        denial.failure_stage(),
        crate::runtime::ForgeQueryGraphCompositionAdmissionTraceStage::DomainInvariantEvaluated
    );
}

#[test]
fn graph_invariant_denial_runtime_materializer_denies_missing_or_wrong_posture() {
    let missing =
        materialize_graph_composition_domain_invariant_denial(ready_invariant_capability(
            ForgeQueryInvariantCapabilityContributionAuthoring::invariant_denial(
                "spatial.non_manifold_edge_split",
                "result would introduce non-manifold topology",
            ),
        ));
    let wrong_posture = materialize_graph_composition_domain_invariant_denial(
        ready_invariant_payload(
            ForgeQueryInvariantCapabilityContributionPayload::with_graph_invariant_denial(
                ForgeQueryInvariantCapabilityContributionPosture::SupportSummary,
                "spatial.non_manifold_edge_split",
                "result would introduce non-manifold topology",
                Some(ForgeQueryGraphInvariantDenialRuntimeSemantics::new(
                    "spatial.non_manifold_edge_split",
                    ["edges"],
                    ["edge:12"],
                    ["mixed_existing_and_symbolic_entity_identity_edges"],
                    ["mixed_existing_target_followup_mutation"],
                    "program-graph-1",
                    "breadth-graph-1",
                    "components=2;symbolic_entities=1;symbolic_relations=0;declared_collections=1;declared_symbols=1;target_combinations=1;lifecycle_families=1",
                )),
            ),
        ),
    );

    assert!(matches!(
        missing,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
    assert!(matches!(
        wrong_posture,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture
    ));
}

#[test]
fn graph_invariant_denial_runtime_materializer_preserves_parity_and_difference() {
    let left = success(materialize_graph_composition_domain_invariant_denial(
        ready_invariant_capability(
            ForgeQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
                "spatial.non_manifold_edge_split",
                ["edges", "faces"],
                ["edge:12"],
                ["mixed_existing_and_symbolic_entity_identity_edges"],
                ["mixed_existing_target_followup_mutation"],
                "program-graph-1",
                "breadth-graph-1",
                "components=3;symbolic_entities=1;symbolic_relations=0;declared_collections=2;declared_symbols=1;target_combinations=1;lifecycle_families=1",
                "spatial.non_manifold_edge_split",
                "result would introduce non-manifold topology",
            ),
        ),
    ));
    let right = success(materialize_graph_composition_domain_invariant_denial(
        ready_invariant_payload(
            ForgeQueryInvariantCapabilityContributionPayload::with_graph_invariant_denial(
                ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial,
                "spatial.non_manifold_edge_split",
                "result would introduce non-manifold topology",
                Some(ForgeQueryGraphInvariantDenialRuntimeSemantics::new(
                    "spatial.non_manifold_edge_split",
                    ["edges", "faces"],
                    ["edge:12"],
                    ["mixed_existing_and_symbolic_entity_identity_edges"],
                    ["mixed_existing_target_followup_mutation"],
                    "program-graph-1",
                    "breadth-graph-1",
                    "components=3;symbolic_entities=1;symbolic_relations=0;declared_collections=2;declared_symbols=1;target_combinations=1;lifecycle_families=1",
                )),
            ),
        ),
    ));
    let different = success(materialize_graph_composition_domain_invariant_denial(
        ready_invariant_capability(
            ForgeQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
                "spatial.closed_loop_self_intersection",
                ["edges", "faces"],
                ["edge:12"],
                ["mixed_existing_and_symbolic_entity_identity_edges"],
                ["mixed_existing_target_followup_mutation"],
                "program-graph-2",
                "breadth-graph-1",
                "components=3;symbolic_entities=1;symbolic_relations=0;declared_collections=2;declared_symbols=1;target_combinations=1;lifecycle_families=1",
                "spatial.closed_loop_self_intersection",
                "result would introduce a self-intersecting loop",
            ),
        ),
    ));

    assert_eq!(left.denial_digest(), right.denial_digest());
    assert_ne!(left.denial_digest(), different.denial_digest());
}

#[test]
fn invariant_registration_materializer_builds_query_registration_artifact() {
    let artifact = success(materialize_query_invariant_catalog_registration_artifact(
        ready_invariant_registration(
            ForgeQueryInvariantCapabilityContributionAuthoring::invariant_rule_registration(
                InvariantRegistration::commit_boundary_blocking(InvariantRule::MaxMergedIntents(2)),
                "runtime.invariant.catalog_registration",
                "register merged-intent invariant at declaration scope",
            ),
        ),
    ));

    assert_eq!(artifact.lane(), "query_invariant_catalog_registration");
    assert_eq!(
        artifact.semantic_code(),
        "runtime.invariant.catalog_registration"
    );
    assert_eq!(artifact.invariant_catalog().registrations.len(), 1);
    assert_eq!(
        artifact.invariant_catalog().registrations[0],
        InvariantRegistration::commit_boundary_blocking(InvariantRule::MaxMergedIntents(2))
    );
    assert_eq!(
        artifact.detail(),
        "register merged-intent invariant at declaration scope"
    );
    assert_eq!(artifact.intent_name(), "domain-capability.intent-invariant");
    assert_eq!(
        artifact.strategy_name(),
        "forge.domain_capability.intent-invariant"
    );
}

#[test]
fn invariant_registration_materializer_denies_wrong_posture() {
    let wrong_posture = materialize_query_invariant_catalog_registration_artifact(
        ready_invariant_registration_payload(
            ForgeQueryInvariantCapabilityContributionPayload::with_invariant_registration(
                ForgeQueryInvariantCapabilityContributionPosture::SupportSummary,
                "runtime.invariant.catalog_registration",
                "registration semantics on the wrong posture should deny",
                Some(
                    ForgeQueryInvariantRegistrationRuntimeSemantics::from_registration(
                        InvariantRegistration::commit_boundary_blocking(
                            InvariantRule::MaxMergedIntents(2),
                        ),
                    ),
                ),
            ),
        ),
    );

    assert!(matches!(
        wrong_posture,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture
    ));
}

#[test]
fn invariant_registration_materializer_denies_missing_runtime_semantics() {
    let missing =
        materialize_query_invariant_catalog_registration_artifact(ready_invariant_registration(
            ForgeQueryInvariantCapabilityContributionAuthoring::support_summary(
                "runtime.invariant.catalog_registration",
                "registration semantics are absent here",
            ),
        ));

    assert!(matches!(
        missing,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
}

#[test]
fn invariant_registration_materializer_accepts_empty_native_catalog() {
    let artifact = success(materialize_query_invariant_catalog_registration_artifact(
        ready_invariant_registration(
            ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(
                InvariantCatalog {
                    registrations: vec![],
                },
                "runtime.invariant.catalog_registration",
                "empty catalog still carries explicit registration semantics",
            ),
        ),
    ));

    assert!(artifact.invariant_catalog().registrations.is_empty());
}

#[test]
fn invariant_registration_materializer_preserves_parity_across_catalog_ordering() {
    let left = success(materialize_query_invariant_catalog_registration_artifact(
        ready_invariant_registration(
            ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(
                registration_catalog([
                    InvariantRegistration::commit_boundary_blocking(
                        InvariantRule::MaxMergedIntents(2),
                    ),
                    InvariantRegistration::snapshot_publication_blocking(
                        InvariantRule::MaxSnapshotEntities(4),
                    ),
                ]),
                "runtime.invariant.catalog_registration",
                "register two native invariants",
            ),
        ),
    ));
    let right = success(materialize_query_invariant_catalog_registration_artifact(
        ready_invariant_registration(
            ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(
                registration_catalog([
                    InvariantRegistration::snapshot_publication_blocking(
                        InvariantRule::MaxSnapshotEntities(4),
                    ),
                    InvariantRegistration::commit_boundary_blocking(
                        InvariantRule::MaxMergedIntents(2),
                    ),
                ]),
                "runtime.invariant.catalog_registration",
                "register two native invariants",
            ),
        ),
    ));
    let different = success(materialize_query_invariant_catalog_registration_artifact(
        ready_invariant_registration(
            ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(
                registration_catalog([
                    InvariantRegistration::commit_boundary_blocking(
                        InvariantRule::MaxMergedIntents(3),
                    ),
                    InvariantRegistration::snapshot_publication_blocking(
                        InvariantRule::MaxSnapshotEntities(4),
                    ),
                ]),
                "runtime.invariant.catalog_registration",
                "register two native invariants",
            ),
        ),
    ));

    assert_eq!(
        left.materialization_digest(),
        right.materialization_digest()
    );
    assert_eq!(left, right);
    assert_ne!(
        left.materialization_digest(),
        different.materialization_digest()
    );
    assert_ne!(left, different);
}

fn registration_catalog(
    registrations: impl IntoIterator<Item = InvariantRegistration>,
) -> InvariantCatalog {
    InvariantCatalog {
        registrations: registrations.into_iter().collect(),
    }
}

fn ready_invariant_capability(
    authoring: ForgeQueryInvariantCapabilityContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyInvariantCapabilityContribution<
    super::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    ready(authoring.bind_to_lower_runtime_boundary_target(lower_runtime_target("boundary-graph")))
}

fn ready_invariant_payload(
    payload: ForgeQueryInvariantCapabilityContributionPayload,
) -> super::ForgeQueryMaterializationReadyInvariantCapabilityContribution<
    super::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    ready_payload(lower_runtime_target("boundary-graph"), payload)
}

fn ready_invariant_registration(
    authoring: ForgeQueryInvariantCapabilityContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyInvariantCapabilityContribution<
    super::ForgeQueryDeclarationBoundContributionTarget,
> {
    ready(authoring.bind_to_declaration_target(declaration_target("intent-invariant")))
}

fn ready_invariant_registration_payload(
    payload: ForgeQueryInvariantCapabilityContributionPayload,
) -> super::ForgeQueryMaterializationReadyInvariantCapabilityContribution<
    super::ForgeQueryDeclarationBoundContributionTarget,
> {
    ready_payload(declaration_target("intent-invariant"), payload)
}
