use worth_ui::facade::graph::{
    UiGraphTouchAuthority, UiGraphTouchDescriptor, UiGraphTouchOriginReceipt,
    UiGraphTouchOriginWitness, UiGraphTouchTarget,
};

fn main() {
    let _ = UiGraphTouchAuthority::from_node_changed;
    let _ = UiGraphTouchDescriptor::new;
    let _ = UiGraphTouchOriginReceipt::declaration_change;
    let _ = UiGraphTouchOriginReceipt::query_fact_change;
    let _ = UiGraphTouchOriginReceipt::host_observation;
    let _ = UiGraphTouchOriginReceipt::service_event;
    let _ = UiGraphTouchOriginReceipt::intent_submission;
    let _ = UiGraphTouchOriginReceipt::diagnostic_only;
    let _ = UiGraphTouchOriginWitness::declaration_instances;
    let _ = UiGraphTouchOriginWitness::mounted_receipt_transition_only;
    let _ = UiGraphTouchOriginWitness::authored_provenance_digests;
    let _ = UiGraphTouchTarget::node;
    let _ = UiGraphTouchTarget::slot_occupancy;
    let _ = UiGraphTouchTarget::page_membership;
    let _ = UiGraphTouchTarget::region_membership;
    let _ = UiGraphTouchTarget::mosaic_membership;
    let _ = UiGraphTouchTarget::mounted_receipt_slot;
}
