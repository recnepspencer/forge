
use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityClaimKind, TouchedGraphParityFamilyKind,
};
use worth_spatial::facade::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout_residue_manifest,
    EvidenceLookupPublicCloseoutResidueDisposition, EvidenceLookupPublicCloseoutResidueOwner,
};
use worth_spatial::touched_graph_parity_closeout::{
    SpatialContributorCatalogRowKind, SpatialContributorLocalLanguagePosture,
    SpatialContributorQueryBoundaryAuthority, SpatialFamilyContributorCatalogRow,
    SPATIAL_EVIDENCE_LOOKUP_REPLACEMENT_LANE, SPATIAL_RETAINED_SURFACE_REPLACEMENT_LANE,
};

use super::*;


#[test]
fn spatial_family_parity_preserves_query_owned_support_meaning() {
    let catalog =
        current_spatial_family_contributor_catalog().expect("spatial contributor catalog");
    let claim = current_spatial_family_parity_claim().expect("spatial family parity claim");
    let residue = current_evidence_lookup_public_closeout_residue_manifest();

    assert_eq!(
        claim.kind(),
        TouchedGraphParityClaimKind::DeclareOnceFamilyParity
    );
    assert_eq!(catalog.rows().len(), 2);
    assert_eq!(claim.rows().len(), 2);
    let evidence_row = catalog
        .rows()
        .iter()
        .find(|row: &&SpatialFamilyContributorCatalogRow| {
            row.kind() == SpatialContributorCatalogRowKind::EvidenceLookupFamily
        })
        .expect("evidence row");
    assert_eq!(
        evidence_row.current_packet_or_digest_source(),
        "current_evidence_lookup_route_packet"
    );
    assert_eq!(
        evidence_row.family_kind(),
        TouchedGraphParityFamilyKind::EvidenceLookup
    );
    assert!(evidence_row
        .support_posture_source()
        .contains("support_snapshot_digest"));
    assert!(evidence_row
        .consumer_residue_source()
        .contains("consumer_residue_report_identity"));

    let retained_row = catalog
        .rows()
        .iter()
        .find(|row: &&SpatialFamilyContributorCatalogRow| {
            row.kind() == SpatialContributorCatalogRowKind::RetainedSurfaceFamily
        })
        .expect("retained row");
    assert_eq!(
        retained_row.current_packet_or_digest_source(),
        "current_evidence_lookup_public_closeout"
    );
    assert_eq!(
        retained_row.family_kind(),
        TouchedGraphParityFamilyKind::RetainedSpatial
    );
    assert_eq!(
        retained_row.worth_local_residue_source(),
        Some("current_evidence_lookup_public_closeout_residue_manifest::{WorthSpatial/ExplicitResidue}")
    );
    assert!(residue.iter().all(|row| {
        row.owner() != EvidenceLookupPublicCloseoutResidueOwner::WorthSpatial
            || row.disposition() != EvidenceLookupPublicCloseoutResidueDisposition::QueryGap
    }));
    assert!(claim.rows().iter().any(|row| {
        row.kind() == SpatialContributorCatalogRowKind::RetainedSurfaceFamily
            && row.family_kind() == TouchedGraphParityFamilyKind::RetainedSpatial
    }));
    assert!(claim
        .rows()
        .iter()
        .all(|row| !row.selected_identity_fields_produced().is_empty()));
}

#[test]
fn spatial_family_parity_rejects_local_query_gap_fabrication() {
    let rows = current_spatial_catalog()
        .expect("raw spatial catalog")
        .rows()
        .to_vec();
    let mut hostile_rows = rows;
    hostile_rows[1] = hostile_rows[1]
        .clone()
        .with_query_boundary_authority_for_testing(
            SpatialContributorQueryBoundaryAuthority::FabricatedLocalQueryGap {
                fabricated_surface: "spatial-query-gap-helper",
            },
        );

    assert_eq!(
        spatial_family_parity_claim_from_catalog(
            &worth_spatial::touched_graph_parity_closeout::SpatialFamilyContributorCatalog::new(
                hostile_rows,
            )
            .expect("hostile spatial catalog")
        )
        .expect_err("local Query-gap fabrication must be rejected")
        .kind(),
        SpatialFamilyParityErrorKind::LocalQueryGapFabricationStillAuthoritative
    );

    current_spatial_family_parity_claim()
        .expect("production spatial family parity should remain valid");
}

#[test]
fn spatial_family_contributors_name_final_closeout_lanes_and_block_legacy_local_language() {
    let catalog = current_spatial_catalog().expect("spatial contributor catalog should load");

    assert!(SPATIAL_EVIDENCE_LOOKUP_REPLACEMENT_LANE.contains("touched_graph_parity_closeout"));
    assert!(SPATIAL_RETAINED_SURFACE_REPLACEMENT_LANE.contains("touched_graph_parity_closeout"));

    let evidence_row = catalog
        .rows()
        .iter()
        .find(|row: &&SpatialFamilyContributorCatalogRow| {
            row.kind() == SpatialContributorCatalogRowKind::EvidenceLookupFamily
        })
        .expect("evidence row");
    assert!(matches!(
        evidence_row.local_language_posture(),
        SpatialContributorLocalLanguagePosture::ExplicitlyBlocked { .. }
    ));
}
