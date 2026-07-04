use topology::certification::TopologyMilestoneFifteenPlannerSeedSupport;
use topology::derived_invalidation_route_input::TopologyInvalidationRouteInput;
use topology::facade::TopologyQueryBackedConsumerFamilyRow;
use worth_spatial::certification::SpatialMilestoneFifteenPlannerSeedSupport;
use worth_spatial::facade::evidence_lookup_route::EvidenceLookupRoutePacket;

use super::packet::WorthTouchedGraphConflictSelectedRoutePacket;
use super::SpatialRouteProjectionMarkers;
use crate::workload_composition::planner_owned_routing::{
    BatchAdmissionPlannerRoutePacket, CompiledProductReusePlannerRoutePacket,
    ConflictIndependencePlannerRoutePacket, PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
    ReplayUndoPlannerRoutePacket, WorthWorkloadOrdinaryConsumerCutover,
    WorthWorkloadOrdinaryConsumerCutoverPosture,
};

pub(super) fn require_matching_replay_undo_route_packet(
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

pub(super) fn require_matching_support(
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

pub(super) fn require_matching_spatial_support(
    route_packet: &EvidenceLookupRoutePacket,
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

#[allow(clippy::too_many_arguments)]
pub(super) fn build_packet(
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
    invalidation_route_input: &TopologyInvalidationRouteInput,
    spatial_route_packet: &EvidenceLookupRoutePacket,
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
        spatial_route_packet.query_support_digest().to_string(),
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
        invalidation_route_input,
        source_firewall_digest,
        deletion_closeout_digest,
    ))
}
