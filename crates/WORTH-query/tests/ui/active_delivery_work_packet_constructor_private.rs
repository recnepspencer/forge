use worth_query::facade::{ActiveDeliveryWorkPacket, ActiveSubscriptionAllocationPosture};

fn main() {
    let _packet = ActiveDeliveryWorkPacket {
        active_lane_digest: todo!(),
        attachment_digest: todo!(),
        maintenance_delta: todo!(),
        lowering_report: todo!(),
        density_posture: todo!(),
        affected_lane_width: 1,
        affected_attachment_width: 1,
        patch_group_width: 1,
        continuation_width: 0,
        preview_residue_width: 0,
        allocation_scope_width: 1,
        allocation_posture: ActiveSubscriptionAllocationPosture::PatchScratch,
        performance_receipt: todo!(),
        work_packet_identity: todo!(),
    };
}
