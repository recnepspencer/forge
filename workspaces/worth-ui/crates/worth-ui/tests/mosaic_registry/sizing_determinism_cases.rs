use worth_ui::facade::{
    app::WorthUi,
    declaration::{MeasurementValue, MosaicSizingKind},
};

use super::sizing_assertions::assert_registered_mosaic_sizing_ids;
use super::sizing_fixtures::{
    bounded_sidebar_contract, complete_sizing_contract, fixed_toolbar_contract,
};

#[test]
fn equivalent_named_sizing_contracts_produce_equivalent_entries() {
    let first = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(bounded_sidebar_contract("workspace.sizing.sidebar"))
        .register_mosaic_sizing_contract(fixed_toolbar_contract("workspace.sizing.toolbar"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(fixed_toolbar_contract("workspace.sizing.toolbar"))
        .register_mosaic_sizing_contract(bounded_sidebar_contract("workspace.sizing.sidebar"))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        first.capabilities().mosaic_sizing_contracts(),
        second.capabilities().mosaic_sizing_contracts()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_mosaic_sizing_ids(
        first.capabilities().mosaic_sizing_contracts(),
        &["workspace.sizing.sidebar", "workspace.sizing.toolbar"],
    );
}

#[test]
fn different_named_sizing_meaning_changes_snapshot_digest() {
    let bounded = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(bounded_sidebar_contract("workspace.sizing.sidebar"))
        .freeze()
        .expect("application preparation should succeed");
    let fill = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(complete_sizing_contract(
            "workspace.sizing.sidebar",
            MosaicSizingKind::fill(),
        ))
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        bounded.capabilities().mosaic_sizing_contracts(),
        fill.capabilities().mosaic_sizing_contracts()
    );
    assert_ne!(
        bounded.capabilities().digest(),
        fill.capabilities().digest()
    );
}

#[test]
fn named_measurement_values_remain_inspectable_after_freeze() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_mosaic_sizing_contract(bounded_sidebar_contract("workspace.sizing.sidebar"))
        .freeze()
        .expect("application preparation should succeed");
    let descriptor = &app.capabilities().mosaic_sizing_contracts().descriptors()[0];

    assert_eq!(descriptor.kind(), &MosaicSizingKind::bounded());
    assert_eq!(
        descriptor
            .named_measurement()
            .expect("measurement should be frozen")
            .value(),
        &MeasurementValue::logical_pixels(320)
    );
}
