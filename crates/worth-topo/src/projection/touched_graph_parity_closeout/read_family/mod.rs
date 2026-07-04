mod declaration_row;

use crate::projection::query_backed_consumer_cutover::current_topology_query_backed_consumer_cutover;
use crate::projection::read_views::domain::TopologyReadRequestFamily;
use crate::projection::touched_graph_parity_closeout::{
    TopologyTouchedGraphParityCoverageContributor, TopologyTouchedGraphParityCoverageError,
    TopologyTouchedGraphParityQuerySurfaceKind,
};

pub use declaration_row::current_topology_read_declaration_row;

pub const TOPOLOGY_READ_FAMILY_CLAIM_PATH: &str =
    "crates/worth-topo/src/projection/touched_graph_parity_closeout/read_family/mod.rs";
pub const TOPOLOGY_READ_FAMILY_REPLACEMENT_LANE: &str =
    "crates/worth-topo/src/projection/touched_graph_parity_closeout/read_family/";

const SOURCE_PATH: &str =
    "crates/worth-topo/src/projection/query_backed_consumer_cutover/current_closeout.rs";
const SELECTED_FIELDS: &[&str] = &[
    "support_snapshot_digest",
    "family_rows.loop_cycle_neighborhood.selected_equivalence_family_identity",
    "family_rows.loop_cycle_neighborhood.compiled_product_identity_digest",
    "family_rows.loop_cycle_neighborhood.equivalence_policy_identity_digest",
    "family_rows.loop_cycle_neighborhood.selected_reuse_basis_identity_digest",
];

pub fn current_topology_read_family_coverage_contributor(
) -> Result<TopologyTouchedGraphParityCoverageContributor, TopologyTouchedGraphParityCoverageError>
{
    let cutover = current_topology_query_backed_consumer_cutover()
        .map_err(|error| TopologyTouchedGraphParityCoverageError::new(error.detail()))?;
    let row = cutover
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
        .ok_or_else(|| {
            TopologyTouchedGraphParityCoverageError::new(
                "read-family coverage contributor requires the live loop-cycle query-backed family row",
            )
        })?;
    if cutover.support_snapshot_digest().is_empty()
        || row.selected_equivalence_family_identity().is_none()
        || row.compiled_product_identity_digest().is_none()
        || row.equivalence_policy_identity_digest().is_none()
        || row.selected_reuse_basis_identity_digest().is_none()
    {
        return Err(TopologyTouchedGraphParityCoverageError::new(
            "read-family coverage contributor requires live support snapshot, compiled product, equivalence policy, selected family, and reuse basis identities",
        ));
    }

    Ok(TopologyTouchedGraphParityCoverageContributor::new(
        "current_topology_query_backed_consumer_cutover::loop_cycle_neighborhood",
        SOURCE_PATH,
        "current_topology_query_backed_consumer_cutover",
        "current_topology_query_backed_consumer_cutover",
        "family_route_product",
        TOPOLOGY_READ_FAMILY_REPLACEMENT_LANE,
        SELECTED_FIELDS,
        TopologyTouchedGraphParityQuerySurfaceKind::SupportPosture,
        "current_worth_touched_graph_conflict_selected_route_packet",
        "crates/worth-kernel/src/workload_composition/planner_owned_routing/selected_route/current.rs",
    ))
}
