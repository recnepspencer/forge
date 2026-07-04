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

pub(crate) use admission_error::{
    require_optional_match, require_string_match, TopologyQueryBackedReadFamilyAdmissionError,
};
pub use admitted_route::admit_topology_query_backed_consumer_cutover;
pub(crate) use admitted_route::admit_topology_query_backed_read_family_route;
#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) use admitted_route::admit_topology_query_backed_read_family_route_with_selected_route_authority;
#[cfg(any(test, feature = "test-support-lowering"))]
pub use current_route::admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority;
pub use current_route::current_topology_query_backed_consumer_cutover;
pub(crate) use current_route::current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides;
pub(crate) use current_route::current_topology_query_backed_read_family_artifacts;
pub(crate) use current_route::current_topology_query_backed_read_family_route_input;
pub(crate) use current_route::current_topology_query_backed_read_family_route_input_with_hostile_selected_basis_overrides;
pub(crate) use current_route::TopologyQueryBackedConsumerCutoverCurrentError;
pub(crate) use digest::{closeout_digest, family_row_digest};
pub use residue_manifest::{
    current_query_backed_consumer_residue_manifest, QueryBackedConsumerResidueDisposition,
    QueryBackedConsumerResidueOwner, QueryBackedConsumerResidueRow,
};
pub use reuse_posture::TopologyReadModelReusePosture;
pub(crate) use route_input::TopologyObservedQueryBackedReadFamilyRow;
pub(crate) use route_input::TopologyQueryBackedReadFamilyRouteInput;
pub use selected_route::{
    TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerFamilyRow,
};
#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) use selected_route_authority::require_selected_route_authority_matches;
#[cfg(any(test, feature = "test-support-lowering"))]
pub use selected_route_authority::TopologyQueryBackedReadFamilySelectedRouteAuthority;
