mod declaration_row;

use crate::facade::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout,
    current_evidence_lookup_public_closeout_residue_manifest,
};
use crate::workload_platform::touched_graph_parity_closeout::{
    SpatialTouchedGraphParityCoverageContributor, SpatialTouchedGraphParityCoverageError,
    SpatialTouchedGraphParityQuerySurfaceKind,
};

pub use declaration_row::current_spatial_retained_surface_declaration_row;

pub const SPATIAL_RETAINED_SURFACE_CLAIM_PATH: &str =
    "crates/worth-spatial/src/workload_platform/touched_graph_parity_closeout/retained_surface_family/mod.rs";
pub const SPATIAL_RETAINED_SURFACE_REPLACEMENT_LANE: &str =
    "crates/worth-spatial/src/workload_platform/touched_graph_parity_closeout/retained_surface_family/";

const SOURCE_PATH: &str =
    "crates/worth-spatial/src/facade/evidence_lookup_public_closeout/current.rs";
const SELECTED_FIELDS: &[&str] = &[
    "closeout_digest",
    "family_coverage_digest",
    "residue_audit_digest",
    "support_snapshot_digest",
    "consumer_residue_report_identity",
];

pub fn current_spatial_retained_surface_coverage_contributor(
) -> Result<SpatialTouchedGraphParityCoverageContributor, SpatialTouchedGraphParityCoverageError> {
    let closeout = current_evidence_lookup_public_closeout()
        .map_err(|error| SpatialTouchedGraphParityCoverageError::new(error.detail()))?;
    let residue = current_evidence_lookup_public_closeout_residue_manifest();
    if closeout.closeout_digest().is_empty()
        || closeout.residue_audit_digest().is_empty()
        || closeout
            .query_consumer_kit()
            .consumer_residue_report_identity()
            .is_empty()
        || residue.iter().any(|row| row.current_surface().is_empty())
    {
        return Err(SpatialTouchedGraphParityCoverageError::new(
            "retained-surface coverage contributor requires live public-closeout, residue-audit, and consumer-residue authority",
        ));
    }

    Ok(SpatialTouchedGraphParityCoverageContributor::new(
        "current_evidence_lookup_public_closeout",
        SOURCE_PATH,
        "current_evidence_lookup_public_closeout",
        "current_evidence_lookup_public_closeout",
        "retained_public_closeout_surface",
        SPATIAL_RETAINED_SURFACE_REPLACEMENT_LANE,
        SELECTED_FIELDS,
        SpatialTouchedGraphParityQuerySurfaceKind::ConsumerResidue,
        "current_evidence_lookup_public_closeout",
        SOURCE_PATH,
    ))
}
