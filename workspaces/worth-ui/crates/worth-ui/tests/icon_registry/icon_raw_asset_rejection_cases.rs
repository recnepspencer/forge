use worth_ui::facade::{CapabilityDiagnosticCode, IconDescriptor, RawIconAssetReference, WorthUi};

use super::icon_assertions::assert_diagnostic_codes;
use super::icon_fixtures::icon_id;

#[test]
fn raw_asset_path_cannot_stand_in_for_stable_icon_id() {
    let report = WorthUi::app()
        .register_icon(IconDescriptor::raw_asset_path_for_diagnostics(
            icon_id("workspace.icon.save"),
            RawIconAssetReference::new("assets/icons/save.svg"),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(
        &report,
        &[
            CapabilityDiagnosticCode::MissingIconSource,
            CapabilityDiagnosticCode::RawIconAssetPathOutsideIconSource,
        ],
    );
}
