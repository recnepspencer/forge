use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};
use topology::facade::TopologyDerivedReuseDecisionPosture;
use worth_spatial::facade::evidence_lookup_reuse_route::EvidenceLookupReuseDecisionPosture;

use super::contributor_catalog::{
    current_reuse_family_contributor_catalog, ReuseFamilyContributorCatalog,
};
use super::parity::{
    current_reuse_family_parity_claim, reuse_family_parity_claim_from_catalog,
    ReuseFamilyParityErrorKind,
};
use super::row::ReuseFamilyContributorRowKind;

#[test]
fn compiled_product_reuse_family_shares_one_semantic_graph_language() {
    let catalog = current_reuse_family_contributor_catalog().expect("reuse-family catalog");
    let claim = current_reuse_family_parity_claim().expect("reuse-family parity claim");

    assert_eq!(
        claim.kind(),
        TouchedGraphParityClaimKind::SelectedRouteParity
    );
    assert_eq!(catalog.rows().len(), 2);
    assert_eq!(claim.rows().len(), 2);
    assert!(catalog
        .rows()
        .iter()
        .all(|row| row.family_kind() == TouchedGraphParityFamilyKind::CompiledProductReuse));

    let reuse = catalog
        .rows()
        .iter()
        .find(|row| row.kind() == ReuseFamilyContributorRowKind::Reuse)
        .expect("reuse row");
    assert_eq!(
        reuse.current_packet_or_identity_source(),
        "current_worth_touched_graph_conflict_compiled_product_reuse_route_packet"
    );
    assert!(reuse
        .carried_reuse_or_denial_source()
        .contains("rebuild_denial_identity_digest"));
}

#[test]
fn compiled_product_reuse_parity_rejects_result_only_equivalence() {
    let mut hostile_rows = current_reuse_family_contributor_catalog()
        .expect("reuse-family catalog")
        .rows()
        .to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_identity_override(
        "foreign-topology-policy",
        "foreign-topology-compatibility",
        hostile_rows[0].topology_selected_reuse_basis_identity_digest(),
        hostile_rows[0].topology_reuse_decision_identity_digest(),
        hostile_rows[0].topology_rebuild_denial_identity_digest(),
        "foreign-spatial-policy",
        hostile_rows[0].certified_spatial_equivalence_basis_digest(),
        hostile_rows[0].spatial_selected_reuse_basis_identity_digest(),
        hostile_rows[0].spatial_reuse_decision_identity_digest(),
        hostile_rows[0].spatial_rebuild_denial_identity_digest(),
    );

    assert_eq!(
        reuse_family_parity_claim_from_catalog(
            &ReuseFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("result-only equivalence must not pass reuse parity")
        .kind(),
        ReuseFamilyParityErrorKind::MismatchedEquivalenceIdentity
    );
}

#[test]
fn compiled_product_reuse_parity_rejects_reuse_basis_drift() {
    let mut hostile_rows = current_reuse_family_contributor_catalog()
        .expect("reuse-family catalog")
        .rows()
        .to_vec();
    hostile_rows[1] = hostile_rows[1].clone().with_test_identity_override(
        hostile_rows[1].topology_equivalence_policy_identity_digest(),
        hostile_rows[1].certified_topology_equivalence_basis_digest(),
        "foreign-topology-reuse-basis",
        hostile_rows[1].topology_reuse_decision_identity_digest(),
        hostile_rows[1].topology_rebuild_denial_identity_digest(),
        hostile_rows[1].spatial_equivalence_policy_identity_digest(),
        hostile_rows[1].certified_spatial_equivalence_basis_digest(),
        "foreign-spatial-reuse-basis",
        hostile_rows[1].spatial_reuse_decision_identity_digest(),
        hostile_rows[1].spatial_rebuild_denial_identity_digest(),
    );

    assert_eq!(
        reuse_family_parity_claim_from_catalog(
            &ReuseFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("reuse basis drift must be rejected")
        .kind(),
        ReuseFamilyParityErrorKind::MismatchedReuseIdentity
    );
}

#[test]
fn compiled_product_reuse_parity_rejects_spatial_reuse_basis_drift() {
    let mut hostile_rows = current_reuse_family_contributor_catalog()
        .expect("reuse-family catalog")
        .rows()
        .to_vec();
    hostile_rows[1] = hostile_rows[1].clone().with_test_identity_override(
        hostile_rows[1].topology_equivalence_policy_identity_digest(),
        hostile_rows[1].certified_topology_equivalence_basis_digest(),
        hostile_rows[1].topology_selected_reuse_basis_identity_digest(),
        hostile_rows[1].topology_reuse_decision_identity_digest(),
        hostile_rows[1].topology_rebuild_denial_identity_digest(),
        hostile_rows[1].spatial_equivalence_policy_identity_digest(),
        hostile_rows[1].certified_spatial_equivalence_basis_digest(),
        "foreign-spatial-reuse-basis",
        hostile_rows[1].spatial_reuse_decision_identity_digest(),
        hostile_rows[1].spatial_rebuild_denial_identity_digest(),
    );

    assert_eq!(
        reuse_family_parity_claim_from_catalog(
            &ReuseFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("spatial reuse basis drift must be rejected")
        .kind(),
        ReuseFamilyParityErrorKind::MismatchedReuseIdentity
    );
}

#[test]
fn compiled_product_reuse_parity_rejects_equivalence_policy_drift() {
    let mut hostile_rows = current_reuse_family_contributor_catalog()
        .expect("reuse-family catalog")
        .rows()
        .to_vec();
    hostile_rows[1] = hostile_rows[1].clone().with_test_identity_override(
        "foreign-topology-policy",
        hostile_rows[1].certified_topology_equivalence_basis_digest(),
        hostile_rows[1].topology_selected_reuse_basis_identity_digest(),
        hostile_rows[1].topology_reuse_decision_identity_digest(),
        hostile_rows[1].topology_rebuild_denial_identity_digest(),
        "foreign-spatial-policy",
        hostile_rows[1].certified_spatial_equivalence_basis_digest(),
        hostile_rows[1].spatial_selected_reuse_basis_identity_digest(),
        hostile_rows[1].spatial_reuse_decision_identity_digest(),
        hostile_rows[1].spatial_rebuild_denial_identity_digest(),
    );

    assert_eq!(
        reuse_family_parity_claim_from_catalog(
            &ReuseFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("equivalence policy drift must be rejected")
        .kind(),
        ReuseFamilyParityErrorKind::MismatchedReuseIdentity
    );
}

#[test]
fn compiled_product_reuse_parity_rejects_denial_identity_drift() {
    let mut hostile_rows = current_reuse_family_contributor_catalog()
        .expect("reuse-family catalog")
        .rows()
        .to_vec();
    hostile_rows[1] = hostile_rows[1].clone().with_test_identity_override(
        hostile_rows[1].topology_equivalence_policy_identity_digest(),
        hostile_rows[1].certified_topology_equivalence_basis_digest(),
        hostile_rows[1].topology_selected_reuse_basis_identity_digest(),
        hostile_rows[1].topology_reuse_decision_identity_digest(),
        Some("foreign-topology-denial"),
        hostile_rows[1].spatial_equivalence_policy_identity_digest(),
        hostile_rows[1].certified_spatial_equivalence_basis_digest(),
        hostile_rows[1].spatial_selected_reuse_basis_identity_digest(),
        hostile_rows[1].spatial_reuse_decision_identity_digest(),
        Some("foreign-spatial-denial"),
    );

    assert_eq!(
        reuse_family_parity_claim_from_catalog(
            &ReuseFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("denial identity drift must be rejected")
        .kind(),
        ReuseFamilyParityErrorKind::MismatchedReuseIdentity
    );
}

#[test]
fn compiled_product_reuse_parity_rejects_reuse_posture_drift() {
    let mut hostile_rows = current_reuse_family_contributor_catalog()
        .expect("reuse-family catalog")
        .rows()
        .to_vec();
    hostile_rows[1] = hostile_rows[1].clone().with_test_posture_override(
        TopologyDerivedReuseDecisionPosture::AdvisoryMatchRequiresRebuild,
        EvidenceLookupReuseDecisionPosture::AdvisoryMatchRequiresRebuild,
    );

    assert_eq!(
        reuse_family_parity_claim_from_catalog(
            &ReuseFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("reuse posture drift must be rejected")
        .kind(),
        ReuseFamilyParityErrorKind::MismatchedReuseIdentity
    );
}

#[test]
fn compiled_product_reuse_parity_rejects_spatial_compatibility_witness_drift() {
    let mut hostile_rows = current_reuse_family_contributor_catalog()
        .expect("reuse-family catalog")
        .rows()
        .to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_identity_override(
        hostile_rows[0].topology_equivalence_policy_identity_digest(),
        hostile_rows[0].certified_topology_equivalence_basis_digest(),
        hostile_rows[0].topology_selected_reuse_basis_identity_digest(),
        hostile_rows[0].topology_reuse_decision_identity_digest(),
        hostile_rows[0].topology_rebuild_denial_identity_digest(),
        hostile_rows[0].spatial_equivalence_policy_identity_digest(),
        "foreign-spatial-compatibility",
        hostile_rows[0].spatial_selected_reuse_basis_identity_digest(),
        hostile_rows[0].spatial_reuse_decision_identity_digest(),
        hostile_rows[0].spatial_rebuild_denial_identity_digest(),
    );

    assert_eq!(
        reuse_family_parity_claim_from_catalog(
            &ReuseFamilyContributorCatalog::new_unvalidated_for_testing(hostile_rows)
        )
        .expect_err("spatial compatibility witness drift must be rejected")
        .kind(),
        ReuseFamilyParityErrorKind::MismatchedEquivalenceIdentity
    );
}
