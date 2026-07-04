use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::facade::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout,
    current_evidence_lookup_public_closeout_residue_manifest,
    EvidenceLookupPublicCloseoutResidueDisposition, EvidenceLookupPublicCloseoutResidueOwner,
};
use crate::workload_platform::touched_graph_parity_closeout::contributor_catalog::{
    SpatialContributorCatalogRowKind, SpatialContributorLocalLanguagePosture,
    SpatialContributorQueryBoundaryAuthority, SpatialContributorQueryInputKind,
    SpatialFamilyContributorCatalogRow,
};
use crate::workload_platform::touched_graph_parity_closeout::retained_surface_family::{
    current_spatial_retained_surface_coverage_contributor,
    SPATIAL_RETAINED_SURFACE_REPLACEMENT_LANE,
};
use crate::workload_platform::touched_graph_parity_closeout::SpatialTouchedGraphParityCoverageError;

const PRODUCED_FIELDS: &[&str] = &[
    "closeout_digest",
    "family_coverage_digest",
    "residue_audit_digest",
    "query_boundary_support_digest",
    "spatial_deletion_ledger_digest",
];

pub fn current_spatial_retained_surface_declaration_row(
) -> Result<SpatialFamilyContributorCatalogRow, SpatialTouchedGraphParityCoverageError> {
    let closeout = current_evidence_lookup_public_closeout()
        .map_err(|error| SpatialTouchedGraphParityCoverageError::new(error.detail()))?;
    let residue = current_evidence_lookup_public_closeout_residue_manifest();
    if closeout.closeout_digest().is_empty()
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
            "retained-surface declaration row requires live public-closeout, Query support posture, and Query consumer residue inputs",
        ));
    }
    if residue.iter().any(|row| {
        row.owner() == EvidenceLookupPublicCloseoutResidueOwner::WorthSpatial
            && row.disposition() == EvidenceLookupPublicCloseoutResidueDisposition::QueryGap
    }) {
        return Err(SpatialTouchedGraphParityCoverageError::new(
            "retained-surface declaration row rejects Worth-local Query-gap fabrication in the public-closeout residue manifest",
        ));
    }

    SpatialFamilyContributorCatalogRow::new(
        SpatialContributorCatalogRowKind::RetainedSurfaceFamily,
        TouchedGraphParityFamilyKind::RetainedSpatial,
        "current_evidence_lookup_public_closeout",
        "current_evidence_lookup_public_closeout::{closeout_digest,family_coverage_digest,residue_audit_digest}",
        "current_evidence_lookup_query_consumer_kit::{support_snapshot_digest,support_pin_contract_digest,support_pin_report_digest}",
        "current_evidence_lookup_query_consumer_kit::{consumer_residue_report_identity,consumer_residue_source_inventory_digest,query_residue_rows}",
        Some(
            "current_evidence_lookup_public_closeout_residue_manifest::{WorthSpatial/ExplicitResidue}",
        ),
        PRODUCED_FIELDS,
        SpatialContributorQueryInputKind::SupportPostureAndConsumerResidue,
        SpatialContributorQueryBoundaryAuthority::QueryOwnedSupportAndResidue,
        SpatialContributorLocalLanguagePosture::ExplicitlyBlocked {
            legacy_surface: "spatial_query_gap_ledger",
            blocking_surface: "current_evidence_lookup_public_closeout_residue_manifest",
        },
        current_spatial_retained_surface_coverage_contributor()?,
    )
    .map_err(|error| {
        SpatialTouchedGraphParityCoverageError::new(format!(
            "{} ({SPATIAL_RETAINED_SURFACE_REPLACEMENT_LANE})",
            error.detail()
        ))
    })
}
