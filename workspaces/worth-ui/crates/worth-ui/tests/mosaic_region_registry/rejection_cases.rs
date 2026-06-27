use super::*;

#[test]
fn mosaic_region_missing_sizing_behavior_rejected() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(
            mosaic_region_descriptor("workspace.region.primary")
                .with_sizing_behavior(MosaicSizingBehavior::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_regions().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingMosaicRegionSizingBehavior],
    );
}

#[test]
fn mosaic_region_missing_scroll_ownership_rejected() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(
            mosaic_region_descriptor("workspace.region.primary")
                .with_scroll_ownership(MosaicScrollOwnership::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_regions().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingMosaicRegionScrollOwnership],
    );
}

#[test]
fn mosaic_region_missing_focus_scope_rejected() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(
            mosaic_region_descriptor("workspace.region.primary")
                .with_focus_scope(MosaicFocusScopeKind::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_regions().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingMosaicRegionFocusScope],
    );
}

#[test]
fn platform_builtin_region_domain_name_rejected() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(mosaic_region_descriptor_with_role(
            "workspace.region.primary",
            MosaicRegionRole::product_domain_name_for_diagnostics("file browser"),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_regions().is_empty());
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::ProductDomainMosaicRegionRole,
            "workspace.region.primary",
        )],
    );
}

#[test]
fn unsupported_surface_class_rejected() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(
            mosaic_region_descriptor("workspace.region.primary").with_allowed_surface_class(
                SurfacePlacementClass::unsupported_for_diagnostics("floating-dock"),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_regions().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedMosaicRegionSurfaceClass],
    );
}

#[test]
fn surface_accepting_mosaic_region_without_surface_class_rejected() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(complete_mosaic_region_descriptor(
            "workspace.region.primary",
            MosaicRegionRole::primary(),
            MosaicChildRule::accepts_surfaces(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_regions().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingMosaicRegionAllowedSurfaceClass],
    );
}

#[test]
fn region_only_mosaic_region_without_surface_classes_is_admitted() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(complete_mosaic_region_descriptor(
            "workspace.region.split",
            MosaicRegionRole::split(),
            MosaicChildRule::accepts_regions(),
        ))
        .freeze_with_registration_report();

    assert!(!report.has_errors());
    assert!(report.registration_diagnostics().is_empty());
    assert_registered_mosaic_region_ids(
        report.accepted_snapshot().mosaic_regions(),
        &["workspace.region.split"],
    );
}

#[test]
fn mosaic_region_descriptor_reports_multiple_independent_violations() {
    let report = WorthUi::app()
        .register_mosaic_region_kind(
            MosaicRegionKindDescriptor::new(
                mosaic_region_id("workspace.region.invalid"),
                MosaicRegionRole::product_domain_name_for_diagnostics("issue list"),
            )
            .with_sizing_behavior(MosaicSizingBehavior::missing_for_diagnostics())
            .with_scroll_ownership(MosaicScrollOwnership::missing_for_diagnostics())
            .with_focus_scope(MosaicFocusScopeKind::missing_for_diagnostics())
            .with_child_rule(MosaicChildRule::missing_for_diagnostics())
            .with_allowed_surface_class(SurfacePlacementClass::unsupported_for_diagnostics(
                "detached-drawer",
            ))
            .with_persistence(MosaicRegionPersistence::missing_for_diagnostics())
            .with_clipping(MosaicClippingPosture::missing_for_diagnostics())
            .with_hit_test(MosaicHitTestPosture::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().mosaic_regions().is_empty());
    assert_exact_diagnostic_topology(
        report.registration_diagnostics(),
        &[
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::MissingMosaicRegionSizingBehavior,
                "workspace.region.invalid",
            ),
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::MissingMosaicRegionScrollOwnership,
                "workspace.region.invalid",
            ),
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::MissingMosaicRegionFocusScope,
                "workspace.region.invalid",
            ),
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::MissingMosaicRegionChildRule,
                "workspace.region.invalid",
            ),
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::MissingMosaicRegionPersistence,
                "workspace.region.invalid",
            ),
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::MissingMosaicRegionClipping,
                "workspace.region.invalid",
            ),
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::MissingMosaicRegionHitTest,
                "workspace.region.invalid",
            ),
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::ProductDomainMosaicRegionRole,
                "workspace.region.invalid",
            ),
            DiagnosticTopology::new(
                CapabilityDiagnosticCode::UnsupportedMosaicRegionSurfaceClass,
                "workspace.region.invalid",
            ),
        ],
    );
}
