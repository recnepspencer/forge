use super::contributor_catalog::{
    current_public_projection_contributor_catalog, PublicProjectionContributorCatalog,
};
use super::parity::{
    current_public_projection_parity_claim, public_projection_parity_claim_from_catalog,
    PublicProjectionParityErrorKind,
};
use super::row::PublicProjectionContributorRowKind;
use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityClaimKind;

#[test]
fn public_proof_and_diagnostics_agree_on_carried_route_authority() {
    let claim = current_public_projection_parity_claim()
        .expect("current public projection parity claim should build");
    let catalog = current_public_projection_contributor_catalog()
        .expect("current public projection catalog should build");

    assert_eq!(claim.rows().len(), 2);
    assert_eq!(catalog.rows().len(), 2);
    assert_eq!(
        claim.kind(),
        TouchedGraphParityClaimKind::PublicProjectionParity
    );
    assert!(catalog
        .rows()
        .iter()
        .any(|row| row.kind() == PublicProjectionContributorRowKind::PublicProof));
    assert!(catalog
        .rows()
        .iter()
        .any(|row| row.kind() == PublicProjectionContributorRowKind::DerivedDiagnostics));
}

#[test]
fn public_projection_parity_rejects_foreign_selected_route_identity() {
    let catalog = current_public_projection_contributor_catalog().expect("catalog");
    let mut hostile_rows = catalog.rows().to_vec();
    hostile_rows[1] = hostile_rows[1].clone().with_test_authority_override(
        "foreign-selected-route",
        hostile_rows[1].selected_family_identity(),
        hostile_rows[1].selected_product_identity_digest(),
        hostile_rows[1].selected_witness_identity_digest(),
        None,
        None,
        None,
        None,
        hostile_rows[1].decision_trace_identity_digest(),
    );

    let error = public_projection_parity_claim_from_catalog(
        &PublicProjectionContributorCatalog::new_unvalidated_for_testing(hostile_rows),
    )
    .expect_err("parity must reject foreign selected-route identity");

    assert_eq!(
        error.kind(),
        PublicProjectionParityErrorKind::MismatchedProjectionAuthority
    );
}

#[test]
fn public_projection_parity_rejects_foreign_seed_digest() {
    let catalog = current_public_projection_contributor_catalog().expect("catalog");
    let mut hostile_rows = catalog.rows().to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_authority_override(
        hostile_rows[0].selected_route_identity_digest(),
        hostile_rows[0].selected_family_identity(),
        hostile_rows[0].selected_product_identity_digest(),
        hostile_rows[0].selected_witness_identity_digest(),
        hostile_rows[0].proof_chain_digest(),
        Some("foreign-seed-digest"),
        hostile_rows[0].residue_digest(),
        hostile_rows[0].source_firewall_digest(),
        None,
    );

    let error = public_projection_parity_claim_from_catalog(
        &PublicProjectionContributorCatalog::new_unvalidated_for_testing(hostile_rows),
    )
    .expect_err("parity must reject foreign seed digest");

    assert_eq!(
        error.kind(),
        PublicProjectionParityErrorKind::MismatchedProjectionAuthority
    );
}

#[test]
fn public_projection_parity_rejects_foreign_residue_digest() {
    let catalog = current_public_projection_contributor_catalog().expect("catalog");
    let mut hostile_rows = catalog.rows().to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_authority_override(
        hostile_rows[0].selected_route_identity_digest(),
        hostile_rows[0].selected_family_identity(),
        hostile_rows[0].selected_product_identity_digest(),
        hostile_rows[0].selected_witness_identity_digest(),
        hostile_rows[0].proof_chain_digest(),
        hostile_rows[0].milestone_fifteen_seed_digest(),
        Some("foreign-residue-digest"),
        hostile_rows[0].source_firewall_digest(),
        None,
    );

    let error = public_projection_parity_claim_from_catalog(
        &PublicProjectionContributorCatalog::new_unvalidated_for_testing(hostile_rows),
    )
    .expect_err("parity must reject foreign residue digest");

    assert_eq!(
        error.kind(),
        PublicProjectionParityErrorKind::MismatchedProjectionAuthority
    );
}

#[test]
fn public_projection_parity_rejects_foreign_source_firewall_digest() {
    let catalog = current_public_projection_contributor_catalog().expect("catalog");
    let mut hostile_rows = catalog.rows().to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_authority_override(
        hostile_rows[0].selected_route_identity_digest(),
        hostile_rows[0].selected_family_identity(),
        hostile_rows[0].selected_product_identity_digest(),
        hostile_rows[0].selected_witness_identity_digest(),
        hostile_rows[0].proof_chain_digest(),
        hostile_rows[0].milestone_fifteen_seed_digest(),
        hostile_rows[0].residue_digest(),
        Some("foreign-firewall-digest"),
        None,
    );

    let error = public_projection_parity_claim_from_catalog(
        &PublicProjectionContributorCatalog::new_unvalidated_for_testing(hostile_rows),
    )
    .expect_err("parity must reject foreign source-firewall digest");

    assert_eq!(
        error.kind(),
        PublicProjectionParityErrorKind::MismatchedProjectionAuthority
    );
}

#[test]
fn public_projection_parity_rejects_foreign_public_proof_witness_identity() {
    let catalog = current_public_projection_contributor_catalog().expect("catalog");
    let mut hostile_rows = catalog.rows().to_vec();
    hostile_rows[0] = hostile_rows[0].clone().with_test_authority_override(
        hostile_rows[0].selected_route_identity_digest(),
        hostile_rows[0].selected_family_identity(),
        hostile_rows[0].selected_product_identity_digest(),
        Some("foreign-public-proof-witness"),
        hostile_rows[0].proof_chain_digest(),
        hostile_rows[0].milestone_fifteen_seed_digest(),
        hostile_rows[0].residue_digest(),
        hostile_rows[0].source_firewall_digest(),
        None,
    );

    let error = public_projection_parity_claim_from_catalog(
        &PublicProjectionContributorCatalog::new_unvalidated_for_testing(hostile_rows),
    )
    .expect_err("parity must reject foreign public-proof witness identity");

    assert_eq!(
        error.kind(),
        PublicProjectionParityErrorKind::MismatchedProjectionAuthority
    );
}
