use crate::workload_platform::{
    boolean_readiness_workload::PlanarBooleanReadinessWorkloadReceipt,
    coplanar_overlap_storm::CoplanarOverlapStormReceipt,
    dirty_planar_clean_fail::DirtyPlanarCleanFailReceipt,
    grazing_basket_stack::GrazingBasketStackReceipt,
    high_valence_singularity::HighValenceSingularityReceipt,
    mixed_surface_kill_box::MixedSurfaceKillBoxReceipt, nmt_radial_fan::NmtRadialFanReceipt,
    open_class_triad_parity::OpenClassTriadParityReceipt,
    open_planar_posture::OpenPlanarPostureReceipt,
    projection_fact_parity::ProjectionFactParityReceipt,
    retained_cancellation_chain::RetainedCancellationChainReceipt,
    thin_feature_scale_separation::ThinFeatureScaleSeparationReceipt,
};

pub(super) fn coplanar_overlap_storm_source_rows(receipt: &CoplanarOverlapStormReceipt) -> usize {
    let counters = receipt.counters();
    counters.topology_entity_count()
        + counters.topology_face_count()
        + counters.topology_relation_count()
        + counters.projected_entity_count()
        + counters.transform_step_count()
        + counters.transform_cancellation_step_count()
        + counters.retained_artifact_count()
        + counters.replay_checkpoint_count()
        + counters.operator_input_count()
        + counters.operator_receipt_count()
        + counters.overlap_extraction_receipt_count()
        + counters.overlap_candidate_pair_breadth()
        + counters.overlap_segment_contacts_certified()
        + counters.overlap_shared_intervals()
        + counters.overlap_islands()
        + counters.overlap_ambiguous_contacts()
}

pub(super) fn high_valence_source_rows(receipt: &HighValenceSingularityReceipt) -> usize {
    let counters = receipt.counters();
    counters.topology_entity_count()
        + counters.topology_face_count()
        + counters.topology_relation_count()
        + counters.binding_target_count()
        + counters.surface_support_count()
        + counters.neighborhood_valence()
        + counters.projected_entity_count()
        + counters.local_basis_part_count()
        + counters.transform_step_count()
        + counters.local_rebuild_evidence_row_count()
        + counters.retained_artifact_count()
        + counters.replay_checkpoint_count()
        + counters.diagnostic_count()
        + counters.user_outcome_count()
}

pub(super) fn thin_feature_source_rows(receipt: &ThinFeatureScaleSeparationReceipt) -> usize {
    let counters = receipt.counters();
    counters.thin_feature_count()
        + counters.local_scale_order_count()
        + counters.world_magnitude_order_count()
        + counters.precision_escalation_count()
        + counters.local_basis_part_count()
        + counters.projected_entity_count()
        + counters.transform_step_count()
        + counters.tiny_rotation_pressure_count()
        + counters.projection_consumed_basis_count()
        + counters.diagnostic_count()
        + counters.user_outcome_count()
}

pub(super) fn retained_cancellation_source_rows(
    receipt: &RetainedCancellationChainReceipt,
) -> usize {
    let counters = receipt.counters();
    counters.checkpoint_count()
        + counters.transform_step_count()
        + counters.replayed_checkpoint_count()
        + counters.trigger_local_replay_count()
        + counters.retained_artifact_count()
        + counters.projection_consumed_fact_count()
        + counters.diagnostic_trigger_count()
        + counters.user_outcome_count()
}

pub(super) fn dirty_clean_fail_source_rows(receipt: &DirtyPlanarCleanFailReceipt) -> usize {
    let counters = receipt.counters();
    counters.topology_clean_fail_receipts()
        + counters.clean_fail_boundary_receipts()
        + counters.recovery_receipts()
        + counters.transform_posture_receipts()
        + counters.diagnostic_receipts()
        + counters.user_outcome_receipts()
}

pub(super) fn open_posture_source_rows(receipt: &OpenPlanarPostureReceipt) -> usize {
    let counters = receipt.counters();
    counters.topology_receipts()
        + counters.unsupported_surface_receipts()
        + counters.clean_fail_boundary_receipts()
        + counters.transform_posture_receipts()
        + counters.diagnostic_receipts()
        + counters.user_outcome_receipts()
        + counters.bounded_surrogate_rejections()
}

pub(super) fn projection_parity_source_rows(receipt: &ProjectionFactParityReceipt) -> usize {
    let counters = receipt.counters();
    counters.lanes_compared()
        + counters.receipt_backed_lanes()
        + counters.denied_lanes()
        + counters.policy_required_lanes()
}

pub(super) fn boolean_readiness_source_rows(
    receipt: &PlanarBooleanReadinessWorkloadReceipt,
) -> usize {
    let counters = receipt.counters();
    counters.required_evidence_stages_consumed()
        + counters.ledger_rows_consumed()
        + counters.parity_lanes_consumed()
        + counters.closeout_rows_consumed()
        + counters.query_boundary_rows()
        + counters.blocked_branch_count()
}

pub(super) fn nmt_radial_fan_source_rows(receipt: &NmtRadialFanReceipt) -> usize {
    let counters = receipt.counters();
    counters.incident_face_count()
        + counters.open_boundary_half_edge_count()
        + counters.non_manifold_edge_count()
        + counters.topology_face_count()
        + counters.projected_entity_count()
        + counters.transform_step_count()
        + counters.changed_coordinate_count()
        + counters.retained_artifact_count()
        + counters.replay_checkpoint_count()
        + counters.diagnostic_count()
        + counters.user_outcome_count()
}

pub(super) fn mixed_surface_kill_box_source_rows(receipt: &MixedSurfaceKillBoxReceipt) -> usize {
    let counters = receipt.counters();
    counters.family_run_count()
        + counters.certified_plane_count()
        + counters.unsupported_family_count()
        + counters.support_receipt_count()
        + counters.user_outcome_count()
        + counters.upstream_geometry_carriers()
}

pub(super) fn open_class_triad_source_rows(receipt: &OpenClassTriadParityReceipt) -> usize {
    let counters = receipt.counters();
    counters.open_classes_compared()
        + counters.lanes_per_class()
        + counters.receipt_backed_lanes()
        + counters.bounded_conversion_guards()
}

pub(super) fn grazing_basket_stack_source_rows(receipt: &GrazingBasketStackReceipt) -> usize {
    let counters = receipt.counters();
    counters.total_layers()
        + counters.strips_per_layer()
        + counters.touched_layers()
        + counters.open_boundary_breadth()
        + counters.projection_breadth()
        + counters.retained_checkpoint_breadth()
        + counters.local_frame_breadth()
        + counters.radial_adjacency_breadth()
        + counters.precision_escalation_breadth()
        + counters.localization_breadth()
}
