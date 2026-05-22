use forge_proof::TransitionOutcome;

use super::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDomainCapabilityTargetBinding,
};
use super::test_support::{admitted_plan_target, lower_runtime_target, ready, success};
use super::{
    materialize_graph_composition_capability_support_row,
    materialize_graph_composition_domain_invariant_denial, materialize_runtime_admission_decision,
    materialize_runtime_continuity_evidence, ForgeQueryAdmissionContributionAuthoring,
    ForgeQueryContinuityContributionAuthoring, ForgeQueryInvariantCapabilityContributionAuthoring,
};

#[test]
fn admission_runtime_materializer_builds_query_decisions() {
    let advisory = success(materialize_runtime_admission_decision(ready_admission(
        ForgeQueryAdmissionContributionAuthoring::advisory_at_stage(
            "spatial_arbitration",
            "spatial.arbitration.requires_clarification",
            "multiple candidates remain admissible",
        ),
    )));
    let violation = success(materialize_runtime_admission_decision(ready_admission(
        ForgeQueryAdmissionContributionAuthoring::violation_at_stage(
            "spatial_arbitration",
            "spatial.arbitration.invalid_target",
            "requested target violates spatial law",
        ),
    )));

    match advisory {
        crate::intent_admission::ForgeQueryIntentAdmissionDecision::Advisory(advisory) => {
            assert_eq!(advisory.family().as_str(), "authoritative-user-intent");
            assert_eq!(
                advisory.entrypoint().as_str(),
                "ForgeQueryRuntime::execute_intent"
            );
            assert_eq!(advisory.stage(), "spatial_arbitration");
        }
        other => panic!("expected advisory decision, got {other:?}"),
    }

    match violation {
        crate::intent_admission::ForgeQueryIntentAdmissionDecision::Violation(violation) => {
            assert_eq!(violation.family().as_str(), "authoritative-user-intent");
            assert_eq!(
                violation.entrypoint().as_str(),
                "ForgeQueryRuntime::execute_intent"
            );
            assert_eq!(violation.stage(), "spatial_arbitration");
        }
        other => panic!("expected violation decision, got {other:?}"),
    }
}

#[test]
fn admission_support_only_runtime_decision_is_denied() {
    let outcome = materialize_runtime_admission_decision(ready_admission(
        ForgeQueryAdmissionContributionAuthoring::support_only(
            "spatial.arbitration.support_only",
            "declaration remains support-scoped only",
        ),
    ));

    match outcome {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.kind(),
                super::ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture
            );
        }
        other => panic!("expected denied support-only decision materialization, got {other:?}"),
    }
}

#[test]
fn graph_capability_runtime_materializer_preserves_capability_semantics() {
    let row = success(materialize_graph_composition_capability_support_row(
        ready_invariant_capability(
            ForgeQueryInvariantCapabilityContributionAuthoring::graph_capability_gap(
                "graph.face_inner_loop_insertion",
                crate::runtime::ForgeQueryGraphCompositionCapabilityClass::TargetCombination,
                "graph.face_inner_loop_insertion",
                "topology substrate is unavailable",
            ),
        ),
    ));

    assert_eq!(row.capability_family(), "graph.face_inner_loop_insertion");
    assert_eq!(
        row.capability_class(),
        crate::runtime::ForgeQueryGraphCompositionCapabilityClass::TargetCombination
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
            super::ForgeQueryInvariantCapabilityContributionPayload::with_graph_capability(
                super::ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial,
                "spatial.non_manifold_edge_split",
                "registered as a runtime invariant",
                Some(super::ForgeQueryGraphCapabilityRuntimeSemantics::new(
                    "graph.face_inner_loop_insertion",
                    crate::runtime::ForgeQueryGraphCompositionCapabilityClass::TargetCombination,
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
            super::ForgeQueryInvariantCapabilityContributionPayload::with_graph_invariant_denial(
                super::ForgeQueryInvariantCapabilityContributionPosture::SupportSummary,
                "spatial.non_manifold_edge_split",
                "result would introduce non-manifold topology",
                Some(super::ForgeQueryGraphInvariantDenialRuntimeSemantics::new(
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
            super::ForgeQueryInvariantCapabilityContributionPayload::with_graph_invariant_denial(
                super::ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial,
                "spatial.non_manifold_edge_split",
                "result would introduce non-manifold topology",
                Some(super::ForgeQueryGraphInvariantDenialRuntimeSemantics::new(
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
fn continuity_runtime_materializer_builds_continuity_evidence() {
    let preserved = success(materialize_runtime_continuity_evidence(ready_continuity(
        ForgeQueryContinuityContributionAuthoring::preserved_rebind(
            "edge:12",
            "edge:14",
            "continuity.identity.preserved",
            "edge retarget keeps a single successor identity",
        ),
    )));
    let split = success(materialize_runtime_continuity_evidence(ready_continuity(
        ForgeQueryContinuityContributionAuthoring::split_successors(
            "edge:12",
            ["edge:14", "edge:15"],
            "continuity.identity.split",
            "edge split produces two descendant identities",
        ),
    )));

    assert_eq!(
        preserved.outcome_class(),
        crate::runtime::ForgeQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
    );
    assert_eq!(preserved.prior_authoritative_identity(), "edge:12");
    assert_eq!(
        preserved.basis_binding_digest(),
        Some(
            ForgeQueryAdmittedPlanBoundContributionTarget::from_digest("plan-continuity")
                .binding_digest()
        )
    );
    assert_eq!(
        preserved.successor_authoritative_identity(),
        Some("edge:14")
    );

    assert_eq!(
        split.outcome_class(),
        crate::runtime::ForgeQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
    );
    assert_eq!(
        split.successor_authoritative_identities(),
        &["edge:14".to_string(), "edge:15".to_string()]
    );
}

#[test]
fn continuity_runtime_materializer_denies_correspondence_only() {
    let outcome = materialize_runtime_continuity_evidence(ready_continuity(
        ForgeQueryContinuityContributionAuthoring::correspondence_only(
            "continuity.correspondence_only",
            "semantic correspondence exists without authoritative continuity",
        ),
    ));

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics
    ));
}

#[test]
fn continuity_runtime_materializer_denies_inconsistent_runtime_semantics() {
    let outcome = materialize_runtime_continuity_evidence(ready_continuity_payload(
        super::ForgeQueryContinuityContributionPayload::with_runtime_semantics(
            super::ForgeQueryContinuityContributionPosture::Preserved,
            "continuity.identity.preserved",
            "posture claims preserved while runtime semantics claim split",
            Some(super::ForgeQueryContinuityRuntimeSemantics::new(
                crate::runtime::ForgeQueryContinuityMutationFamily::SplitExistingTarget,
                crate::runtime::ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors,
                "edge:12",
                ["edge:14", "edge:15"],
            )),
        ),
    ));

    assert!(matches!(
        outcome,
        TransitionOutcome::Denied(denial)
            if denial.kind()
                == super::ForgeQueryDomainCapabilityProgressionDenialKind::InconsistentCanonicalMaterializationSemantics
    ));
}

fn ready_admission(
    authoring: ForgeQueryAdmissionContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyAdmissionContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready(authoring.bind_to_admitted_plan_target(admitted_plan_target("plan-admission")))
}

fn ready_invariant_capability(
    authoring: ForgeQueryInvariantCapabilityContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyInvariantCapabilityContribution<
    super::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    ready(authoring.bind_to_lower_runtime_boundary_target(lower_runtime_target("boundary-graph")))
}

fn ready_invariant_payload(
    payload: super::ForgeQueryInvariantCapabilityContributionPayload,
) -> super::ForgeQueryMaterializationReadyInvariantCapabilityContribution<
    super::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    ready(
        super::proof_integration::create_requested_domain_capability_contribution(
            lower_runtime_target("boundary-graph"),
            payload,
        ),
    )
}

fn ready_continuity(
    authoring: ForgeQueryContinuityContributionAuthoring,
) -> super::ForgeQueryMaterializationReadyContinuityContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready_continuity_payload(
        authoring
            .bind_to_admitted_plan_target(admitted_plan_target("plan-continuity"))
            .payload()
            .payload()
            .clone(),
    )
}

fn ready_continuity_payload(
    payload: super::ForgeQueryContinuityContributionPayload,
) -> super::ForgeQueryMaterializationReadyContinuityContribution<
    ForgeQueryAdmittedPlanBoundContributionTarget,
> {
    ready(
        super::proof_integration::create_requested_domain_capability_contribution(
            admitted_plan_target("plan-continuity"),
            payload,
        ),
    )
}
