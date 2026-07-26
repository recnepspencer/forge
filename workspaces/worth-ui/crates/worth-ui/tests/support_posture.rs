use worth_ui::facade::{
    declaration::{CommandId, ComponentId},
    support::{
        CapabilitySupportKind, CapabilitySupportPosture, SupportRequirement, SupportSnapshot,
    },
};

#[test]
fn equivalent_admitted_entries_preserve_support_posture() {
    let first_id = command_id("app.command.save");
    let second_id = command_id("app.command.save");

    let first = CapabilitySupportPosture::admitted(first_id);
    let second = CapabilitySupportPosture::admitted(second_id);

    assert_eq!(first, second);
    assert!(first.is_admitted());
    assert_eq!(first.kind(), CapabilitySupportKind::Admitted);

    let support_snapshot = SupportSnapshot::from_support_kinds([first.kind(), second.kind()]);

    assert_eq!(support_snapshot.admitted_count(), 2);
    assert_eq!(support_snapshot.deferred_count(), 0);
    assert_eq!(support_snapshot.unsupported_count(), 0);
    assert_eq!(support_snapshot.platform_internal_count(), 0);
    assert_eq!(support_snapshot.total_width(), 2);
}

#[test]
fn support_snapshot_preserves_mixed_posture_counts() {
    let support_snapshot = SupportSnapshot::from_support_kinds([
        CapabilitySupportKind::Admitted,
        CapabilitySupportKind::Deferred,
        CapabilitySupportKind::Unsupported,
        CapabilitySupportKind::PlatformInternal,
        CapabilitySupportKind::Deferred,
    ]);

    assert_eq!(support_snapshot.admitted_count(), 1);
    assert_eq!(support_snapshot.deferred_count(), 2);
    assert_eq!(support_snapshot.unsupported_count(), 1);
    assert_eq!(support_snapshot.platform_internal_count(), 1);
    assert_eq!(support_snapshot.total_width(), 5);
}

#[test]
fn deferred_entry_reference_rejected_as_not_admitted() {
    let deferred = CapabilitySupportPosture::deferred(command_id("app.command.preview"));

    let rejection = SupportRequirement::admitted()
        .check(deferred)
        .expect_err("deferred support must not satisfy admitted requirement");

    assert_eq!(rejection.id().as_str(), "app.command.preview");
    assert_eq!(rejection.required(), CapabilitySupportKind::Admitted);
    assert_eq!(rejection.actual(), CapabilitySupportKind::Deferred);
}

#[test]
fn unsupported_entry_reference_rejected_without_fallback() {
    let unsupported = CapabilitySupportPosture::unsupported(command_id("app.command.retired"));

    let rejection = SupportRequirement::admitted()
        .check(unsupported)
        .expect_err("unsupported support must not fallback to admitted");

    assert_eq!(rejection.id().as_str(), "app.command.retired");
    assert_eq!(rejection.required(), CapabilitySupportKind::Admitted);
    assert_eq!(rejection.actual(), CapabilitySupportKind::Unsupported);
}

#[test]
fn platform_internal_entry_reference_rejected_as_not_admitted() {
    let platform_internal =
        CapabilitySupportPosture::platform_internal(command_id("platform.command.reload"));

    let rejection = SupportRequirement::admitted()
        .check(platform_internal)
        .expect_err("platform-internal support must not become public admitted support");

    assert_eq!(rejection.id().as_str(), "platform.command.reload");
    assert_eq!(rejection.required(), CapabilitySupportKind::Admitted);
    assert_eq!(rejection.actual(), CapabilitySupportKind::PlatformInternal);
}

#[test]
fn support_rejection_preserves_required_actual_and_id() {
    let deferred = CapabilitySupportPosture::deferred(command_id("app.command.preview"));

    let rejection = SupportRequirement::admitted()
        .check(deferred)
        .expect_err("deferred support must reject admitted requirement");

    assert_eq!(rejection.id().as_str(), "app.command.preview");
    assert_eq!(rejection.required(), CapabilitySupportKind::Admitted);
    assert_eq!(rejection.actual(), CapabilitySupportKind::Deferred);
    assert_eq!(
        rejection.to_string(),
        "capability 'app.command.preview' required Admitted support but had Deferred support"
    );
}

#[test]
fn support_posture_is_generic_over_distinct_id_families() {
    let admitted_command = SupportRequirement::admitted()
        .check(CapabilitySupportPosture::admitted(command_id(
            "app.shared.action",
        )))
        .expect("command support should be admitted");
    let admitted_component = SupportRequirement::admitted()
        .check(CapabilitySupportPosture::admitted(component_id(
            "app.shared.action",
        )))
        .expect("component support should be admitted");

    assert_eq!(
        admitted_command.id().as_str(),
        admitted_component.id().as_str()
    );
    assert_eq!(admitted_command.kind(), CapabilitySupportKind::Admitted);
    assert_eq!(admitted_component.kind(), CapabilitySupportKind::Admitted);
}

fn command_id(raw_text: &str) -> CommandId {
    CommandId::new(raw_text).expect("valid command id")
}

fn component_id(raw_text: &str) -> ComponentId {
    ComponentId::new(raw_text).expect("valid component id")
}
