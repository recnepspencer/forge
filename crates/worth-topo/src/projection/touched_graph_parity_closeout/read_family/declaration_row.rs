use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::projection::query_backed_consumer_cutover::current_topology_query_backed_consumer_cutover;
use crate::projection::touched_graph_parity_closeout::contributor_catalog::{
    TopologyContributorCatalogRowKind, TopologyContributorCoverageAuthority,
    TopologyContributorLocalLanguagePosture, TopologyFamilyContributorCatalogRow,
};
use crate::projection::touched_graph_parity_closeout::read_family::{
    current_topology_read_family_coverage_contributor, TOPOLOGY_READ_FAMILY_REPLACEMENT_LANE,
};
use crate::projection::touched_graph_parity_closeout::TopologyTouchedGraphParityCoverageError;

const PRODUCED_FIELDS: &[&str] = &[
    "support_snapshot_digest",
    "selected_equivalence_family_identity",
    "compiled_product_identity_digest",
    "equivalence_policy_identity_digest",
    "selected_reuse_basis_identity_digest",
];

pub fn current_topology_read_declaration_row(
) -> Result<TopologyFamilyContributorCatalogRow, TopologyTouchedGraphParityCoverageError> {
    let cutover = current_topology_query_backed_consumer_cutover()
        .map_err(|error| TopologyTouchedGraphParityCoverageError::new(error.detail()))?;
    let coverage = cutover
        .family_rows()
        .iter()
        .map(|row| row.request_family().as_str().to_string())
        .collect::<Vec<_>>();

    TopologyFamilyContributorCatalogRow::new(
        TopologyContributorCatalogRowKind::ReadFamily,
        TouchedGraphParityFamilyKind::ReadRouting,
        "current_topology_query_backed_consumer_cutover",
        PRODUCED_FIELDS,
        TopologyContributorCoverageAuthority::ReadRequestFamilies(coverage),
        TopologyContributorLocalLanguagePosture::ExplicitlyBlocked {
            legacy_surface: "planner_owned_routing/query_backed_read_family",
            blocking_surface: "current_topology_read_family_coverage_contributor",
        },
        current_topology_read_family_coverage_contributor()?,
    )
    .map_err(|error| {
        TopologyTouchedGraphParityCoverageError::new(format!(
            "{} ({TOPOLOGY_READ_FAMILY_REPLACEMENT_LANE})",
            error.detail()
        ))
    })
}
