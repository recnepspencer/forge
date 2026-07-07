mod admission;
mod admission_error;
mod closeout;
mod current_closeout;
mod read_model_reuse_posture;
mod residue_manifest;
mod route_input;
mod selected_route_authority;
#[cfg(test)]
mod tests;
mod types;

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) use admission::admit_topology_query_backed_read_family_route_with_selected_route_authority;
pub(crate) use admission::admit_topology_query_backed_read_family_route;
#[cfg(test)]
pub(crate) use admission::TopologyQueryBackedReadFamilyAdmissionAuthority;
pub use closeout::{
    admit_topology_query_backed_consumer_cutover, TopologyQueryBackedConsumerCutover,
    TopologyQueryBackedConsumerFamilyRow,
};
#[cfg(any(test, feature = "test-support-lowering"))]
pub use current_closeout::admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority;
pub(crate) use current_closeout::current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides;
pub use current_closeout::{
    current_topology_query_backed_consumer_cutover, TopologyQueryBackedConsumerCutoverCurrentError,
};
#[cfg(test)]
pub(crate) use current_closeout::{
    current_topology_query_backed_read_family_route_input,
};
pub use read_model_reuse_posture::TopologyReadModelReusePosture;
pub use residue_manifest::{
    current_query_backed_consumer_residue_manifest, QueryBackedConsumerResidueDisposition,
    QueryBackedConsumerResidueOwner, QueryBackedConsumerResidueRow,
};
pub(crate) use route_input::{
    TopologyObservedQueryBackedReadFamilyRow, TopologyQueryBackedReadFamilyRouteInput,
};
#[cfg(any(test, feature = "test-support-lowering"))]
pub use selected_route_authority::TopologyQueryBackedReadFamilySelectedRouteAuthority;
