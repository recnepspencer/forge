use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::facade::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;
use crate::facade::evidence_lookup_route::current_evidence_lookup_route_packet;
use crate::workload_platform::touched_graph_parity_closeout::contributor_catalog::{
    SpatialContributorCatalogRowKind, SpatialContributorLocalLanguagePosture,
    SpatialContributorQueryBoundaryAuthority, SpatialContributorQueryInputKind,
    SpatialFamilyContributorCatalogRow,
};
use crate::workload_platform::touched_graph_parity_closeout::evidence_lookup_family::{
    current_spatial_evidence_lookup_coverage_contributor, SPATIAL_EVIDENCE_LOOKUP_REPLACEMENT_LANE,
};
use crate::workload_platform::touched_graph_parity_closeout::SpatialTouchedGraphParityCoverageError;

const PRODUCED_FIELDS: &[&str] = &[
    "route_packet_digest",
    "route_authority_digest",
    "selected_equivalence_family_identity",
    "compiled_product_identity_digest",
    "query_support_digest",
];

pub fn current_spatial_evidence_lookup_declaration_row(
) -> Result<SpatialFamilyContributorCatalogRow, SpatialTouchedGraphParityCoverageError> {
    let packet = current_evidence_lookup_route_packet()
        .map_err(|error| SpatialTouchedGraphParityCoverageError::new(error.detail()))?;
    let closeout = current_evidence_lookup_public_closeout()
        .map_err(|error| SpatialTouchedGraphParityCoverageError::new(error.detail()))?;
    if packet.route_packet_digest().is_empty()
        || closeout.closeout_digest().is_empty()
        || closeout
            .query_consumer_kit()
            .support_snapshot_digest()
            .is_empty()
        || closeout
            .query_consumer_kit()
            .consumer_residue_report_identity()
            .is_empty()
    {
        return Err(SpatialTouchedGraphParityCoverageError::new(
            "evidence-lookup declaration row requires live route, public-closeout, Query support posture, and Query consumer residue inputs",
        ));
    }

    SpatialFamilyContributorCatalogRow::new(
        SpatialContributorCatalogRowKind::EvidenceLookupFamily,
        TouchedGraphParityFamilyKind::EvidenceLookup,
        "current_evidence_lookup_route_packet",
        "current_evidence_lookup_public_closeout::{closeout_digest,family_coverage_digest,query_boundary_support_digest}",
        "current_evidence_lookup_query_consumer_kit::{support_snapshot_digest,support_pin_contract_digest,support_pin_report_digest}",
        "current_evidence_lookup_query_consumer_kit::{consumer_residue_report_identity,consumer_residue_source_inventory_digest,query_residue_rows}",
        None,
        PRODUCED_FIELDS,
        SpatialContributorQueryInputKind::SupportPostureAndConsumerResidue,
        SpatialContributorQueryBoundaryAuthority::QueryOwnedSupportAndResidue,
        SpatialContributorLocalLanguagePosture::ExplicitlyBlocked {
            legacy_surface: "planner_owned_routing/evidence_lookup_route",
            blocking_surface: "current_spatial_evidence_lookup_coverage_contributor",
        },
        current_spatial_evidence_lookup_coverage_contributor()?,
    )
    .map_err(|error| {
        SpatialTouchedGraphParityCoverageError::new(format!(
            "{} ({SPATIAL_EVIDENCE_LOOKUP_REPLACEMENT_LANE})",
            error.detail()
        ))
    })
}
