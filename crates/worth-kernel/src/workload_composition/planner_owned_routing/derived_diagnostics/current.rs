use topology::derived_read_diagnostic_input::{
    support::{
        current_topology_derived_read_diagnostic_input_with_selected_route_authority,
        TopologyDerivedReadDiagnosticInputCurrentError,
        TopologyDerivedReadDiagnosticSelectedRouteAuthority,
    },
    TopologyDerivedReadDiagnosticInput,
};

#[cfg(test)]
use crate::workload_composition::planner_owned_routing::selected_route::WorthTouchedGraphConflictSelectedRoutePacket;
use crate::workload_composition::planner_owned_routing::{
    current_worth_touched_graph_conflict_selected_route_packet, PlannerOwnedRoutingError,
    PlannerOwnedRoutingErrorKind,
};

pub fn current_worth_touched_graph_conflict_derived_read_diagnostic_input(
) -> Result<TopologyDerivedReadDiagnosticInput, PlannerOwnedRoutingError> {
    build_current_worth_touched_graph_conflict_derived_read_diagnostic_input(
        current_worth_touched_graph_conflict_selected_route_packet()?,
    )
}

#[cfg(test)]
pub(crate) fn current_worth_touched_graph_conflict_derived_read_diagnostic_input_with_packet_loader(
    packet_loader: impl FnOnce() -> Result<
        WorthTouchedGraphConflictSelectedRoutePacket,
        PlannerOwnedRoutingError,
    >,
) -> Result<TopologyDerivedReadDiagnosticInput, PlannerOwnedRoutingError> {
    build_current_worth_touched_graph_conflict_derived_read_diagnostic_input(packet_loader()?)
}

fn build_current_worth_touched_graph_conflict_derived_read_diagnostic_input(
    selected_route_packet: crate::workload_composition::planner_owned_routing::selected_route::WorthTouchedGraphConflictSelectedRoutePacket,
) -> Result<TopologyDerivedReadDiagnosticInput, PlannerOwnedRoutingError> {
    let diagnostic_authority =
        TopologyDerivedReadDiagnosticSelectedRouteAuthority::from(&selected_route_packet);
    current_topology_derived_read_diagnostic_input_with_selected_route_authority(
        &diagnostic_authority,
    )
    .map_err(current_diagnostic_error)
}

fn current_diagnostic_error(
    error: TopologyDerivedReadDiagnosticInputCurrentError,
) -> PlannerOwnedRoutingError {
    PlannerOwnedRoutingError::new(
        PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
        error.detail(),
    )
}
