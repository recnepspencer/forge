use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        IconColorSupport, IconDescriptor, IconFamily, IconSizeSupport, IconSourceDescriptor,
    },
    diagnostics::CapabilityDiagnosticCode,
};

use super::icon_assertions::assert_diagnostic_codes;
use super::icon_fixtures::icon_id;

#[test]
fn unknown_icon_family_rejected() {
    let report = WorthUi::app()
        .register_icon(IconDescriptor::new(
            icon_id("workspace.icon.domain"),
            IconFamily::unknown_for_diagnostics("mailbox"),
            IconSourceDescriptor::symbol("mailbox"),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(&report, &[CapabilityDiagnosticCode::UnknownIconFamily]);
}

#[test]
fn missing_icon_source_rejected() {
    let report = WorthUi::app()
        .register_icon(IconDescriptor::missing_source_for_diagnostics(
            icon_id("workspace.icon.save"),
            IconFamily::command(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(&report, &[CapabilityDiagnosticCode::MissingIconSource]);
}

#[test]
fn unsupported_icon_source_kind_rejected() {
    let report = WorthUi::app()
        .register_icon(IconDescriptor::new(
            icon_id("workspace.icon.save"),
            IconFamily::command(),
            IconSourceDescriptor::unsupported_for_diagnostics("save"),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::UnsupportedIconSourceKind],
    );
}

#[test]
fn missing_icon_source_metadata_rejected() {
    let report = WorthUi::app()
        .register_icon(IconDescriptor::new(
            icon_id("workspace.icon.save"),
            IconFamily::command(),
            IconSourceDescriptor::vector_asset("", "  "),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(
        &report,
        &[CapabilityDiagnosticCode::MissingIconSourceMetadata],
    );
}

#[test]
fn missing_icon_source_capabilities_rejected_together() {
    let report = WorthUi::app()
        .register_icon(IconDescriptor::new(
            icon_id("workspace.icon.save"),
            IconFamily::command(),
            IconSourceDescriptor::symbol("save")
                .with_size_support(IconSizeSupport::missing_for_diagnostics())
                .with_color_support(IconColorSupport::missing_for_diagnostics()),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().icons().is_empty());
    assert_diagnostic_codes(
        &report,
        &[
            CapabilityDiagnosticCode::MissingIconSizeSupport,
            CapabilityDiagnosticCode::MissingIconColorSupport,
        ],
    );
}
