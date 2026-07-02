use super::batch_admission_support::require_matching_batch_admission_route_packet;
use super::compiled_product_reuse_support::require_matching_compiled_product_reuse_route_packet;
use topology::certification::{
    current_topology_milestone_fifteen_planner_seed_support,
    TopologyMilestoneFifteenPlannerSeedSupport, TopologyPublicCloseoutSeedSupportError,
};
use topology::facade::{
    current_topology_query_backed_consumer_cutover, TopologyQueryBackedConsumerFamilyRow,
};
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::certification::{
    current_spatial_milestone_fifteen_planner_seed_support,
    SpatialMilestoneFifteenPlannerSeedSupport, SpatialPublicCloseoutSeedSupportError,
};
use worth_spatial::facade::planner_owned_routing::evidence_lookup_route::current_evidence_lookup_route_packet;

use super::conflict_independence_support::require_matching_conflict_independence_route_packet;
use super::packet::WorthTouchedGraphConflictSelectedRoutePacket;
use super::SpatialRouteProjectionMarkers;
use crate::workload_composition::planner_owned_routing::{
    batch_admission_route::current_worth_touched_graph_conflict_batch_admission_route_packet,
    compiled_product_reuse_route::current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    conflict_independence_route::{
        current_worth_touched_graph_conflict_independence_route_packet,
        ConflictIndependencePlannerRoutePacket,
    },
    replay_undo_route::{
        current_replay_undo_transaction_route_packet, ReplayUndoPlannerRoutePacket,
    },
    BatchAdmissionPlannerRoutePacket, CompiledProductReusePlannerRoutePacket,
    PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
};
use crate::workload_composition::worth_workload::{
    current_worth_workload_ordinary_consumer_cutover, WorthWorkloadOrdinaryConsumerCutover,
    WorthWorkloadOrdinaryConsumerCutoverPosture,
};
use crate::workload_composition::{
    current_worth_touched_graph_conflict_deletion_closeout,
    current_worth_touched_graph_conflict_source_firewall_report,
};

pub fn current_worth_touched_graph_conflict_selected_route_packet(
) -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError> {
    current_worth_touched_graph_conflict_selected_route_packet_with_support_loaders(
        current_topology_milestone_fifteen_planner_seed_support,
        current_spatial_milestone_fifteen_planner_seed_support,
    )
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
    let cutover = current_worth_workload_ordinary_consumer_cutover().map_err(|error| {
        PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            format!("selected-route packet requires current ordinary cutover: {error:?}"),
        )
    })?;
    let topology_cutover = current_topology_query_backed_consumer_cutover().map_err(|error| {
        PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            error.detail(),
        )
    })?;
    let topology_row = topology_cutover
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
        .ok_or_else(|| {
            PlannerOwnedRoutingError::new(
                PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
                "selected-route packet requires the loop-cycle topology family row",
            )
        })?;
    let topology_support = load_topology_support().map_err(|error| {
        PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            format!(
                "selected-route packet requires topology planner support: {}",
                error.detail()
            ),
        )
    })?;
    let spatial_support = load_spatial_support().map_err(|error| {
        PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            format!(
                "selected-route packet requires spatial planner support: {}",
                error.detail()
            ),
        )
    })?;
    require_matching_support(topology_row, &topology_support)?;
    let spatial_route_packet = current_evidence_lookup_route_packet().map_err(|error| {
        PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            format!(
                "selected-route packet requires planner-owned evidence lookup route: {}",
                error.detail()
            ),
        )
    })?;
    require_matching_spatial_support(&spatial_route_packet, &spatial_support)?;
    let replay_undo_route_packet = load_replay_undo_route()?;
    require_matching_replay_undo_route_packet(&replay_undo_route_packet, &cutover)?;
    let batch_admission_route_packet = load_batch_admission_route()?;
    require_matching_batch_admission_route_packet(
        &batch_admission_route_packet,
        cutover.batch_execution_receipt(),
    )?;
    let conflict_independence_route_packet = load_conflict_independence_route()?;
    require_matching_conflict_independence_route_packet(
        &conflict_independence_route_packet,
        cutover.batch_execution_receipt(),
    )?;
    let compiled_product_reuse_route_packet = load_compiled_product_reuse_route()?;
    require_matching_compiled_product_reuse_route_packet(
        &compiled_product_reuse_route_packet,
        &topology_support,
        &spatial_support,
    )?;
    let spatial_route_projection_markers =
        SpatialRouteProjectionMarkers::from_route_packet(&spatial_route_packet);
    let source_firewall_digest = current_worth_touched_graph_conflict_source_firewall_report()
        .map_err(|error| {
            PlannerOwnedRoutingError::new(
                PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                format!("selected-route packet requires source firewall report: {error:?}"),
            )
        })?
        .report_digest()
        .to_string();
    let deletion_closeout_digest = current_worth_touched_graph_conflict_deletion_closeout()
        .map_err(|error| {
            PlannerOwnedRoutingError::new(
                PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                format!("selected-route packet requires deletion closeout: {error:?}"),
            )
        })?
        .closeout_digest()
        .to_string();

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
        source_firewall_digest,
        deletion_closeout_digest,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_packet(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    replay_undo_route_packet: &ReplayUndoPlannerRoutePacket,
    batch_admission_route_packet: BatchAdmissionPlannerRoutePacket,
    conflict_independence_route_packet: ConflictIndependencePlannerRoutePacket,
    compiled_product_reuse_route_packet: CompiledProductReusePlannerRoutePacket,
    spatial_route_projection_markers: &SpatialRouteProjectionMarkers,
    topology_cutover: &topology::facade::TopologyQueryBackedConsumerCutover,
    topology_row: &TopologyQueryBackedConsumerFamilyRow,
    topology_support: &TopologyMilestoneFifteenPlannerSeedSupport,
    spatial_support: &SpatialMilestoneFifteenPlannerSeedSupport,
    source_firewall_digest: String,
    deletion_closeout_digest: String,
) -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError> {
    let selected_plan_witnesses = cutover
        .rows()
        .iter()
        .filter(|row| {
            row.posture()
                == WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
        })
        .map(|row| {
            row.selected_plan_witness().ok_or_else(|| {
                PlannerOwnedRoutingError::new(
                    PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
                    format!(
                        "selected-route packet requires selected-plan witness for `{}`",
                        row.surface_name()
                    ),
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    if selected_plan_witnesses.is_empty() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
            "selected-route packet requires at least one selected-plan witness",
        ));
    }

    Ok(WorthTouchedGraphConflictSelectedRoutePacket::new(
        selected_plan_witnesses
            .iter()
            .map(|witness| witness.route_authority_digest())
            .map(str::to_string)
            .collect(),
        selected_plan_witnesses
            .iter()
            .map(|witness| witness.route_lineage_digest())
            .map(str::to_string)
            .collect(),
        conflict_independence_route_packet
            .overlap_identity_digests()
            .to_vec(),
        conflict_independence_route_packet
            .locality_footprint_digests()
            .to_vec(),
        conflict_independence_route_packet
            .selected_conflict_plan_digests()
            .to_vec(),
        conflict_independence_route_packet
            .independence_proof_identities()
            .to_vec(),
        batch_admission_route_packet
            .selected_batch_plan_digest()
            .to_string(),
        batch_admission_route_packet
            .batch_execution_receipt_digest()
            .to_string(),
        cutover.replay_undo_boundary_proof_digests(),
        replay_undo_route_packet.route_packet_identity().to_string(),
        replay_undo_route_packet.family(),
        batch_admission_route_packet,
        conflict_independence_route_packet,
        compiled_product_reuse_route_packet,
        cutover.transaction_packet_identities(),
        cutover.replay_scope_identities(),
        cutover.undo_scope_identities(),
        spatial_route_projection_markers
            .evidence_lookup_public_closeout_digest()
            .to_string(),
        spatial_route_projection_markers
            .evidence_lookup_family_coverage_digest()
            .to_string(),
        spatial_route_projection_markers
            .evidence_lookup_query_surface_matrix_digest()
            .to_string(),
        spatial_route_projection_markers
            .evidence_lookup_query_consumer_kit_digest()
            .to_string(),
        spatial_route_projection_markers
            .evidence_lookup_query_boundary_support_digest()
            .to_string(),
        topology_cutover.closeout_digest().to_string(),
        topology_row.row_digest().to_string(),
        topology_cutover.handle_identity_digest().to_string(),
        topology_cutover
            .operating_context_identity_digest()
            .to_string(),
        topology_cutover.support_snapshot_digest().to_string(),
        topology_cutover.parity_verified_count(),
        topology_row
            .compiled_product_identity_digest()
            .expect("typed topology row"),
        topology_row
            .equivalence_policy_identity_digest()
            .expect("typed topology row"),
        topology_support,
        spatial_support,
        source_firewall_digest,
        deletion_closeout_digest,
    ))
}

fn require_matching_replay_undo_route_packet(
    route_packet: &ReplayUndoPlannerRoutePacket,
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
) -> Result<(), PlannerOwnedRoutingError> {
    if !cutover
        .transaction_packet_identities()
        .iter()
        .any(|identity| identity == route_packet.transaction_boundary_packet().packet_identity())
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet replay/undo route packet does not match the current ordinary cutover transaction packet",
        ));
    }
    if !cutover.replay_scope_identities().iter().any(|identity| {
        identity
            == route_packet
                .transaction_boundary_packet()
                .replay_scope_identity()
                .digest()
    }) {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet replay/undo route packet does not match the current ordinary cutover replay scope",
        ));
    }
    if !cutover.undo_scope_identities().iter().any(|identity| {
        identity
            == route_packet
                .transaction_boundary_packet()
                .undo_scope_identity()
                .digest()
    }) {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet replay/undo route packet does not match the current ordinary cutover undo scope",
        ));
    }
    Ok(())
}

fn require_matching_support(
    topology_row: &TopologyQueryBackedConsumerFamilyRow,
    topology_support: &TopologyMilestoneFifteenPlannerSeedSupport,
) -> Result<(), PlannerOwnedRoutingError> {
    let row_family = topology_row
        .selected_equivalence_family_identity()
        .ok_or_else(|| {
            PlannerOwnedRoutingError::new(
                PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
                "selected-route packet requires selected equivalence family on topology row",
            )
        })?;
    let row_reuse_basis = topology_row
        .selected_reuse_basis_identity_digest()
        .ok_or_else(|| {
            PlannerOwnedRoutingError::new(
                PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
                "selected-route packet requires selected reuse basis on topology row",
            )
        })?;
    if row_family != topology_support.selected_equivalence_family_identity()
        || row_reuse_basis != topology_support.selected_reuse_basis_identity_digest()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            format!(
                "selected-route packet support mismatch: row_family={row_family}, topo_family={}, row_reuse_basis={row_reuse_basis}, topo_reuse_basis={}",
                topology_support.selected_equivalence_family_identity(),
                topology_support.selected_reuse_basis_identity_digest(),
            ),
        ));
    }
    Ok(())
}

fn require_matching_spatial_support(
    route_packet: &worth_spatial::facade::planner_owned_routing::evidence_lookup_route::EvidenceLookupRoutePacket,
    spatial_support: &SpatialMilestoneFifteenPlannerSeedSupport,
) -> Result<(), PlannerOwnedRoutingError> {
    if route_packet.selected_equivalence_family_identity()
        != spatial_support.selected_equivalence_family_identity()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            format!(
                "selected-route packet support mismatch: spatial_selected_family={}, spatial_support_family={}",
                route_packet.selected_equivalence_family_identity(),
                spatial_support.selected_equivalence_family_identity(),
            ),
        ));
    }
    if route_packet.compiled_product_identity_digest()
        != spatial_support.compiled_product_identity_digest()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            format!(
                "selected-route packet support mismatch: spatial_compiled_product={}, spatial_support_compiled_product={}",
                route_packet.compiled_product_identity_digest(),
                spatial_support.compiled_product_identity_digest(),
            ),
        ));
    }
    if route_packet.equivalence_policy_identity_digest()
        != spatial_support.equivalence_policy_identity_digest()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            format!(
                "selected-route packet support mismatch: spatial_equivalence_policy={}, spatial_support_equivalence_policy={}",
                route_packet.equivalence_policy_identity_digest(),
                spatial_support.equivalence_policy_identity_digest(),
            ),
        ));
    }
    Ok(())
}
