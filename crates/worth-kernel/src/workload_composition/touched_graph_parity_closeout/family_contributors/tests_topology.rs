use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};
use topology::touched_graph_parity_closeout::{
    current_topology_family_contributor_catalog as current_topology_catalog,
    TopologyContributorCatalogRowKind, TopologyContributorCoverageAuthority,
    TopologyContributorLocalLanguagePosture, TOPOLOGY_INVALIDATION_REPLACEMENT_LANE,
    TOPOLOGY_READ_FAMILY_REPLACEMENT_LANE, TOPOLOGY_VALIDATOR_INVARIANT_REPLACEMENT_LANE,
};

use super::topology_family_catalog::current_topology_family_contributor_catalog;
use super::topology_family_parity::{
    current_topology_family_declare_once_parity_claim,
    topology_family_declare_once_parity_claim_from_catalog,
};
use super::topology_family_parity::TopologyFamilyDeclareOnceParityErrorKind;

#[test]
fn topology_family_declare_once_parity_holds_across_read_validator_invalidation() {
    let catalog =
        current_topology_family_contributor_catalog().expect("topology contributor catalog");
    let claim =
        current_topology_family_declare_once_parity_claim().expect("declare-once parity claim");

    assert_eq!(
        claim.kind(),
        TouchedGraphParityClaimKind::DeclareOnceFamilyParity
    );
    assert_eq!(catalog.rows().len(), 3);
    assert_eq!(claim.rows().len(), 3);
    let read_row = catalog
        .rows()
        .iter()
        .find(|row| row.kind() == TopologyContributorCatalogRowKind::ReadFamily)
        .expect("read family row");
    assert!(matches!(
        read_row.coverage_authority(),
        TopologyContributorCoverageAuthority::ReadRequestFamilies(_)
    ));
    let validator_row = catalog
        .rows()
        .iter()
        .find(|row| row.kind() == TopologyContributorCatalogRowKind::ValidatorInvariantFamily)
        .expect("validator row");
    assert!(matches!(
        validator_row.coverage_authority(),
        TopologyContributorCoverageAuthority::ValidatorRuleIdentities(_)
    ));
    assert_eq!(
        validator_row.current_packet_or_function(),
        "current_topology_validator_invariant_milestone_nine_closeout"
    );
    assert!(validator_row.operator_or_stage_coverage().len() > 1);
    let invalidation_row = catalog
        .rows()
        .iter()
        .find(|row| row.kind() == TopologyContributorCatalogRowKind::InvalidationFamily)
        .expect("invalidation row");
    assert!(matches!(
        invalidation_row.coverage_authority(),
        TopologyContributorCoverageAuthority::InvalidationStageIdentities(_)
    ));
    assert!(claim.rows().iter().any(|row| {
        row.family_kind() == TouchedGraphParityFamilyKind::ReadRouting
            && !row.current_packet_or_function().is_empty()
    }));
    assert!(claim
        .rows()
        .iter()
        .all(|row| !row.selected_identity_fields_produced().is_empty()));
}

#[test]
fn topology_family_parity_rejects_operator_local_routing_folklore() {
    let rows = current_topology_catalog()
        .expect("raw topology catalog")
        .rows()
        .to_vec();
    let mut local_language_rows = rows.clone();
    local_language_rows[1] = local_language_rows[1]
        .clone()
        .with_local_language_posture_for_testing(
            TopologyContributorLocalLanguagePosture::AuthoritativeLocalHelper {
                legacy_surface: "operator-local-validator-expectation",
            },
        );
    assert_eq!(
        topology_family_declare_once_parity_claim_from_catalog(
            &topology::touched_graph_parity_closeout::TopologyFamilyContributorCatalog::new(
                local_language_rows,
            )
            .expect("hostile catalog")
        )
        .expect_err("operator-local routing folklore must be rejected")
        .kind(),
        TopologyFamilyDeclareOnceParityErrorKind::OperatorLocalRoutingStillAuthoritative
    );

    let mut entity_fallback_rows = rows;
    entity_fallback_rows[1] = entity_fallback_rows[1]
        .clone()
        .with_coverage_authority_for_testing(
            TopologyContributorCoverageAuthority::RegistrationEntityFallback(
                entity_fallback_rows[1]
                    .operator_or_stage_coverage()
                    .to_vec(),
            ),
        );
    assert_eq!(
        topology_family_declare_once_parity_claim_from_catalog(
            &topology::touched_graph_parity_closeout::TopologyFamilyContributorCatalog::new(
                entity_fallback_rows,
            )
            .expect("entity fallback catalog")
        )
        .expect_err("entity fallback coverage must be rejected")
        .kind(),
        TopologyFamilyDeclareOnceParityErrorKind::EntityFallbackStillAuthoritative
    );

    current_topology_family_declare_once_parity_claim()
        .expect("production parity claim should remain valid");
}

#[test]
fn topology_family_contributors_name_final_closeout_lanes_and_block_legacy_local_language() {
    let catalog = current_topology_catalog().expect("topology contributor catalog should load");

    assert!(TOPOLOGY_READ_FAMILY_REPLACEMENT_LANE.contains("touched_graph_parity_closeout"));
    assert!(TOPOLOGY_VALIDATOR_INVARIANT_REPLACEMENT_LANE.contains("touched_graph_parity_closeout"));
    assert!(TOPOLOGY_INVALIDATION_REPLACEMENT_LANE.contains("touched_graph_parity_closeout"));

    let read_row = catalog
        .rows()
        .iter()
        .find(|row| row.kind() == TopologyContributorCatalogRowKind::ReadFamily)
        .expect("read row");
    assert!(matches!(
        read_row.local_language_posture(),
        TopologyContributorLocalLanguagePosture::ExplicitlyBlocked { .. }
    ));
}
