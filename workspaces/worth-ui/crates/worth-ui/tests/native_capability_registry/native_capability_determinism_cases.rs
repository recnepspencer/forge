use worth_ui::facade::{
    NativeCapabilityDescriptor, NativeCapabilityFamily, NativePlatformPosture, WorthUi,
};

use super::native_capability_assertions::assert_registered_native_capability_ids;
use super::native_capability_fixtures::{native_capability, native_capability_id};

#[test]
fn equivalent_native_capabilities_produce_equivalent_support_entries() {
    let first = WorthUi::app()
        .register_native_capability(native_capability("platform.native.clipboard"))
        .register_native_capability(native_capability("platform.native.file_dialog"))
        .freeze();
    let second = WorthUi::app()
        .register_native_capability(native_capability("platform.native.file_dialog"))
        .register_native_capability(native_capability("platform.native.clipboard"))
        .freeze();

    assert_eq!(
        first.capabilities().native_capabilities(),
        second.capabilities().native_capabilities()
    );
    assert_eq!(
        first.capabilities().digest(),
        second.capabilities().digest()
    );
    assert_registered_native_capability_ids(
        first.capabilities().native_capabilities(),
        &["platform.native.clipboard", "platform.native.file_dialog"],
    );
}

#[test]
fn all_domain_agnostic_builtin_native_capability_families_are_admitted() {
    let app = all_builtin_native_families()
        .into_iter()
        .enumerate()
        .fold(WorthUi::app(), |builder, (index, family)| {
            builder.register_native_capability(
                NativeCapabilityDescriptor::new(native_capability_id(&format!(
                    "platform.native.builtin_{index}"
                )))
                .with_family(family)
                .with_platform_posture(NativePlatformPosture::runtime_declared()),
            )
        })
        .freeze();

    assert_eq!(app.capabilities().native_capabilities().len(), 9);
    assert_eq!(
        app.capabilities()
            .native_capabilities()
            .entries()
            .iter()
            .map(|entry| entry
                .descriptor()
                .family()
                .expect("native capability family")
                .clone())
            .collect::<Vec<_>>(),
        all_builtin_native_families()
    );
    assert!(app
        .capabilities()
        .native_capabilities()
        .entries()
        .iter()
        .all(|entry| entry.descriptor().platform_posture()
            == Some(NativePlatformPosture::runtime_declared())));
}

#[test]
fn frozen_native_capabilities_support_index_lookup_by_typed_id() {
    let native_id = native_capability_id("platform.native.clipboard");
    let app = WorthUi::app()
        .register_native_capability(native_capability("platform.native.clipboard_backup"))
        .register_native_capability(native_capability(native_id.as_str()))
        .freeze();

    let descriptor = app
        .capabilities()
        .native_capabilities()
        .get(&native_id)
        .expect("native capability");

    assert_eq!(descriptor.id(), &native_id);
    assert_eq!(
        descriptor.family(),
        Some(&NativeCapabilityFamily::clipboard())
    );
}

#[test]
fn explicit_unsupported_platform_posture_is_registered_as_declared_support_seam() {
    let native_id = native_capability_id("platform.native.clipboard");
    let app = WorthUi::app()
        .register_native_capability(
            NativeCapabilityDescriptor::new(native_id.clone())
                .with_family(NativeCapabilityFamily::clipboard())
                .with_platform_posture(NativePlatformPosture::unsupported()),
        )
        .freeze();

    let descriptor = app
        .capabilities()
        .native_capabilities()
        .get(&native_id)
        .expect("explicit native capability support seam");

    assert_eq!(
        descriptor.platform_posture(),
        Some(NativePlatformPosture::unsupported())
    );
}

#[test]
fn different_native_platform_posture_changes_snapshot_digest() {
    let runtime_declared = WorthUi::app()
        .register_native_capability(native_capability("platform.native.clipboard"))
        .freeze();
    let deferred = WorthUi::app()
        .register_native_capability(
            native_capability("platform.native.clipboard")
                .with_platform_posture(NativePlatformPosture::deferred()),
        )
        .freeze();

    assert_ne!(
        runtime_declared.capabilities().native_capabilities(),
        deferred.capabilities().native_capabilities()
    );
    assert_ne!(
        runtime_declared.capabilities().digest(),
        deferred.capabilities().digest()
    );
}

fn all_builtin_native_families() -> Vec<NativeCapabilityFamily> {
    vec![
        NativeCapabilityFamily::native_menu_adapter(),
        NativeCapabilityFamily::file_dialog(),
        NativeCapabilityFamily::clipboard(),
        NativeCapabilityFamily::drag_drop(),
        NativeCapabilityFamily::notification(),
        NativeCapabilityFamily::tray(),
        NativeCapabilityFamily::url_file_association(),
        NativeCapabilityFamily::os_theme(),
        NativeCapabilityFamily::keychain(),
    ]
}
