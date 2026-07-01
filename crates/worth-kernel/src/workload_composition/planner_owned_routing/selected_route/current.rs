use topology::certification::{
    current_topology_milestone_fifteen_planner_seed_support,
    TopologyMilestoneFifteenPlannerSeedSupport, TopologyPublicCloseoutSeedSupportError,
};
use topology::facade::{current_topology_query_backed_consumer_cutover, TopologyQueryBackedConsumerFamilyRow};
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::certification::{
    current_spatial_milestone_fifteen_planner_seed_support,
    SpatialMilestoneFifteenPlannerSeedSupport, SpatialPublicCloseoutSeedSupportError,
};
use worth_spatial::facade::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout, EvidenceLookupPublicCloseout,
};

use super::packet::WorthTouchedGraphConflictSelectedRoutePacket;
use crate::workload_composition::planner_owned_routing::{
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

pub(crate) fn current_worth_touched_graph_conflict_selected_route_packet_with_support_loaders<T, S>(
    load_topology_support: T,
    load_spatial_support: S,
) -> Result<WorthTouchedGraphConflictSelectedRoutePacket, PlannerOwnedRoutingError>
where
    T: FnOnce() -> Result<TopologyMilestoneFifteenPlannerSeedSupport, TopologyPublicCloseoutSeedSupportError>,
    S: FnOnce() -> Result<SpatialMilestoneFifteenPlannerSeedSupport, SpatialPublicCloseoutSeedSupportError>,
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
            format!("selected-route packet requires topology planner support: {}", error.detail()),
        )
    })?;
    let spatial_support = load_spatial_support().map_err(|error| {
        PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            format!("selected-route packet requires spatial planner support: {}", error.detail()),
        )
    })?;
    require_matching_support(topology_row, &topology_support)?;
    let lookup_public_closeout = current_evidence_lookup_public_closeout().map_err(|error| {
        PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            format!("selected-route packet requires evidence lookup public closeout: {error:?}"),
        )
    })?;
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
        &lookup_public_closeout,
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
    lookup_public_closeout: &EvidenceLookupPublicCloseout,
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
        .filter(|row| row.posture() == WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer)
        .map(|row| {
            row.selected_plan_witness().ok_or_else(|| {
                PlannerOwnedRoutingError::new(
                    PlannerOwnedRoutingErrorKind::IncompleteSelectedRoutePacket,
                    format!("selected-route packet requires selected-plan witness for `{}`", row.surface_name()),
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
        selected_plan_witnesses.iter().map(|witness| witness.route_authority_digest()).map(str::to_string).collect(),
        selected_plan_witnesses.iter().map(|witness| witness.route_lineage_digest()).map(str::to_string).collect(),
        cutover.batch_execution_receipt().overlap_identity_digests().to_vec(),
        cutover.batch_execution_receipt().locality_footprint_digests().to_vec(),
        cutover.batch_execution_receipt().selected_conflict_plan_digests().to_vec(),
        cutover.batch_execution_receipt().independence_proof_identities().to_vec(),
        cutover.batch_execution_receipt().selected_batch_plan_digest().to_string(),
        cutover.batch_execution_receipt().execution_receipt_digest().to_string(),
        cutover.replay_undo_boundary_proof_digests(),
        cutover.transaction_packet_identities(),
        cutover.replay_scope_identities(),
        cutover.undo_scope_identities(),
        lookup_public_closeout.closeout_digest().to_string(),
        lookup_public_closeout.family_coverage_digest().to_string(),
        lookup_public_closeout.query_surface_matrix().matrix_digest().to_string(),
        lookup_public_closeout.query_consumer_kit().closeout_digest().to_string(),
        lookup_public_closeout.query_boundary_support_digest().to_string(),
        topology_cutover.closeout_digest().to_string(),
        topology_row.row_digest().to_string(),
        topology_cutover.handle_identity_digest().to_string(),
        topology_cutover.operating_context_identity_digest().to_string(),
        topology_cutover.support_snapshot_digest().to_string(),
        topology_row.compiled_product_identity_digest().expect("typed topology row"),
        topology_row.equivalence_policy_identity_digest().expect("typed topology row"),
        topology_support,
        spatial_support,
        source_firewall_digest,
        deletion_closeout_digest,
    ))
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
