use worth_ui::facade::{
    CapabilityDiagnosticCode, CapabilityDiagnosticRichness, CapabilityDiagnosticSeverity,
    CapabilityRegistrationReport, ImageAssetDescriptor, ImageAssetId, ImageAssetSourceKind,
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

#[test]
fn unsupported_image_assets_report_capability_posture_and_do_not_freeze() {
    let asset_id = ImageAssetId::new("worth.image.remote_logo").expect("valid image asset id");
    let report = WorthUi::app()
        .register_image_asset(ImageAssetDescriptor::unsupported(
            asset_id.clone(),
            ImageAssetSourceKind::remote_unsupported(),
            "https://example.test/logo.png",
        ))
        .freeze_with_registration_report();

    assert!(report.registration_diagnostics().iter().any(|diagnostic| {
        diagnostic.code() == CapabilityDiagnosticCode::UnsupportedPostureReference
            && diagnostic.family_name() == Some("image_asset")
            && diagnostic.identity_text() == Some(asset_id.as_str())
    }));
    assert!(report
        .accepted_snapshot()
        .image_assets()
        .get(&asset_id)
        .is_none());
}

fn _report_type_is_public_observation_artifact(_report: CapabilityRegistrationReport) {}
