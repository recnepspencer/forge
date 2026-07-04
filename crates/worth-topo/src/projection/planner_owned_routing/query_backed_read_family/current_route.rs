#[cfg(any(test, feature = "test-support-lowering"))]
pub use crate::projection::query_backed_consumer_cutover::admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority;
pub use crate::projection::query_backed_consumer_cutover::current_topology_query_backed_consumer_cutover;
pub(crate) use crate::projection::query_backed_consumer_cutover::{
    current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides,
    current_topology_query_backed_read_family_artifacts,
    current_topology_query_backed_read_family_route_input,
    current_topology_query_backed_read_family_route_input_with_hostile_selected_basis_overrides,
    TopologyQueryBackedConsumerCutoverCurrentError,
};
