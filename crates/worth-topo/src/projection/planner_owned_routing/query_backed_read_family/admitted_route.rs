pub use crate::projection::query_backed_consumer_cutover::admit_topology_query_backed_consumer_cutover;
pub(crate) use crate::projection::query_backed_consumer_cutover::{
    admit_topology_query_backed_read_family_route, TopologyQueryBackedReadFamilyAdmissionAuthority,
};

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) use crate::projection::query_backed_consumer_cutover::admit_topology_query_backed_read_family_route_with_selected_route_authority;
