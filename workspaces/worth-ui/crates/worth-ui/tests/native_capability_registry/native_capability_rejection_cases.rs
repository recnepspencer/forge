use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        NativeCapabilityDescriptor, NativeCapabilityFamily, NativePlatformPosture,
        NativeShellAuthorityClaim,
    },
    diagnostics::CapabilityDiagnosticCode,
    support::AmbientHostCheck,
};

use super::native_capability_assertions::{
    assert_diagnostic_codes, assert_registered_native_capability_ids,
};
use super::native_capability_fixtures::{native_capability, native_capability_id};

#[test]
fn unsupported_native_family_rejected() {
    let report = WorthUi::app()
        .register_native_capability(
            native_capability("platform.native.unsupported").with_family(
                NativeCapabilityFamily::unsupported_for_diagnostics("ambient_desktop_bus"),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().native_capabilities().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::UnsupportedNativeCapabilityFamily],
    );
}

#[test]
fn native_capability_missing_platform_posture_rejected() {
    let report = WorthUi::app()
        .register_native_capability(
            NativeCapabilityDescriptor::new(native_capability_id("platform.native.clipboard"))
                .with_family(NativeCapabilityFamily::clipboard()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().native_capabilities().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingNativePlatformPosture],
    );
}

#[test]
fn native_capability_missing_family_rejected() {
    let report = WorthUi::app()
        .register_native_capability(
            NativeCapabilityDescriptor::new(native_capability_id("platform.native.unknown"))
                .with_platform_posture(NativePlatformPosture::runtime_declared()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().native_capabilities().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::MissingNativeCapabilityFamily],
    );
}

#[test]
fn duplicate_native_capability_id_rejected_before_snapshot_freeze() {
    let report = WorthUi::app()
        .register_native_capability(native_capability("platform.native.clipboard"))
        .register_native_capability(
            native_capability("platform.native.clipboard")
                .with_platform_posture(NativePlatformPosture::deferred()),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().native_capabilities().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
        ],
    );
}

#[test]
fn native_adapter_claims_shell_authority_rejected() {
    let report = WorthUi::app()
        .register_native_capability(
            native_capability("platform.native.menu").with_shell_authority_claim_for_diagnostics(
                NativeShellAuthorityClaim::redefines_shell_semantics_for_diagnostics(),
            ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().native_capabilities().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::NativeAdapterClaimsShellAuthority],
    );
}

#[test]
fn ambient_host_check_cannot_replace_native_capability_posture() {
    let report = WorthUi::app()
        .register_native_capability(
            NativeCapabilityDescriptor::new(native_capability_id("platform.native.clipboard"))
                .with_family(NativeCapabilityFamily::clipboard())
                .with_platform_posture(NativePlatformPosture::runtime_declared())
                .with_ambient_host_check_for_diagnostics(
                    AmbientHostCheck::current_host_for_diagnostics(),
                ),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().native_capabilities().is_empty());
    assert_diagnostic_codes(
        report.registration_diagnostics(),
        &[CapabilityDiagnosticCode::AmbientHostCheckCannotReplaceNativeCapabilityPosture],
    );
}

#[test]
fn invalid_native_capability_does_not_poison_valid_native_capability() {
    let report = WorthUi::app()
        .register_native_capability(native_capability("platform.native.clipboard"))
        .register_native_capability(NativeCapabilityDescriptor::new(native_capability_id(
            "platform.native.empty",
        )))
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert_registered_native_capability_ids(
        report.accepted_snapshot().native_capabilities(),
        &["platform.native.clipboard"],
    );
}
