use worth_ui::facade::{
    CommandCategory, CommandProjectionCommandReference, CommandProjectionDescriptor,
    CommandProjectionGrouping, CommandProjectionOverflowBehavior, CommandProjectionSurface,
    WorthUi,
};

use super::command_projection_assertions::assert_registered_command_projection_ids;
use super::command_projection_fixtures::{
    command_descriptor, command_id, command_projection, command_projection_id,
};

#[test]
fn equivalent_command_projections_produce_equivalent_entries() {
    let first = WorthUi::app()
        .register_command_projection(command_projection("workspace.projection.palette"))
        .register_command_projection(command_projection("workspace.projection.toolbar"))
        .freeze();
    let second = WorthUi::app()
        .register_command_projection(command_projection("workspace.projection.toolbar"))
        .register_command_projection(command_projection("workspace.projection.palette"))
        .freeze();

    assert_eq!(
        first.capabilities().command_projections(),
        second.capabilities().command_projections()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_command_projection_ids(
        first.capabilities().command_projections(),
        &[
            "workspace.projection.palette",
            "workspace.projection.toolbar",
        ],
    );
}

#[test]
fn equivalent_command_projection_command_references_are_canonicalized() {
    let first = WorthUi::app()
        .register_command(command_descriptor("workspace.command.open", "Open"))
        .register_command(command_descriptor("workspace.command.save", "Save"))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.toolbar"),
                CommandProjectionSurface::toolbar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.open",
            )))
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.save",
            )))
            .with_ordering(worth_ui::facade::CommandProjectionOrdering::ByCommandId),
        )
        .freeze();
    let second = WorthUi::app()
        .register_command(command_descriptor("workspace.command.open", "Open"))
        .register_command(command_descriptor("workspace.command.save", "Save"))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.toolbar"),
                CommandProjectionSurface::toolbar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.save",
            )))
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.open",
            )))
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.open",
            )))
            .with_ordering(worth_ui::facade::CommandProjectionOrdering::ByCommandId),
        )
        .freeze();

    assert_eq!(
        first.capabilities().command_projections(),
        second.capabilities().command_projections()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_eq!(
        first
            .capabilities()
            .command_projections()
            .entries()
            .first()
            .expect("projection")
            .descriptor()
            .command_references()
            .iter()
            .map(|reference| reference.command_id().as_str())
            .collect::<Vec<_>>(),
        vec!["workspace.command.open", "workspace.command.save"]
    );
}

#[test]
fn declaration_ordered_command_projection_references_preserve_ordering_meaning() {
    let open_then_save = WorthUi::app()
        .register_command(command_descriptor("workspace.command.open", "Open"))
        .register_command(command_descriptor("workspace.command.save", "Save"))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.toolbar"),
                CommandProjectionSurface::toolbar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.open",
            )))
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.save",
            ))),
        )
        .freeze();
    let save_then_open = WorthUi::app()
        .register_command(command_descriptor("workspace.command.open", "Open"))
        .register_command(command_descriptor("workspace.command.save", "Save"))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.toolbar"),
                CommandProjectionSurface::toolbar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.save",
            )))
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.open",
            ))),
        )
        .freeze();

    assert_ne!(
        open_then_save.capabilities().command_projections(),
        save_then_open.capabilities().command_projections()
    );
    assert_ne!(
        open_then_save.capabilities().digest(),
        save_then_open.capabilities().digest()
    );
}

#[test]
fn declaration_ordered_command_projection_references_deduplicate_without_reordering() {
    let app = WorthUi::app()
        .register_command(command_descriptor("workspace.command.open", "Open"))
        .register_command(command_descriptor("workspace.command.save", "Save"))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.toolbar"),
                CommandProjectionSurface::toolbar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.save",
            )))
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.open",
            )))
            .with_command_reference(CommandProjectionCommandReference::command(command_id(
                "workspace.command.save",
            ))),
        )
        .freeze();

    assert_eq!(
        app.capabilities()
            .command_projections()
            .entries()
            .first()
            .expect("projection")
            .descriptor()
            .command_references()
            .iter()
            .map(|reference| reference.command_id().as_str())
            .collect::<Vec<_>>(),
        vec!["workspace.command.save", "workspace.command.open"]
    );
}

#[test]
fn duplicate_command_projection_groupings_do_not_change_snapshot_meaning() {
    let single_grouping = WorthUi::app()
        .register_command_projection(
            command_projection("workspace.projection.palette")
                .with_grouping(CommandProjectionGrouping::optional("workspace")),
        )
        .freeze();
    let duplicated_grouping = WorthUi::app()
        .register_command_projection(
            command_projection("workspace.projection.palette")
                .with_grouping(CommandProjectionGrouping::optional("workspace"))
                .with_grouping(CommandProjectionGrouping::optional("workspace")),
        )
        .freeze();

    assert_eq!(
        single_grouping.capabilities().command_projections(),
        duplicated_grouping.capabilities().command_projections()
    );
    assert_eq!(
        single_grouping.capabilities().digest(),
        duplicated_grouping.capabilities().digest()
    );
}

#[test]
fn different_projection_policy_changes_snapshot_digest() {
    let plain = WorthUi::app()
        .register_command_projection(command_projection("workspace.projection.palette"))
        .freeze();
    let richer = WorthUi::app()
        .register_command_projection(
            worth_ui::facade::CommandProjectionDescriptor::new(
                command_projection_id("workspace.projection.palette"),
                CommandProjectionSurface::command_palette(),
            )
            .with_eligible_category(CommandCategory::Workspace)
            .with_grouping(CommandProjectionGrouping::optional("workspace"))
            .show_shortcuts()
            .show_readiness()
            .with_overflow_behavior(CommandProjectionOverflowBehavior::collapse_to_more()),
        )
        .freeze();

    assert_ne!(
        plain.capabilities().command_projections(),
        richer.capabilities().command_projections()
    );
    assert_ne!(
        plain.capabilities().digest(),
        richer.capabilities().digest()
    );
}
