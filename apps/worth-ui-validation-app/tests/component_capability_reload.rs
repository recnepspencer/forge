mod component_capability_reload_support;

use component_capability_reload_support::{
    component_id, runtime_workbench, ComponentReloadLoopFixture,
};
use worth_ui::facade::{
    WorthUiAppearanceReloadPackage, WorthUiCapabilityReloadRequest, WorthUiCapabilityReloadStatus,
    WorthUiComponentCompatibility, WorthUiComponentReloadPackage, WorthUiComponentStateDropReason,
    WorthUiPageHostRebindStatus, WorthUiRuntimeFactId,
};
use worth_ui_validation_app::reload::{
    ValidationComponentSource, ValidationReloadInput, ValidationReloadTick,
    ValidationRuntimeReloadTickOutcome,
};

#[test]
fn component_reload_emits_exact_component_fact_and_preserve_receipt() {
    let mut workbench = runtime_workbench();
    let component_id = component_id("validation.component.header.dropdown");
    let before = workbench
        .runtime()
        .inspect_active_component_descriptor(&component_id)
        .expect("dropdown component should exist")
        .clone();
    let source = ValidationComponentSource::new(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive
command_binding_slots = validation.command.header.reload",
    );

    let prepared = workbench.prepare_component_capability_reload(&source);
    assert!(prepared.is_ready());
    let evidence = workbench
        .activate_capability_reload(prepared)
        .expect("component reload should activate");

    assert!(matches!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::ReadyForFrameBoundary
            | WorthUiCapabilityReloadStatus::Activated
    ));
    assert_eq!(evidence.touched_component_count(), 1);
    assert_eq!(evidence.changed_component_count(), 1);
    assert_eq!(evidence.changed_facts().len(), 1);
    assert!(evidence
        .changed_facts()
        .contains(&WorthUiRuntimeFactId::component(&component_id)));
    let receipt = evidence
        .component_reload_receipt()
        .expect("component reload should emit a runtime-owned receipt");
    assert_eq!(receipt.component_ids(), &[component_id.clone()]);
    assert!(matches!(
        receipt.compatibility(),
        WorthUiComponentCompatibility::CompatiblePreserveState(_)
    ));
    let after = workbench
        .runtime()
        .inspect_active_component_descriptor(&component_id)
        .expect("dropdown component should remain present");
    assert_eq!(after.prop_schema(), before.prop_schema());
    assert_eq!(after.state_ownership(), before.state_ownership());
    assert_eq!(after.child_policy(), before.child_policy());
    assert_eq!(after.focus(), before.focus());
    assert_eq!(after.execution_lane(), before.execution_lane());
    assert_eq!(after.command_binding_slots().len(), 1);
    assert_eq!(
        after.command_binding_slots()[0].as_str(),
        "validation.command.header.reload"
    );
}

#[test]
fn component_reload_denies_state_ownership_change_before_activation() {
    let workbench = runtime_workbench();
    let component_id = component_id("validation.component.header.dropdown");
    let before = workbench
        .runtime()
        .inspect_active_component_descriptor(&component_id)
        .expect("dropdown component should exist")
        .clone();
    let source = ValidationComponentSource::new(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = stateless
focus = focusable
execution_lane = interactive",
    );

    let prepared = workbench.prepare_component_capability_reload(&source);
    let evidence = prepared.evidence();

    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(
            worth_ui::facade::WorthUiCapabilityReloadStage::ComponentAdmission
        )
    );
    assert_eq!(evidence.changed_facts().len(), 0);
    assert!(evidence
        .denial_detail()
        .is_some_and(|detail| detail.contains("state ownership")));
    let after = workbench
        .runtime()
        .inspect_active_component_descriptor(&component_id)
        .expect("denied reload must preserve active descriptor");
    assert_eq!(after, &before);
}

#[test]
fn component_reload_denies_focus_change_before_activation() {
    let workbench = runtime_workbench();
    let source = ValidationComponentSource::new(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focus_container
execution_lane = interactive",
    );

    let prepared = workbench.prepare_component_capability_reload(&source);
    let evidence = prepared.evidence();

    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(
            worth_ui::facade::WorthUiCapabilityReloadStage::ComponentAdmission
        )
    );
    assert!(evidence
        .denial_detail()
        .is_some_and(|detail| detail.contains("focus posture")));
}

#[test]
fn component_reload_can_drop_state_with_explicit_reason() {
    let mut workbench = runtime_workbench();
    let source = ValidationComponentSource::new(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.alt_props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive",
    );

    let prepared = workbench.prepare_component_capability_reload(&source);
    assert!(prepared.is_ready());
    let evidence = workbench
        .activate_capability_reload(prepared)
        .expect("component reload should activate");

    assert!(matches!(
        evidence
            .component_reload_receipt()
            .expect("component reload should emit a receipt")
            .compatibility(),
        WorthUiComponentCompatibility::CompatibleDropState(
            WorthUiComponentStateDropReason::PropSchemaIncompatible { .. }
        )
    ));
    assert_eq!(evidence.changed_facts().len(), 1);
}

#[test]
fn component_reload_loop_observes_file_edits_and_updates_runtime_descriptor_truth() {
    let mut workbench = runtime_workbench();
    let fixture = ComponentReloadLoopFixture::new();
    let component_id = component_id("validation.component.header.dropdown");
    let before = workbench
        .runtime()
        .inspect_active_component_descriptor(&component_id)
        .expect("dropdown component should exist")
        .clone();
    let mut reload_loop = fixture.start_loop();

    assert!(matches!(
        reload_loop.poll_inputs(),
        ValidationReloadTick::Unchanged(_)
    ));

    fixture.write_component(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive
command_binding_slots = validation.command.header.reload",
    );

    let tick = reload_loop.poll_inputs();
    let ValidationReloadTick::Changed(ValidationReloadInput::HeaderComponents(component)) = tick
    else {
        panic!("observed file edit should produce a typed component reload tick");
    };
    assert_eq!(component.source_path(), fixture.component_path.as_path());

    let ValidationRuntimeReloadTickOutcome::ComponentReloaded {
        evidence,
        phase_execution,
        ..
    } = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderComponents(component),
    ))
    else {
        panic!("component tick should route through the runtime workbench");
    };
    let phase_execution = phase_execution.expect("component reload should emit phase execution");

    assert_eq!(evidence.status(), WorthUiCapabilityReloadStatus::Activated);
    assert_eq!(
        evidence
            .component_reload_receipt()
            .expect("component reload should emit a receipt")
            .component_ids(),
        &[component_id.clone()]
    );
    assert_eq!(
        phase_execution.page_host_rebind().status(),
        WorthUiPageHostRebindStatus::EquivalentAfterActivation
    );
    let after = workbench
        .runtime()
        .inspect_active_component_descriptor(&component_id)
        .expect("activated component should remain present");
    assert_eq!(after.prop_schema(), before.prop_schema());
    assert_eq!(after.command_binding_slots().len(), 1);
    assert_eq!(
        after.command_binding_slots()[0].as_str(),
        "validation.command.header.reload"
    );
}

#[test]
fn component_reload_denies_child_policy_change_before_activation() {
    let workbench = runtime_workbench();
    let source = ValidationComponentSource::new(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = text_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive",
    );

    let prepared = workbench.prepare_component_capability_reload(&source);
    let evidence = prepared.evidence();

    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(
            worth_ui::facade::WorthUiCapabilityReloadStage::ComponentAdmission
        )
    );
    assert!(evidence
        .denial_detail()
        .is_some_and(|detail| detail.contains("child policy")));
}

#[test]
fn component_reload_denies_accessibility_change_before_activation() {
    let workbench = runtime_workbench();
    let source = ValidationComponentSource::new(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
accessibility = decorative_only
execution_lane = interactive",
    );

    let prepared = workbench.prepare_component_capability_reload(&source);
    let evidence = prepared.evidence();

    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(
            worth_ui::facade::WorthUiCapabilityReloadStage::ComponentAdmission
        )
    );
    assert!(evidence
        .denial_detail()
        .is_some_and(|detail| detail.contains("accessibility posture")));
}

#[test]
fn component_reload_denies_execution_lane_change_before_activation() {
    let workbench = runtime_workbench();
    let source = ValidationComponentSource::new(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = passive",
    );

    let prepared = workbench.prepare_component_capability_reload(&source);
    let evidence = prepared.evidence();

    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(
            worth_ui::facade::WorthUiCapabilityReloadStage::ComponentAdmission
        )
    );
    assert!(evidence
        .denial_detail()
        .is_some_and(|detail| detail.contains("execution lane")));
}

#[test]
fn mixed_component_and_appearance_reload_stays_atomic() {
    let mut workbench = runtime_workbench();
    let component_reload = WorthUiComponentReloadPackage::from_source(
        "apps/worth-ui-validation-app/theme/header.components",
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive
command_binding_slots = validation.command.header.reload",
    );
    let appearance_reload = WorthUiAppearanceReloadPackage::from_source(
        "apps/worth-ui-validation-app/theme/header.appearance",
        "validation.appearance.header.menu_min_width = 260px",
    );

    let prepared =
        workbench
            .runtime()
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::batch([
                WorthUiCapabilityReloadRequest::from_components(component_reload),
                WorthUiCapabilityReloadRequest::from_appearance(appearance_reload),
            ]));
    let evidence = workbench
        .activate_capability_reload(prepared)
        .expect("component and appearance reload should activate atomically");

    assert_eq!(evidence.family_rows().len(), 2);
    assert_eq!(evidence.changed_facts().len(), 2);
    assert_eq!(evidence.touched_component_count(), 1);
    assert_eq!(evidence.touched_appearance_count(), 1);
    assert!(evidence.family_rows().iter().any(|row| {
        row.family() == worth_ui::facade::WorthUiCapabilityReloadFamilyKind::Components
    }));
    assert!(evidence.family_rows().iter().any(|row| {
        row.family() == worth_ui::facade::WorthUiCapabilityReloadFamilyKind::Appearance
    }));
}
