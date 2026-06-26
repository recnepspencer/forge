use worth_ui::facade::{
    CapabilityDiagnosticCode, MosaicPlacementAction, MosaicPlacementEligibility,
    MosaicPlacementSource, MosaicPlacementSupport, MosaicPlacementTarget, MosaicRegionRole,
    SurfacePlacementClass, WorthUi,
};

use crate::mosaic_placement_registry_assertions::assert_diagnostic_codes;
use crate::mosaic_placement_registry_fixtures::complete_policy;

#[test]
fn illegal_surface_to_region_placement_rejected() {
    let report = WorthUi::app()
        .register_mosaic_placement_policy(
            complete_policy(
                "workspace.placement.primary_to_toolbar",
                MosaicPlacementAction::dock(),
            )
            .with_source(MosaicPlacementSource::surface_class(
                SurfacePlacementClass::primary_region(),
            ))
            .with_target(MosaicPlacementTarget::region_role(
                MosaicRegionRole::toolbar(),
            )),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_placement_policies()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::IllegalMosaicPlacementSourceTarget],
    );
}

#[test]
fn cyclic_mosaic_containment_policy_rejected() {
    let report = WorthUi::app()
        .register_mosaic_placement_policy(
            complete_policy(
                "workspace.placement.split_cycle",
                MosaicPlacementAction::split(),
            )
            .with_source(MosaicPlacementSource::region_role(MosaicRegionRole::split()))
            .with_target(MosaicPlacementTarget::region_stack(
                MosaicRegionRole::split(),
            )),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_placement_policies()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::CyclicMosaicContainmentPolicy],
    );
}

#[test]
fn unsupported_float_or_overlay_policy_rejected() {
    let report = WorthUi::app()
        .register_mosaic_placement_policy(
            complete_policy("workspace.placement.float", MosaicPlacementAction::float())
                .with_source(MosaicPlacementSource::surface_class(
                    SurfacePlacementClass::transient_layer(),
                ))
                .with_target(MosaicPlacementTarget::region_role(
                    MosaicRegionRole::floating(),
                ))
                .with_support(
                    MosaicPlacementSupport::unsupported_float_or_overlay_for_diagnostics(),
                ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_placement_policies()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedMosaicFloatOrOverlayPolicy],
    );
}

#[test]
fn surface_target_must_match_declared_action() {
    let report = WorthUi::app()
        .register_mosaic_placement_policy(
            complete_policy(
                "workspace.placement.primary_toolbar_projection",
                MosaicPlacementAction::toolbar_projection(),
            )
            .with_eligibility(MosaicPlacementEligibility::new(
                MosaicPlacementSource::surface_class(SurfacePlacementClass::primary_region()),
                MosaicPlacementTarget::region_role(MosaicRegionRole::primary()),
            )),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_placement_policies()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::IllegalMosaicPlacementSourceTarget],
    );
}

#[test]
fn plugin_cannot_imperatively_rearrange_mosaic_state() {
    let report = WorthUi::app()
        .register_mosaic_placement_policy(
            complete_policy(
                "workspace.placement.plugin_mutation",
                MosaicPlacementAction::dock(),
            )
            .with_source(MosaicPlacementSource::plugin_imperative_mutation_for_diagnostics())
            .with_target(MosaicPlacementTarget::region_role(
                MosaicRegionRole::primary(),
            )),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report
        .accepted_snapshot()
        .mosaic_placement_policies()
        .is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::ImperativeMosaicStateMutationPolicy],
    );
}

#[test]
fn mosaic_placement_policy_requires_source_and_target_families() {
    let report = WorthUi::app()
        .register_mosaic_placement_policy(
            complete_policy("workspace.placement.missing", MosaicPlacementAction::dock())
                .with_source(MosaicPlacementSource::missing_for_diagnostics())
                .with_target(MosaicPlacementTarget::missing_for_diagnostics()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::MissingMosaicPlacementSource,
            CapabilityDiagnosticCode::MissingMosaicPlacementTarget,
        ],
    );
}
