use worth_ui::facade::{
    CapabilityDiagnosticRichness, CapabilityDiagnosticSeverity, CapabilityRegistrationReport,
    WorthUi,
};

#[test]
fn successful_freeze_can_return_empty_registration_report() {
    let report = WorthUi::app().freeze_with_registration_report();

    assert!(!report.has_errors());
    assert!(report.registration_diagnostics().is_empty());
    assert!(report
        .accepted_snapshot()
        .registered_capabilities()
        .is_empty());
}

#[test]
fn diagnostics_do_not_change_accepted_snapshot_digest() {
    let minimal = WorthUi::app()
        .with_minimal_registration_diagnostics()
        .freeze_with_registration_report();
    let rich = WorthUi::app()
        .with_rich_registration_diagnostics()
        .freeze_with_registration_report();

    assert_eq!(
        minimal.accepted_snapshot().digest(),
        rich.accepted_snapshot().digest()
    );
    assert_eq!(minimal.accepted_snapshot(), rich.accepted_snapshot());
}

#[test]
fn diagnostic_richness_and_severity_are_typed_public_vocabulary() {
    assert!(CapabilityDiagnosticSeverity::Error.is_error());
    assert_eq!(
        CapabilityDiagnosticRichness::default(),
        CapabilityDiagnosticRichness::Rich,
    );
}

fn _report_type_is_public_observation_artifact(_report: CapabilityRegistrationReport) {}
