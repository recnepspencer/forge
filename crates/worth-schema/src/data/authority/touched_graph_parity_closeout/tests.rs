use crate::data::authority::planner_owned_routing_semantic_graph::{
    admit_planner_admitted_explanation_input, admit_planner_public_proof_identity,
    admit_planner_selected_family_identity, admit_planner_selected_product_identity,
    admit_planner_selected_route_identity, admit_planner_witness_identity, PlannerMismatchLocus,
    PlannerWitnessRole,
};

use super::architecture_claim::{
    admit_touched_graph_parity_readiness_claim, TouchedGraphParityArchitectureClaim,
    TouchedGraphParityClaimKind,
};
use super::family_kind::TouchedGraphParityFamilyKind;
use super::readiness_input::admit_touched_graph_parity_readiness_input;
use super::residue_classification::TouchedGraphParityResidueClassification;

fn admitted_input() -> crate::data::authority::PlannerAdmittedExplanationInput {
    admit_planner_admitted_explanation_input("selection-input", "scope-digest").unwrap()
}

#[test]
fn parity_contract_vocabulary_distinguishes_claim_kinds() {
    let input = admitted_input();
    let family = admit_planner_selected_family_identity(&input, "loop-cycle-neighborhood").unwrap();
    let route = admit_planner_selected_route_identity(&family, "selected-route").unwrap();
    let product = admit_planner_selected_product_identity(&route, "compiled-product").unwrap();
    let witness = admit_planner_witness_identity(
        &route,
        PlannerWitnessRole::DenialOrAdvisory,
        PlannerMismatchLocus::QuerySupportPosture,
        "query posture carried",
    )
    .unwrap();
    let public_proof =
        admit_planner_public_proof_identity(&route, &product, "public-proof").unwrap();

    let declare_once = TouchedGraphParityArchitectureClaim::declare_once_family_parity(
        TouchedGraphParityFamilyKind::ReadRouting,
        route.clone(),
        family.clone(),
        Some(product.clone()),
        Some(witness.clone()),
    );
    let selected_route = TouchedGraphParityArchitectureClaim::selected_route_parity(
        TouchedGraphParityFamilyKind::Invalidation,
        route.clone(),
        family.clone(),
        Some(product.clone()),
        Some(witness.clone()),
    );
    let public_projection = TouchedGraphParityArchitectureClaim::public_projection_parity(
        TouchedGraphParityFamilyKind::PublicProof,
        route.clone(),
        family.clone(),
        product.clone(),
        public_proof.clone(),
    );
    let readiness = admit_touched_graph_parity_readiness_claim(
        TouchedGraphParityFamilyKind::DerivedDiagnostics,
        route,
        family,
        product,
        witness,
        public_proof,
    );

    assert_eq!(
        declare_once.kind(),
        TouchedGraphParityClaimKind::DeclareOnceFamilyParity
    );
    assert_eq!(
        selected_route.kind(),
        TouchedGraphParityClaimKind::SelectedRouteParity
    );
    assert_eq!(
        public_projection.kind(),
        TouchedGraphParityClaimKind::PublicProjectionParity
    );
    assert_eq!(
        readiness.kind(),
        TouchedGraphParityClaimKind::ReadinessParity
    );
}

#[test]
fn readiness_contract_rejects_untyped_helper_inputs() {
    let input = admitted_input();
    let family = admit_planner_selected_family_identity(&input, "loop-cycle-neighborhood").unwrap();
    let route = admit_planner_selected_route_identity(&family, "selected-route").unwrap();
    let claim = TouchedGraphParityArchitectureClaim::declare_once_family_parity(
        TouchedGraphParityFamilyKind::ReadRouting,
        route,
        family,
        None,
        None,
    );

    let error = admit_touched_graph_parity_readiness_input(
        claim,
        TouchedGraphParityResidueClassification::OrdinaryPathCarried,
        "touched-closure",
        vec!["overlap".to_string()],
        vec![TouchedGraphParityFamilyKind::ReadRouting],
        "topology-query-posture",
        "spatial-query-posture",
        "residue-digest",
        "firewall-digest",
        "architecture-claim-digest",
    )
    .expect_err("readiness input should reject lower parity claims");

    assert_eq!(
        error.kind(),
        super::error::TouchedGraphParityReadinessErrorKind::ClaimKindMustBeReadinessParity
    );
}

#[test]
fn readiness_claim_requires_full_authority_set() {
    let input = admitted_input();
    let family = admit_planner_selected_family_identity(&input, "loop-cycle-neighborhood").unwrap();
    let route = admit_planner_selected_route_identity(&family, "selected-route").unwrap();
    let product = admit_planner_selected_product_identity(&route, "compiled-product").unwrap();
    let witness = admit_planner_witness_identity(
        &route,
        PlannerWitnessRole::DenialOrAdvisory,
        PlannerMismatchLocus::QuerySupportPosture,
        "query posture carried",
    )
    .unwrap();
    let public_proof =
        admit_planner_public_proof_identity(&route, &product, "public-proof").unwrap();

    let claim = admit_touched_graph_parity_readiness_claim(
        TouchedGraphParityFamilyKind::DerivedDiagnostics,
        route,
        family,
        product,
        witness,
        public_proof,
    );

    assert!(claim.selected_product_identity().is_some());
    assert!(claim.witness_identity().is_some());
    assert!(claim.public_proof_identity().is_some());
}

#[test]
fn readiness_input_requires_full_handoff_fields() {
    let input = admitted_input();
    let family = admit_planner_selected_family_identity(&input, "loop-cycle-neighborhood").unwrap();
    let route = admit_planner_selected_route_identity(&family, "selected-route").unwrap();
    let product = admit_planner_selected_product_identity(&route, "compiled-product").unwrap();
    let witness = admit_planner_witness_identity(
        &route,
        PlannerWitnessRole::DenialOrAdvisory,
        PlannerMismatchLocus::QuerySupportPosture,
        "query posture carried",
    )
    .unwrap();
    let public_proof =
        admit_planner_public_proof_identity(&route, &product, "public-proof").unwrap();
    let claim = admit_touched_graph_parity_readiness_claim(
        TouchedGraphParityFamilyKind::DerivedDiagnostics,
        route,
        family,
        product,
        witness,
        public_proof,
    );

    let error = admit_touched_graph_parity_readiness_input(
        claim,
        TouchedGraphParityResidueClassification::OrdinaryPathCarried,
        "",
        Vec::new(),
        vec![TouchedGraphParityFamilyKind::ReadRouting],
        "topology-query-posture",
        "spatial-query-posture",
        "residue-digest",
        "firewall-digest",
        "architecture-claim-digest",
    )
    .expect_err("readiness input should require carried touched or overlap identity");

    assert_eq!(
        error.kind(),
        super::error::TouchedGraphParityReadinessErrorKind::MissingTouchedOrOverlapIdentity
    );
}

#[test]
fn readiness_input_derives_selected_authority_from_claim() {
    let input = admitted_input();
    let family = admit_planner_selected_family_identity(&input, "loop-cycle-neighborhood").unwrap();
    let route = admit_planner_selected_route_identity(&family, "selected-route").unwrap();
    let product = admit_planner_selected_product_identity(&route, "compiled-product").unwrap();
    let witness = admit_planner_witness_identity(
        &route,
        PlannerWitnessRole::DenialOrAdvisory,
        PlannerMismatchLocus::QuerySupportPosture,
        "query posture carried",
    )
    .unwrap();
    let public_proof =
        admit_planner_public_proof_identity(&route, &product, "public-proof").unwrap();
    let claim = admit_touched_graph_parity_readiness_claim(
        TouchedGraphParityFamilyKind::DerivedDiagnostics,
        route.clone(),
        family.clone(),
        product.clone(),
        witness.clone(),
        public_proof,
    );

    let readiness = admit_touched_graph_parity_readiness_input(
        claim,
        TouchedGraphParityResidueClassification::OrdinaryPathCarried,
        "touched-closure",
        vec!["overlap".to_string()],
        vec![TouchedGraphParityFamilyKind::ReadRouting],
        "topology-query-posture",
        "spatial-query-posture",
        "residue-digest",
        "firewall-digest",
        "architecture-claim-digest",
    )
    .expect("readiness input should derive selected authority from the carried claim");

    assert_eq!(
        readiness.selected_route_identity_digest(),
        route.identity_digest()
    );
    assert_eq!(
        readiness.selected_family_identity(),
        family.selected_family_name()
    );
    assert_eq!(
        readiness.selected_product_identity_digest(),
        product.identity_digest()
    );
    assert_eq!(
        readiness.selected_witness_identity_digest(),
        Some(witness.identity_digest())
    );
}

#[test]
fn family_kind_vocabulary_names_retained_spatial_as_shared_family() {
    assert!(
        TouchedGraphParityFamilyKind::ALL.contains(&TouchedGraphParityFamilyKind::RetainedSpatial)
    );
    assert_eq!(
        TouchedGraphParityFamilyKind::RetainedSpatial.as_str(),
        "retained-spatial"
    );
}
