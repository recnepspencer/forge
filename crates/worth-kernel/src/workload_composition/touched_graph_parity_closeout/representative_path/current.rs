use topology::facade::current_topology_query_backed_consumer_cutover;
use worth_spatial::facade::evidence_lookup_route::current_evidence_lookup_route_packet;

use crate::workload_composition::planner_owned_routing::{
    current_replay_undo_transaction_route_packet,
    current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    current_worth_touched_graph_conflict_public_facade,
    current_worth_touched_graph_conflict_selected_route_packet,
};

use super::builder::build_representative_selected_route_parity_path;
use super::path::{
    RepresentativeSelectedRouteParityPath, RepresentativeSelectedRouteParityPathError,
    RepresentativeSelectedRouteParityPathErrorKind,
};

pub fn current_representative_selected_route_parity_path(
) -> Result<RepresentativeSelectedRouteParityPath, RepresentativeSelectedRouteParityPathError> {
    let selected_route_packet = current_worth_touched_graph_conflict_selected_route_packet()
        .map_err(|error| {
            RepresentativeSelectedRouteParityPathError::new(
                RepresentativeSelectedRouteParityPathErrorKind::CurrentSelectedRouteUnavailable,
                error.detail(),
            )
        })?;
    let public_facade = current_worth_touched_graph_conflict_public_facade().map_err(|error| {
        RepresentativeSelectedRouteParityPathError::new(
            RepresentativeSelectedRouteParityPathErrorKind::CurrentPublicFacadeUnavailable,
            error.detail(),
        )
    })?;
    let query_cutover = current_topology_query_backed_consumer_cutover().map_err(|error| {
        RepresentativeSelectedRouteParityPathError::new(
            RepresentativeSelectedRouteParityPathErrorKind::CurrentQueryBackedReadUnavailable,
            error.detail(),
        )
    })?;
    let evidence_route = current_evidence_lookup_route_packet().map_err(|error| {
        RepresentativeSelectedRouteParityPathError::new(
            RepresentativeSelectedRouteParityPathErrorKind::CurrentEvidenceLookupUnavailable,
            error.detail(),
        )
    })?;
    let replay_route = current_replay_undo_transaction_route_packet().map_err(|error| {
        RepresentativeSelectedRouteParityPathError::new(
            RepresentativeSelectedRouteParityPathErrorKind::CurrentReplayUnavailable,
            error.detail(),
        )
    })?;
    let reuse_route = current_worth_touched_graph_conflict_compiled_product_reuse_route_packet()
        .map_err(|error| {
            RepresentativeSelectedRouteParityPathError::new(
                RepresentativeSelectedRouteParityPathErrorKind::CurrentReuseUnavailable,
                error.detail(),
            )
        })?;
    representative_selected_route_parity_path_from_authorities(
        selected_route_packet,
        public_facade,
        query_cutover,
        evidence_route,
        replay_route,
        reuse_route,
    )
}

pub(crate) fn representative_selected_route_parity_path_from_authorities(
    selected_route_packet: crate::workload_composition::WorthTouchedGraphConflictSelectedRoutePacket,
    public_facade: crate::workload_composition::WorthTouchedGraphConflictPublicFacade,
    query_cutover: topology::facade::TopologyQueryBackedConsumerCutover,
    evidence_route: worth_spatial::facade::evidence_lookup_route::EvidenceLookupRoutePacket,
    replay_route: crate::workload_composition::planner_owned_routing::ReplayUndoPlannerRoutePacket,
    reuse_route: crate::workload_composition::planner_owned_routing::CompiledProductReusePlannerRoutePacket,
) -> Result<RepresentativeSelectedRouteParityPath, RepresentativeSelectedRouteParityPathError> {
    build_representative_selected_route_parity_path(
        selected_route_packet,
        public_facade,
        query_cutover,
        evidence_route,
        replay_route,
        reuse_route,
    )
}
