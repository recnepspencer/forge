use worth_ui::facade::{
    app::WorthUi,
    registry::{
        CommandProjectionCommandReference, CommandProjectionDescriptor, CommandProjectionSurface,
    },
};

use super::command_projection_assertions::{
    assert_dependency_diagnostics, assert_registered_command_projection_ids,
};
use super::command_projection_fixtures::{
    command_descriptor, command_id, command_projection, command_projection_for_command,
    command_projection_id, mosaic_placement_policy, mosaic_placement_policy_id,
};

#[test]
fn projection_references_unknown_command_rejected() {
    let report = WorthUi::app()
        .register_command_projection(command_projection_for_command(
            "workspace.projection.toolbar",
            "workspace.command.save",
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().command_projections().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            "workspace.projection.toolbar",
            "command",
            "workspace.command.save",
        )],
    );
}

#[test]
fn duplicated_missing_command_reference_reports_one_dependency() {
    let report = WorthUi::app()
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.toolbar"),
                CommandProjectionSurface::toolbar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.save",
            )))
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.save",
            ))),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            "workspace.projection.toolbar",
            "command",
            "workspace.command.save",
        )],
    );
}

#[test]
fn projection_command_reference_resolves_against_registered_command() {
    let app = WorthUi::app()
        .register_command(command_descriptor("workspace.command.save", "Save"))
        .register_command_projection(command_projection_for_command(
            "workspace.projection.toolbar",
            "workspace.command.save",
        ))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(app.capabilities().commands().len(), 1);
    assert_registered_command_projection_ids(
        app.capabilities().command_projections(),
        &["workspace.projection.toolbar"],
    );
}

#[test]
fn command_projection_eligibility_resolves_against_registered_projection() {
    let projection_id = command_projection_id("workspace.projection.palette");
    let app = WorthUi::app()
        .register_command_projection(command_projection("workspace.projection.palette"))
        .register_command(
            command_descriptor("workspace.command.open", "Open")
                .with_projection_eligibility(projection_id.clone()),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(app.capabilities().commands().len(), 1);
    assert_eq!(
        app.capabilities()
            .commands()
            .get(&command_id("workspace.command.open"))
            .expect("command")
            .projection_eligibility(),
        Some(&projection_id)
    );
    assert_registered_command_projection_ids(
        app.capabilities().command_projections(),
        &["workspace.projection.palette"],
    );
}

#[test]
fn projection_references_unknown_mosaic_placement_rejected() {
    let report = WorthUi::app()
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.region"),
                CommandProjectionSurface::region_header_action(),
            )
            .with_eligible_category(worth_ui::facade::registry::CommandCategory::Workspace)
            .with_mosaic_scope(
                worth_ui::facade::registry::CommandProjectionMosaicScope::placement_policy(
                    mosaic_placement_policy_id("workspace.placement.primary"),
                ),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            "workspace.projection.region",
            "mosaic_placement_policy",
            "workspace.placement.primary",
        )],
    );
}

#[test]
fn projection_mosaic_scope_resolves_against_registered_placement_policy() {
    let app = WorthUi::app()
        .register_mosaic_placement_policy(mosaic_placement_policy("workspace.placement.primary"))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.region"),
                CommandProjectionSurface::region_header_action(),
            )
            .with_eligible_category(worth_ui::facade::registry::CommandCategory::Workspace)
            .with_mosaic_scope(
                worth_ui::facade::registry::CommandProjectionMosaicScope::placement_policy(
                    mosaic_placement_policy_id("workspace.placement.primary"),
                ),
            ),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_registered_command_projection_ids(
        app.capabilities().command_projections(),
        &["workspace.projection.region"],
    );
}
