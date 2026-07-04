#[cfg(any(test, feature = "test-support-lowering"))]
pub use crate::projection::query_backed_consumer_cutover::TopologyQueryBackedReadFamilySelectedRouteAuthority;

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) use crate::projection::query_backed_consumer_cutover::require_selected_route_authority_matches;
