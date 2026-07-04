mod declaration_row;

use crate::facade::evidence_lookup_route::current_evidence_lookup_route_packet;
use crate::workload_platform::touched_graph_parity_closeout::{
    SpatialTouchedGraphParityCoverageContributor, SpatialTouchedGraphParityCoverageError,
    SpatialTouchedGraphParityQuerySurfaceKind,
};

pub use declaration_row::current_spatial_evidence_lookup_declaration_row;

pub const SPATIAL_EVIDENCE_LOOKUP_CLAIM_PATH: &str =
    "crates/worth-spatial/src/workload_platform/touched_graph_parity_closeout/evidence_lookup_family/mod.rs";
pub const SPATIAL_EVIDENCE_LOOKUP_REPLACEMENT_LANE: &str =
    "crates/worth-spatial/src/workload_platform/touched_graph_parity_closeout/evidence_lookup_family/";

const SOURCE_PATH: &str = "crates/worth-spatial/src/facade/evidence_lookup_route/current.rs";
const SELECTED_FIELDS: &[&str] = &[
    "evidence_lookup_public_closeout_digest",
    "evidence_lookup_family_coverage_digest",
    "evidence_lookup_query_surface_matrix_digest",
    "evidence_lookup_query_consumer_kit_digest",
    "evidence_lookup_query_boundary_support_digest",
    "evidence_lookup_query_support_digest",
    "spatial_selected_family_identity",
    "spatial_selected_product_identity_digest",
];

pub fn current_spatial_evidence_lookup_coverage_contributor(
) -> Result<SpatialTouchedGraphParityCoverageContributor, SpatialTouchedGraphParityCoverageError> {
    let packet = current_evidence_lookup_route_packet()
        .map_err(|error| SpatialTouchedGraphParityCoverageError::new(error.detail()))?;
    if packet.route_packet_digest().is_empty()
        || packet.route_authority_digest().is_empty()
        || packet.query_support_digest().is_empty()
        || packet.selected_equivalence_family_identity().is_empty()
        || packet.compiled_product_identity_digest().is_empty()
    {
        return Err(SpatialTouchedGraphParityCoverageError::new(
            "spatial evidence-lookup coverage contributor requires a live route packet with route authority, query support, selected family, and compiled product identity",
        ));
    }

    Ok(SpatialTouchedGraphParityCoverageContributor::new(
        "current_evidence_lookup_route_packet",
        SOURCE_PATH,
        "current_evidence_lookup_route_packet",
        "current_evidence_lookup_route_packet",
        "family_route_product",
        SPATIAL_EVIDENCE_LOOKUP_REPLACEMENT_LANE,
        SELECTED_FIELDS,
        SpatialTouchedGraphParityQuerySurfaceKind::SupportPosture,
        "current_worth_touched_graph_conflict_selected_route_packet",
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs",
    ))
}
