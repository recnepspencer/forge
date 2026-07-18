use super::*;

#[test]
fn equivalent_mosaic_region_descriptors_produce_equivalent_entries() {
    let first = WorthUi::app()
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.primary"))
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.sidebar"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.sidebar"))
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.primary"))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        first.capabilities().mosaic_regions(),
        second.capabilities().mosaic_regions()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_mosaic_region_ids(
        first.capabilities().mosaic_regions(),
        &["workspace.region.primary", "workspace.region.sidebar"],
    );
}

#[test]
fn duplicate_mosaic_region_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.primary"))
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.primary"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_regions().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn duplicate_mosaic_region_id_rejects_only_the_duplicate_identity() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.valid"))
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.primary"))
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.primary"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_mosaic_region_ids(
        report.accepted_snapshot().mosaic_regions(),
        &["workspace.region.valid"],
    );
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "workspace.region.primary",
            ),
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "workspace.region.primary",
            ),
        ],
    );
}

#[test]
fn different_mosaic_region_descriptor_meaning_produces_different_snapshot_digest() {
    let primary = WorthUi::app()
        .register_mosaic_region_kind(mosaic_region_descriptor("workspace.region.primary"))
        .freeze()
        .expect("application preparation should succeed");
    let modal = WorthUi::app()
        .register_mosaic_region_kind(mosaic_region_descriptor_with_role(
            "workspace.region.primary",
            MosaicRegionRole::modal(),
        ))
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        primary.capabilities().mosaic_regions(),
        modal.capabilities().mosaic_regions()
    );
    assert_ne!(
        primary.capabilities().digest(),
        modal.capabilities().digest()
    );
}

#[test]
fn different_allowed_surface_classes_change_snapshot_digest() {
    let combined = WorthUi::app()
        .register_mosaic_region_kind(
            mosaic_region_descriptor("workspace.region.primary")
                .with_allowed_surface_class(SurfacePlacementClass::primary_region())
                .with_allowed_surface_class(SurfacePlacementClass::status_region()),
        )
        .freeze()
        .expect("application preparation should succeed");
    let split = WorthUi::app()
        .register_mosaic_region_kind(
            mosaic_region_descriptor("workspace.region.primary")
                .with_allowed_surface_class(SurfacePlacementClass::primary_region())
                .with_allowed_surface_class(SurfacePlacementClass::overlay_layer()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        combined.capabilities().mosaic_regions(),
        split.capabilities().mosaic_regions()
    );
    assert_ne!(
        combined.capabilities().digest(),
        split.capabilities().digest()
    );
}
