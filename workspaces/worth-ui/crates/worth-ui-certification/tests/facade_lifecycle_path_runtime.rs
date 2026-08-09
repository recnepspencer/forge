use worth_ui_runtime::facade::{
    entry::WorthUi, inspection_bridge::InspectionDispatchLane,
    lifecycle::RUNTIME_SUPPORT_INVENTORY, registry::snapshot::CapabilitySnapshot,
};

#[test]
fn facade_lifecycle_submodules_teach_valid_entry_order() {
    let _entry = WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse());
    let _inventory = RUNTIME_SUPPORT_INVENTORY;
    let _registry = core::mem::size_of::<CapabilitySnapshot>();
    let _dispatch_lane = InspectionDispatchLane::MeasurementScope;
}
