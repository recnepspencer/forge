mod alignment_summary;
mod planner_seed_support;

#[cfg(any(test, feature = "test-support-lowering"))]
pub use crate::projection::query_backed_consumer_cutover::{
    admit_current_topology_query_backed_consumer_cutover_with_selected_route_authority,
    TopologyQueryBackedReadFamilySelectedRouteAuthority,
};
pub use alignment_summary::{
    current_topology_public_closeout_alignment_summary, TopologyPublicCloseoutAlignmentSummary,
    TopologyPublicCloseoutFreshnessRequirementPosture,
    TopologyPublicCloseoutRenderedOutputComparisonPosture, TopologyPublicCloseoutSeedSupportError,
};
pub use planner_seed_support::{
    current_topology_milestone_fifteen_planner_seed_support,
    current_topology_milestone_fifteen_planner_seed_support_with_hostile_selected_reuse_basis_identity_digest,
    current_topology_query_backed_consumer_cutover_with_hostile_loop_cycle_selected_compatibility_basis,
    TopologyMilestoneFifteenPlannerSeedSupport,
};
