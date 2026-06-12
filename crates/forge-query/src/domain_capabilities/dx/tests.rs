use serde_json::json;

use super::{
    forge_query_domain, ForgeQueryDomainCapabilityMaterializationError,
    ForgeQueryDomainCapabilityOutcomeKind,
};
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution, materialize_canonical_admission_artifact,
    materialize_intent_declaration_support_traceability_artifact,
    materialize_query_invariant_catalog_registration_artifact,
    materialize_query_preview_workflow_artifact, materialize_query_workflow_declaration,
    materialize_runtime_admission_decision, materialize_runtime_continuity_evidence,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryAdmissionContributionAuthoring, ForgeQueryContinuityContributionAuthoring,
    ForgeQueryInvariantCapabilityContributionAuthoring, ForgeQuerySupportContributionAuthoring,
    ForgeQueryWorkflowContributionAuthoring,
};
use crate::runtime::{ForgeQueryIntentDeclaration, InvariantCatalog};

#[test]
fn common_intent_support_lane_matches_proof_lane_materialization() {
    let declaration = intent_declaration("intent-support");

    let common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_capability("graph.face_inner_loop_insertion")
        .because("topology substrate is available")
        .materialize()
        .expect("common support lane should materialize");

    let proof_requested = ForgeQuerySupportContributionAuthoring::declaration_support(
        "worth.spatial.graph.face_inner_loop_insertion",
        "topology substrate is available",
    )
    .for_intent_declaration(&declaration);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_intent_declaration_support_traceability_artifact(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common, success(proof));
}

#[test]
fn common_intent_advisory_lane_matches_proof_lane_materialization() {
    let declaration = intent_declaration("intent-advisory");

    let common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .advises("arbitration.requires_clarification")
        .because("multiple spatial candidates remain admissible")
        .materialize()
        .expect("declaration-bound advisory lane should materialize");

    let proof_requested = ForgeQueryAdmissionContributionAuthoring::advisory(
        "worth.spatial.arbitration.requires_clarification",
        "multiple spatial candidates remain admissible",
    )
    .for_intent_declaration(&declaration);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_canonical_admission_artifact(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common.canonical_family(), proof.canonical_family());
    assert_eq!(
        common.semantic_identity_digest(),
        proof.semantic_identity_digest()
    );
    assert_eq!(
        common.materialization_digest(),
        proof.materialization_digest()
    );
}

#[test]
fn common_admission_lane_matches_proof_lane_materialization() {
    let plan = admitted_basis_observation_plan();

    let common = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .advises("arbitration.requires_clarification")
        .because("multiple spatial candidates remain admissible")
        .materialize()
        .expect("common advisory lane should materialize");

    let proof_requested = ForgeQueryAdmissionContributionAuthoring::advisory(
        "worth.spatial.arbitration.requires_clarification",
        "multiple spatial candidates remain admissible",
    )
    .for_admitted_intent_plan(&plan);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_runtime_admission_decision(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common, success(proof));
}

#[test]
fn common_intent_preview_inspection_lane_matches_proof_lane_materialization() {
    let declaration = intent_declaration("intent-preview");

    let common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .inspects_query_preview(
            "topology.preview_conflict",
            crate::facade::runtime::BridgePreviewSessionIdentity::from_stable_name(
                "preview-session:42",
            ),
        )
        .because("preview should stay read-only while topology is inspected")
        .materialize()
        .expect("preview inspection lane should materialize");

    let proof_requested = ForgeQueryWorkflowContributionAuthoring::preview_only_query_inspection(
        "worth.spatial.topology.preview_conflict",
        "preview should stay read-only while topology is inspected",
        crate::facade::runtime::BridgePreviewSessionIdentity::from_stable_name(
            "preview-session:42",
        ),
    )
    .for_intent_declaration(&declaration);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_query_preview_workflow_artifact(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common, success(proof));
}

#[test]
fn common_intent_preview_mutation_lane_matches_proof_lane_materialization() {
    let declaration = intent_declaration("intent-preview-mutation");

    let common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .plans_preview_mutation(
            "topology.preview_mutation",
            crate::facade::runtime::BridgePreviewSessionIdentity::from_stable_name(
                "preview-session:77",
            ),
        )
        .because("promotion-eligible preview can plan a bounded mutation workflow")
        .materialize()
        .expect("preview mutation lane should materialize");

    let proof_requested =
        ForgeQueryWorkflowContributionAuthoring::promotion_eligible_mutation_lowering(
            "worth.spatial.topology.preview_mutation",
            "promotion-eligible preview can plan a bounded mutation workflow",
            crate::facade::runtime::BridgePreviewSessionIdentity::from_stable_name(
                "preview-session:77",
            ),
        )
        .for_intent_declaration(&declaration);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_query_workflow_declaration(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common.workflow_declaration(), &success(proof));
}

#[test]
fn common_continuity_lane_matches_proof_lane_materialization() {
    let plan = admitted_basis_observation_plan();

    let common = forge_query_domain("worth.spatial")
        .for_admitted_intent_plan(&plan)
        .preserves_continuity("identity.edge_split", "edge:before", "edge:after")
        .because("edge split replaces one edge with one canonical successor")
        .materialize()
        .expect("common continuity lane should materialize");

    let proof_requested = ForgeQueryContinuityContributionAuthoring::preserved_rebind(
        "edge:before",
        "edge:after",
        "worth.spatial.identity.edge_split",
        "edge split replaces one edge with one canonical successor",
    )
    .for_admitted_intent_plan(&plan);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_runtime_continuity_evidence(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common, success(proof));
}

#[test]
fn common_invariant_registration_lane_matches_proof_lane_materialization() {
    let declaration = intent_declaration("intent-invariant");
    let invariant_catalog = InvariantCatalog::default();

    let common = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .register_invariant_catalog("spatial.non_manifold_edge_split", invariant_catalog.clone())
        .because("geometry kernel must reject non-manifold edge splits")
        .materialize()
        .expect("common invariant lane should materialize");

    let proof_requested =
        ForgeQueryInvariantCapabilityContributionAuthoring::invariant_registration(
            invariant_catalog,
            "worth.spatial.spatial.non_manifold_edge_split",
            "geometry kernel must reject non-manifold edge splits",
        )
        .for_intent_declaration(&declaration);
    let proof_admitted = success(admit_eligible_domain_capability_contribution(success(
        evaluate_requested_domain_capability_contribution(proof_requested),
    )));
    let proof_target = proof_admitted.payload().target().clone();
    let proof = materialize_query_invariant_catalog_registration_artifact(success(
        prepare_admitted_domain_capability_contribution_for_materialization(
            proof_admitted,
            proof_target,
        ),
    ));

    assert_eq!(common, success(proof));
}

#[test]
fn checked_lane_preserves_denied_outcome_metadata_and_typed_error() {
    let declaration = intent_declaration("intent-denial");

    let checked = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_capability("graph.face_inner_loop_insertion")
        .because("")
        .try_materialize();

    assert_eq!(
        checked.kind(),
        ForgeQueryDomainCapabilityOutcomeKind::Denied
    );
    assert_eq!(checked.category(), "support-traceability");
    assert_eq!(checked.semantic_posture(), "declaration-support");
    assert_eq!(
        checked.target_kind(),
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::IntentDeclaration
    );
    assert!(checked.denial().is_some());

    let error = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .supports_capability("graph.face_inner_loop_insertion")
        .because("")
        .materialize()
        .expect_err("empty detail should deny");
    assert!(matches!(
        error,
        ForgeQueryDomainCapabilityMaterializationError::Denied(_)
    ));
}

#[test]
fn checked_intent_admission_lane_preserves_declaration_bound_metadata() {
    let declaration = intent_declaration("intent-admission-denial");

    let checked = forge_query_domain("worth.spatial")
        .for_intent(&declaration)
        .advises("arbitration.requires_clarification")
        .because("")
        .try_materialize();

    assert_eq!(
        checked.kind(),
        ForgeQueryDomainCapabilityOutcomeKind::Denied
    );
    assert_eq!(checked.category(), "admission");
    assert_eq!(checked.semantic_posture(), "advisory");
    assert_eq!(
        checked.target_kind(),
        crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind::IntentDeclaration
    );
    assert!(checked.denial().is_some());
    assert!(matches!(
        checked.transition_outcome(),
        forge_proof::TransitionOutcome::Denied(_)
    ));
}

fn success<T>(
    outcome: crate::domain_capabilities::ForgeQueryDomainCapabilityTransitionOutcome<T>,
) -> T {
    match outcome {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => panic!("expected success"),
    }
}

fn intent_declaration(name: &str) -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        name,
        "spatial.commit",
        "1",
        "geometry.patch",
        json!({"edge":"e-1"}),
    )
}

fn admitted_basis_observation_plan() -> crate::runtime::ForgeQueryAdmittedIntentPlan {
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::basis_observation_lane(
            crate::basis_lifecycle::RawBasisIntent::CurrentHead,
        )
        .expect("basis-observation request should build");

    match crate::intent_admission::admit_runtime_intent_request(request) {
        crate::intent_admission::ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan,
        other => panic!("expected admitted basis-observation plan, got {other:?}"),
    }
}
