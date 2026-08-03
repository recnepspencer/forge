use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        CommandDescriptor, ComponentAccessibilitySupport, ComponentExecutionLane,
        ComponentFocusSupport,
    },
};

#[path = "component_registry/component_admission_contracts.rs"]
mod component_admission_contracts;
#[path = "component_registry/component_registry_assertions.rs"]
mod component_registry_assertions;
#[path = "component_registry/component_registry_fixtures.rs"]
mod component_registry_fixtures;
#[path = "component_registry/component_visual_contracts.rs"]
mod component_visual_contracts;

use component_registry_assertions::assert_registered_component_ids;
use component_registry_fixtures::{command_id, component_descriptor, component_id};

#[test]
fn equivalent_component_descriptors_produce_equivalent_entries() {
    let first = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_component(component_descriptor("workspace.component.editor"))
        .register_component(component_descriptor("workspace.component.sidebar"))
        .freeze()
        .expect("application preparation should succeed");
    let second = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
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
fn component_command_binding_resolves_against_registered_command() {
    let app = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
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
fn different_component_descriptor_meaning_produces_different_snapshot_digest() {
    let passive = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
        .register_component(component_descriptor("workspace.component.editor"))
        .freeze()
        .expect("application preparation should succeed");
    let interactive = WorthUi::app()
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
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
        .with_change_profile(worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse())
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
