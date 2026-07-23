use worth_ui::facade::{
    app::WorthUi,
    diagnostics::CapabilityDiagnosticCode,
    registry::{
        CommandCategory, CommandProjectionDescriptor, CommandProjectionGrouping,
        CommandProjectionMeaningOverride, CommandProjectionMosaicScope, CommandProjectionSurface,
    },
};

use super::command_projection_assertions::assert_diagnostic_codes;
use super::command_projection_fixtures::{
    command_projection, command_projection_id, mosaic_placement_policy_id,
};

#[test]
fn projection_references_unsupported_surface_rejected() {
    let report = WorthUi::app()
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.unsupported"),
                CommandProjectionSurface::unsupported_for_diagnostics("ad_hoc_side_rail"),
            )
            .with_eligible_category(CommandCategory::Workspace),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().command_projections().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedCommandProjectionSurface],
    );
}

#[test]
fn projection_with_conflicting_required_grouping_rejected() {
    let report = WorthUi::app()
        .register_command_projection(
            command_projection("workspace.projection.conflicting")
                .with_grouping(CommandProjectionGrouping::required("file"))
                .with_grouping(CommandProjectionGrouping::required("edit")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::ConflictingCommandProjectionGrouping],
    );
}

#[test]
fn projection_cannot_define_new_command_meaning() {
    let report = WorthUi::app()
        .register_command_projection(
            command_projection("workspace.projection.meaning")
                .with_command_meaning_override_for_diagnostics(
                    CommandProjectionMeaningOverride::Readiness,
                ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::CommandProjectionDefinesCommandMeaning],
    );
}

#[test]
fn projection_without_command_reference_or_category_rejected() {
    let report = WorthUi::app()
        .register_command_projection(CommandProjectionDescriptor::new(
            command_projection_id("workspace.projection.empty"),
            CommandProjectionSurface::command_palette(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingCommandProjectionEligibility],
    );
}

#[test]
fn region_header_projection_requires_mosaic_scope() {
    let report = WorthUi::app()
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.region"),
                CommandProjectionSurface::region_header_action(),
            )
            .with_eligible_category(CommandCategory::Workspace),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingCommandProjectionMosaicScope],
    );
}

#[test]
fn global_projection_rejects_mosaic_scope() {
    let report = WorthUi::app()
        .register_command_projection(
            command_projection("workspace.projection.palette").with_mosaic_scope(
                CommandProjectionMosaicScope::placement_policy(mosaic_placement_policy_id(
                    "workspace.placement.primary",
                )),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::UnexpectedCommandProjectionMosaicScope,
            CapabilityDiagnosticCode::MissingDependency,
        ],
    );
}
