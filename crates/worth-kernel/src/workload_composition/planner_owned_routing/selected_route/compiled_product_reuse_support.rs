use crate::workload_composition::planner_owned_routing::{
    CompiledProductReusePlannerRoutePacket, PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
};
use topology::certification::TopologyMilestoneFifteenPlannerSeedSupport;
use worth_spatial::certification::SpatialMilestoneFifteenPlannerSeedSupport;

pub(super) fn require_matching_compiled_product_reuse_route_packet(
    route_packet: &CompiledProductReusePlannerRoutePacket,
    topology_support: &TopologyMilestoneFifteenPlannerSeedSupport,
    spatial_support: &SpatialMilestoneFifteenPlannerSeedSupport,
) -> Result<(), PlannerOwnedRoutingError> {
    if route_packet.selected_family_identity()
        != topology_support.selected_equivalence_family_identity()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet compiled-product reuse route does not match topology selected family identity",
        ));
    }
    if route_packet.selected_product_identity_digest()
        != spatial_support.compiled_product_identity_digest()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet compiled-product reuse route does not match spatial compiled product identity",
        ));
    }
    if route_packet.selected_reuse_basis_identity_digest()
        != topology_support.selected_reuse_basis_identity_digest()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet compiled-product reuse route does not match topology selected reuse basis identity",
        ));
    }
    Ok(())
}
