use worth_ui_runtime::facade::{
    entry::WorthUi, inspection_bridge::InspectionDispatchLane,
    lifecycle::RUNTIME_SUPPORT_INVENTORY, registry::CapabilitySnapshot,
};

#[test]
fn facade_lifecycle_submodules_teach_valid_entry_order() {
    let _entry = WorthUi::app();
    let _inventory = RUNTIME_SUPPORT_INVENTORY;
    let _registry = core::mem::size_of::<CapabilitySnapshot>();
    let _dispatch_lane = InspectionDispatchLane::MeasurementScope;
}
