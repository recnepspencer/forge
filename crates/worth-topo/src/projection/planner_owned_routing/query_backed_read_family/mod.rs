mod admission_error;
mod admitted_route;
mod current_route;
mod digest;
mod residue_manifest;
mod reuse_posture;
mod route_input;
mod selected_route;
mod selected_route_authority;

#[cfg(test)]
mod tests;

pub use admitted_route::admit_topology_query_backed_consumer_cutover;
pub(crate) use admitted_route::admit_topology_query_backed_read_family_route;
pub(crate) use admitted_route::admit_topology_query_backed_read_family_route_with_selected_route_authority;
pub(crate) use current_route::current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides;
pub(crate) use current_route::current_topology_query_backed_read_family_artifacts;
#[cfg(test)]
pub(crate) use current_route::current_topology_query_backed_read_family_route_input;
pub use current_route::{
    admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority,
    current_topology_query_backed_consumer_cutover, TopologyQueryBackedConsumerCutoverCurrentError,
};
pub use residue_manifest::{
    current_query_backed_consumer_residue_manifest, QueryBackedConsumerResidueDisposition,
    QueryBackedConsumerResidueOwner, QueryBackedConsumerResidueRow,
};
pub use reuse_posture::TopologyReadModelReusePosture;
pub use selected_route::{
    TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerFamilyRow,
};
pub use selected_route_authority::TopologyQueryBackedReadFamilySelectedRouteAuthority;
