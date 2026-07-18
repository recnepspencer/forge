use worth_ui::facade::{
    CapabilityDiagnosticCode, CommandDescriptor, ComponentAccessibilitySupport,
    ComponentChildPolicy, ComponentDescriptor, ComponentExecutionLane, ComponentFocusSupport,
    ComponentPropSchema, ComponentStateOwnership, ThemeTokenId, WorthUi,
};

#[path = "component_registry/component_registry_assertions.rs"]
mod component_registry_assertions;
#[path = "component_registry/component_registry_fixtures.rs"]
mod component_registry_fixtures;

use component_registry_assertions::{
    assert_dependency_diagnostics, assert_diagnostic_codes, assert_diagnostic_codes_and_identities,
    assert_registered_component_ids,
};
use component_registry_fixtures::{command_id, component_descriptor, component_id};

#[test]
fn equivalent_component_descriptors_produce_equivalent_entries() {
    let first = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_component(component_descriptor("workspace.component.sidebar"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .register_component(component_descriptor("workspace.component.sidebar"))
        .register_component(component_descriptor("workspace.component.editor"))
        .freeze()
        .expect("application preparation should succeed");

    assert_eq!(
        first.capabilities().components(),
        second.capabilities().components()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_component_ids(
        first.capabilities().components(),
        &["workspace.component.editor", "workspace.component.sidebar"],
    );
}

#[test]
fn duplicate_component_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .register_component(component_descriptor("workspace.component.editor"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().components().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn duplicate_component_id_rejects_only_the_duplicate_identity() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.valid"))
        .register_component(component_descriptor("workspace.component.editor"))
        .register_component(component_descriptor("workspace.component.editor"))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_component_ids(
        report.accepted_snapshot().components(),
        &["workspace.component.valid"],
    );
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "workspace.component.editor",
            ),
            (
                CapabilityDiagnosticCode::DuplicateCapabilityId,
                "workspace.component.editor",
            ),
        ],
    );
}

#[test]
fn component_with_untyped_props_rejected() {
    let report = WorthUi::app()
        .register_component(ComponentDescriptor::new(
            component_id("workspace.component.editor"),
            ComponentPropSchema::untyped_for_diagnostics("workspace.editor.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().components().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingComponentPropSchema],
    );
}

#[test]
fn component_missing_state_ownership_rejected() {
    let report = WorthUi::app()
        .register_component(
            ComponentDescriptor::without_state_ownership_for_diagnostics(
                component_id("workspace.component.editor"),
                ComponentPropSchema::named("workspace.editor.props"),
                ComponentChildPolicy::no_children(),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().components().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingComponentStateOwnership],
    );
}

#[test]
fn component_with_illegal_child_policy_rejected() {
    let report = WorthUi::app()
        .register_component(ComponentDescriptor::new(
            component_id("workspace.component.editor"),
            ComponentPropSchema::named("workspace.editor.props"),
            ComponentChildPolicy::shell_layout_authority_for_diagnostics(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().components().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::IllegalComponentChildPolicy],
    );
}

#[test]
fn component_references_missing_theme_token_rejected() {
    let missing_token_id =
        ThemeTokenId::new("workspace.theme_token.accent").expect("valid token id");
    let report = WorthUi::app()
        .register_component(
            component_descriptor("workspace.component.editor")
                .with_theme_token_dependency(missing_token_id),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().components().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingDependency,
            "workspace.component.editor",
            "theme_token",
            "workspace.theme_token.accent",
        )],
    );
}

#[test]
fn component_missing_theme_token_does_not_poison_valid_component() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.valid"))
        .register_component(
            component_descriptor("workspace.component.editor").with_theme_token_dependency(
                ThemeTokenId::new("workspace.theme_token.accent").expect("valid token id"),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_component_ids(
        report.accepted_snapshot().components(),
        &["workspace.component.valid"],
    );
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingDependency,
            "workspace.component.editor",
        )],
    );
}

#[test]
fn component_references_missing_command_binding_rejected() {
    let report = WorthUi::app()
        .register_component(
            component_descriptor("workspace.component.editor")
                .with_command_binding_slot(command_id("workspace.command.open")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().components().is_empty());
    assert_dependency_diagnostics(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingDependency,
            "workspace.component.editor",
            "command",
            "workspace.command.open",
        )],
    );
}

#[test]
fn component_command_binding_resolves_against_registered_command() {
    let app = WorthUi::app()
        .register_command(CommandDescriptor::new(
            command_id("workspace.command.open"),
            "Open Workspace",
        ))
        .register_component(
            component_descriptor("workspace.component.editor")
                .with_command_binding_slot(command_id("workspace.command.open")),
        )
        .freeze()
        .expect("application preparation should succeed");

    let descriptor = app
        .capabilities()
        .components()
        .get(&component_id("workspace.component.editor"))
        .expect("registered component");
    assert_eq!(
        descriptor.command_binding_slots(),
        &[command_id("workspace.command.open")]
    );
}

#[test]
fn invalid_component_descriptor_does_not_poison_valid_component() {
    let report = WorthUi::app()
        .register_component(component_descriptor("workspace.component.valid"))
        .register_component(ComponentDescriptor::without_prop_schema_for_diagnostics(
            component_id("workspace.component.invalid"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_component_ids(
        report.accepted_snapshot().components(),
        &["workspace.component.valid"],
    );
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[(
            CapabilityDiagnosticCode::MissingComponentPropSchema,
            "workspace.component.invalid",
        )],
    );
}

#[test]
fn component_descriptor_reports_multiple_independent_violations() {
    let report = WorthUi::app()
        .register_component(ComponentDescriptor::new(
            component_id("workspace.component.invalid"),
            ComponentPropSchema::untyped_for_diagnostics("workspace.invalid.props"),
            ComponentChildPolicy::shell_layout_authority_for_diagnostics(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().components().is_empty());
    assert_diagnostic_codes_and_identities(
        report.registration_diagnostics(),
        &[
            (
                CapabilityDiagnosticCode::MissingComponentPropSchema,
                "workspace.component.invalid",
            ),
            (
                CapabilityDiagnosticCode::IllegalComponentChildPolicy,
                "workspace.component.invalid",
            ),
        ],
    );
}

#[test]
fn different_component_descriptor_meaning_produces_different_snapshot_digest() {
    let passive = WorthUi::app()
        .register_component(component_descriptor("workspace.component.editor"))
        .freeze()
        .expect("application preparation should succeed");
    let interactive = WorthUi::app()
        .register_component(
            component_descriptor("workspace.component.editor")
                .with_accessibility(ComponentAccessibilitySupport::semantic())
                .with_focus(ComponentFocusSupport::focusable())
                .with_execution_lane(ComponentExecutionLane::Interactive),
        )
        .freeze()
        .expect("application preparation should succeed");

    assert_ne!(
        passive.capabilities().components(),
        interactive.capabilities().components()
    );
    assert_ne!(
        passive.capabilities().digest(),
        interactive.capabilities().digest()
    );
}

#[test]
fn component_accessibility_focus_and_execution_metadata_survive_freeze() {
    let app = WorthUi::app()
        .register_component(
            component_descriptor("workspace.component.editor")
                .with_accessibility(ComponentAccessibilitySupport::decorative_only())
                .with_focus(ComponentFocusSupport::focus_container())
                .with_execution_lane(ComponentExecutionLane::Virtualized),
        )
        .freeze()
        .expect("application preparation should succeed");

    let descriptor = app
        .capabilities()
        .components()
        .get(&component_id("workspace.component.editor"))
        .expect("registered component");
    assert_eq!(
        descriptor.accessibility(),
        ComponentAccessibilitySupport::decorative_only()
    );
    assert_eq!(descriptor.focus(), ComponentFocusSupport::focus_container());
    assert_eq!(
        descriptor.execution_lane(),
        ComponentExecutionLane::Virtualized
    );
}
