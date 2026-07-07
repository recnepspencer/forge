use super::batch_admission_support::require_matching_batch_admission_route_packet;
use super::compiled_product_reuse_support::require_matching_compiled_product_reuse_route_packet;
use std::sync::OnceLock;
use topology::certification::{
    current_topology_milestone_fifteen_planner_seed_support,
    TopologyMilestoneFifteenPlannerSeedSupport, TopologyPublicCloseoutSeedSupportError,
};
use topology::derived_invalidation_route_input::current_topology_invalidation_route_input;
use topology::facade::current_topology_query_backed_consumer_cutover;
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::certification::{
    current_spatial_milestone_fifteen_planner_seed_support,
    SpatialMilestoneFifteenPlannerSeedSupport, SpatialPublicCloseoutSeedSupportError,
};
use worth_spatial::facade::evidence_lookup_route::current_evidence_lookup_route_packet;

use super::builder::{
    build_packet, require_matching_replay_undo_route_packet, require_matching_spatial_support,
    require_matching_support,
};
use super::conflict_independence_support::require_matching_conflict_independence_route_packet;
use super::packet::WorthTouchedGraphConflictSelectedRoutePacket;
use super::SpatialRouteProjectionMarkers;
use crate::workload_composition::performance_trace::trace_scope;
use crate::workload_composition::planner_owned_routing::{
    batch_admission_route::current_worth_touched_graph_conflict_batch_admission_route_packet,
    compiled_product_reuse_route::current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    conflict_independence_route::{
        current_worth_touched_graph_conflict_independence_route_packet,
        ConflictIndependencePlannerRoutePacket,
    },
    current_worth_workload_ordinary_consumer_cutover,
    replay_undo_route::{
        current_replay_undo_transaction_route_packet, ReplayUndoPlannerRoutePacket,
    },
    BatchAdmissionPlannerRoutePacket, CompiledProductReusePlannerRoutePacket,
    PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
};
use crate::workload_composition::{
    current_worth_touched_graph_conflict_deletion_closeout,
    current_worth_touched_graph_conflict_source_firewall_report,
};

pub fn current_worth_touched_graph_conflict_selected_route_packet(
) -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError> {
    static CACHE: OnceLock<WorthTouchedGraphConflictSelectedRoutePacket> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let packet = current_worth_touched_graph_conflict_selected_route_packet_with_support_loaders(
        current_topology_milestone_fifteen_planner_seed_support,
        current_spatial_milestone_fifteen_planner_seed_support,
    )?;
    let _ = CACHE.set(packet.clone());
    Ok(packet)
}

pub(crate) fn current_worth_touched_graph_conflict_selected_route_packet_with_support_loaders<
    T,
    S,
>(
    load_topology_support: T,
    load_spatial_support: S,
) -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError>
where
    T: FnOnce() -> Result<
        TopologyMilestoneFifteenPlannerSeedSupport,
        TopologyPublicCloseoutSeedSupportError,
    >,
    S: FnOnce() -> Result<
        SpatialMilestoneFifteenPlannerSeedSupport,
        SpatialPublicCloseoutSeedSupportError,
    >,
{
    current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders(
        load_topology_support,
        load_spatial_support,
        current_replay_undo_transaction_route_packet,
        current_worth_touched_graph_conflict_batch_admission_route_packet,
        current_worth_touched_graph_conflict_independence_route_packet,
        current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    )
}

pub(crate) fn current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders<
    T,
    S,
    R,
    B,
    C,
    U,
>(
    load_topology_support: T,
    load_spatial_support: S,
    load_replay_undo_route: R,
    load_batch_admission_route: B,
    load_conflict_independence_route: C,
    load_compiled_product_reuse_route: U,
) -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError>
where
    T: FnOnce() -> Result<
        TopologyMilestoneFifteenPlannerSeedSupport,
        TopologyPublicCloseoutSeedSupportError,
    >,
    S: FnOnce() -> Result<
        SpatialMilestoneFifteenPlannerSeedSupport,
        SpatialPublicCloseoutSeedSupportError,
    >,
    R: FnOnce() -> Result<ReplayUndoPlannerRoutePacket, PlannerOwnedRoutingError>,
    B: FnOnce() -> Result<BatchAdmissionPlannerRoutePacket, PlannerOwnedRoutingError>,
    C: FnOnce() -> Result<ConflictIndependencePlannerRoutePacket, PlannerOwnedRoutingError>,
    U: FnOnce() -> Result<CompiledProductReusePlannerRoutePacket, PlannerOwnedRoutingError>,
{
    trace_scope(
        "current_worth_touched_graph_conflict_selected_route_packet",
        || {
            let cutover =
                trace_scope("selected_route_current_cutover", || {
                    current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
                        PlannerOwnedRoutingError::new(
                    PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                    format!("selected-route packet requires current ordinary cutover: {error:?}"),
                )
                    })
                })?;
            let topology_cutover = trace_scope("selected_route_topology_cutover", || {
                current_topology_query_backed_consumer_cutover().map_err(|error| {
                    PlannerOwnedRoutingError::new(
                        PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                        error.detail(),
                    )
                })
            })?;
            let topology_row = topology_cutover
                .family_rows()
                .iter()
                .find(|row| {
                    row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood
                })
                .ok_or_else(|| {
                    PlannerOwnedRoutingError::new(
                        PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
                        "selected-route packet requires the loop-cycle topology family row",
                    )
                })?;
            let topology_support = trace_scope("selected_route_topology_support", || {
                load_topology_support().map_err(|error| {
                    PlannerOwnedRoutingError::new(
                        PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                        format!(
                            "selected-route packet requires topology planner support: {}",
                            error.detail()
                        ),
                    )
                })
            })?;
            let spatial_support = trace_scope("selected_route_spatial_support", || {
                load_spatial_support().map_err(|error| {
                    PlannerOwnedRoutingError::new(
                        PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                        format!(
                            "selected-route packet requires spatial planner support: {}",
                            error.detail()
                        ),
                    )
                })
            })?;
            let invalidation_route_input =
                trace_scope("selected_route_topology_invalidation_route_input", || {
                    current_topology_invalidation_route_input().map_err(|error| {
                        PlannerOwnedRoutingError::new(
                            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                            format!(
                            "selected-route packet requires topology invalidation route input: {}",
                            error.detail()
                        ),
                        )
                    })
                })?;
            require_matching_support(topology_row, &topology_support)?;
            let spatial_route_packet = trace_scope("selected_route_spatial_route_packet", || {
                current_evidence_lookup_route_packet().map_err(|error| {
                    PlannerOwnedRoutingError::new(
                        PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                        format!(
                        "selected-route packet requires planner-owned evidence lookup route: {}",
                        error.detail()
                    ),
                    )
                })
            })?;
            require_matching_spatial_support(&spatial_route_packet, &spatial_support)?;
            let replay_undo_route_packet = trace_scope(
                "selected_route_replay_undo_route_packet",
                load_replay_undo_route,
            )?;
            require_matching_replay_undo_route_packet(&replay_undo_route_packet, &cutover)?;
            let batch_admission_route_packet = trace_scope(
                "selected_route_batch_admission_route_packet",
                load_batch_admission_route,
            )?;
            require_matching_batch_admission_route_packet(
                &batch_admission_route_packet,
                cutover.batch_execution_receipt(),
            )?;
            let conflict_independence_route_packet = trace_scope(
                "selected_route_conflict_independence_route_packet",
                load_conflict_independence_route,
            )?;
            require_matching_conflict_independence_route_packet(
                &conflict_independence_route_packet,
                cutover.batch_execution_receipt(),
            )?;
            let compiled_product_reuse_route_packet = trace_scope(
                "selected_route_compiled_product_reuse_route_packet",
                load_compiled_product_reuse_route,
            )?;
            require_matching_compiled_product_reuse_route_packet(
                &compiled_product_reuse_route_packet,
                &topology_support,
                &spatial_support,
            )?;
            let spatial_route_projection_markers =
                SpatialRouteProjectionMarkers::from_route_packet(&spatial_route_packet);
            let source_firewall_digest =
                trace_scope("selected_route_source_firewall_report", || {
                    current_worth_touched_graph_conflict_source_firewall_report()
                        .map_err(|error| {
                            PlannerOwnedRoutingError::new(
                                PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                                format!(
                            "selected-route packet requires source firewall report: {error:?}"
                        ),
                            )
                        })
                        .map(|report| report.report_digest().to_string())
                })?;
            let deletion_closeout_digest = trace_scope("selected_route_deletion_closeout", || {
                current_worth_touched_graph_conflict_deletion_closeout()
                    .map_err(|error| {
                        PlannerOwnedRoutingError::new(
                            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                            format!("selected-route packet requires deletion closeout: {error:?}"),
                        )
                    })
                    .map(|closeout| closeout.closeout_digest().to_string())
            })?;

            trace_scope("selected_route_build_packet", || {
                build_packet(
                    &cutover,
                    &replay_undo_route_packet,
                    batch_admission_route_packet,
                    conflict_independence_route_packet,
                    compiled_product_reuse_route_packet,
                    &spatial_route_projection_markers,
                    &topology_cutover,
                    topology_row,
                    &topology_support,
                    &spatial_support,
                    &invalidation_route_input,
                    &spatial_route_packet,
                    source_firewall_digest,
                    deletion_closeout_digest,
                )
            })
        },
    )
}
