use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, NATIVE_CAPABILITY_FAMILY_NAME,
};

use super::super::NativeCapabilityDescriptor;

impl NativeCapabilityDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            NATIVE_CAPABILITY_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_native_capability_diagnostics(candidate, self)
    }
}

fn add_native_capability_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &NativeCapabilityDescriptor,
) -> RegistrationCandidate {
    candidate = add_missing_family_diagnostic(candidate, descriptor);
    candidate = add_unsupported_family_diagnostic(candidate, descriptor);
    candidate = add_missing_platform_posture_diagnostic(candidate, descriptor);
    candidate = add_shell_authority_claim_diagnostic(candidate, descriptor);
    add_ambient_host_check_diagnostic(candidate, descriptor)
}

fn add_missing_family_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &NativeCapabilityDescriptor,
) -> RegistrationCandidate {
    if descriptor.family().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingNativeCapabilityFamily,
            "native capabilities must declare a built-in platform capability family",
        );
    }
    candidate
}

fn add_unsupported_family_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &NativeCapabilityDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .family()
        .is_some_and(|family| !family.is_supported())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnsupportedNativeCapabilityFamily,
            "native capabilities must use built-in platform capability families",
        );
    }
    candidate
}

fn add_missing_platform_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &NativeCapabilityDescriptor,
) -> RegistrationCandidate {
    if descriptor.platform_posture().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingNativePlatformPosture,
            "native capabilities must declare explicit platform support posture",
        );
    }
    candidate
}

fn add_shell_authority_claim_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &NativeCapabilityDescriptor,
) -> RegistrationCandidate {
    if !descriptor.shell_authority_claims().is_empty() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::NativeAdapterClaimsShellAuthority,
            "native adapters cannot redefine shell or runtime semantics",
        );
    }
    candidate
}

fn add_ambient_host_check_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &NativeCapabilityDescriptor,
) -> RegistrationCandidate {
    if !descriptor.ambient_host_checks().is_empty() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::AmbientHostCheckCannotReplaceNativeCapabilityPosture,
            "native support must be registered explicitly instead of inferred from the host",
        );
    }
    candidate
}

fn with_descriptor_diagnostic(
    candidate: RegistrationCandidate,
    code: CapabilityDiagnosticCode,
    message: &'static str,
) -> RegistrationCandidate {
    candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(code, message))
}
