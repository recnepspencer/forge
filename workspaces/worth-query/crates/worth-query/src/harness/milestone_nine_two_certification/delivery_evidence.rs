use crate::harness::certification::digest_parts;
use crate::subscription::{
    advance_subscription_acknowledgement, build_active_delivery_work_packet,
    emit_query_delivery_batch, ActiveAllocationScopeWidth, ActiveDeliveryAffectedAttachmentWidth,
    ActiveDeliveryAffectedLaneWidth, ActiveDeliveryContinuationWidth, ActiveDeliveryDensityPosture,
    ActiveDeliveryPreviewResidueWidth, ActiveSubscriptionAllocationPosture,
    ActiveSubscriptionRuntime, PatchGroupWidth, QuerySubscriptionMaintenanceDelta,
};

#[derive(Debug, Eq, PartialEq)]
pub(super) struct DeliveryEvidence {
    pub(super) performance_receipt_digest: String,
    pub(super) active_delivery_work_packet_digest: String,
    pub(super) density_posture_digest: String,
    pub(super) maintenance_delta_digest: String,
    pub(super) delivery_batch_digest: String,
    pub(super) delivery_window_digest: String,
    pub(super) patch_group_digest: String,
    pub(super) delivery_receipt_digest: String,
    pub(super) acknowledgement_frontier_digest: String,
    pub(super) work_packet: crate::subscription::ActiveDeliveryWorkPacket,
    pub(super) delivery_batch: crate::subscription::QueryDeliveryBatch,
    pub(super) acknowledged_attachment: crate::subscription::SubscriptionConsumerAttachment,
    pub(super) work_packet_counter_digest: String,
    pub(super) batch_counter_digest: String,
    pub(super) ack_counter_digest: String,
}

pub(super) fn deliver_to_attachment(
    runtime: &mut ActiveSubscriptionRuntime,
    attachment: crate::subscription::SubscriptionConsumerAttachment,
    window: crate::subscription::QueryDeliveryWindow,
    delta: QuerySubscriptionMaintenanceDelta,
    lowering_report: crate::subscription::QueryMaintenanceDeltaLoweringReport,
    density_posture: ActiveDeliveryDensityPosture,
    affected_lane_width: u64,
    affected_attachment_width: u64,
    patch_width: u64,
    continuation_width: u64,
    preview_residue_width: u64,
    allocation_scope_width: u64,
    allocation_posture: ActiveSubscriptionAllocationPosture,
) -> DeliveryEvidence {
    let work_packet = build_active_delivery_work_packet(
        runtime,
        &attachment,
        delta,
        lowering_report,
        density_posture,
        ActiveDeliveryAffectedLaneWidth::measured(affected_lane_width),
        ActiveDeliveryAffectedAttachmentWidth::measured(affected_attachment_width),
        PatchGroupWidth::measured(patch_width),
        ActiveDeliveryContinuationWidth::measured(continuation_width),
        ActiveDeliveryPreviewResidueWidth::measured(preview_residue_width),
        ActiveAllocationScopeWidth::measured(allocation_scope_width),
        allocation_posture,
    )
    .unwrap();
    let work_packet_counter_digest = runtime.counters().counter_projection().label().to_string();
    let performance_receipt_digest = work_packet
        .performance_receipt()
        .performance_receipt_projection()
        .label()
        .to_string();
    let active_delivery_work_packet_digest =
        work_packet.work_packet_projection().label().to_string();
    let density_posture_digest =
        digest_parts(&[work_packet.density_posture().as_str().to_string()]);
    let maintenance_delta_digest = work_packet
        .maintenance_delta()
        .maintenance_delta_projection()
        .label()
        .to_string();
    let batch = emit_query_delivery_batch(runtime, window, work_packet.clone()).unwrap();
    let batch_counter_digest = batch.counters().counter_projection().label().to_string();
    let delivery_batch_digest = batch.delivery_batch_projection().label().to_string();
    let delivery_window_digest = batch.delivery_window_projection().label().to_string();
    let patch_group_digest = batch
        .patch_group()
        .patch_group_projection()
        .label()
        .to_string();
    let delivery_receipt_digest = batch.receipt().receipt_projection().label().to_string();
    let acknowledgement =
        advance_subscription_acknowledgement(runtime, attachment, batch.receipt().clone()).unwrap();
    let ack_counter_digest = runtime.counters().counter_projection().label().to_string();
    let acknowledgement_frontier_digest = acknowledgement
        .acknowledgement_frontier()
        .frontier_projection()
        .label()
        .to_string();

    DeliveryEvidence {
        performance_receipt_digest,
        active_delivery_work_packet_digest,
        density_posture_digest,
        maintenance_delta_digest,
        delivery_batch_digest,
        delivery_window_digest,
        patch_group_digest,
        delivery_receipt_digest,
        acknowledgement_frontier_digest,
        work_packet,
        delivery_batch: batch,
        acknowledged_attachment: acknowledgement,
        work_packet_counter_digest,
        batch_counter_digest,
        ack_counter_digest,
    }
}
