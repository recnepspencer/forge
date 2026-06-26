use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, RegistrationDependency, ICON_FAMILY_NAME,
    RUNTIME_OUTCOME_PROJECTION_FAMILY_NAME,
};

use super::super::RuntimeOutcomeProjectionDescriptor;

impl RuntimeOutcomeProjectionDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            RUNTIME_OUTCOME_PROJECTION_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_runtime_outcome_projection_diagnostics(candidate, self)
    }
}

fn add_runtime_outcome_projection_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    candidate = add_unknown_family_diagnostic(candidate, descriptor);
    candidate = add_missing_source_diagnostic(candidate, descriptor);
    candidate = add_family_source_mismatch_diagnostic(candidate, descriptor);
    candidate = add_local_status_enum_diagnostic(candidate, descriptor);
    candidate = add_missing_denial_posture_diagnostic(candidate, descriptor);
    candidate = add_unexpected_denial_posture_diagnostic(candidate, descriptor);
    candidate = add_missing_recovery_posture_diagnostic(candidate, descriptor);
    candidate = add_unexpected_recovery_posture_diagnostic(candidate, descriptor);
    add_icon_dependency(candidate, descriptor)
}

fn add_icon_dependency(
    candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    match descriptor
        .presentation()
        .and_then(|presentation| presentation.icon())
    {
        Some(icon) => candidate.with_dependency(RegistrationDependency::new(
            ICON_FAMILY_NAME,
            ICON_FAMILY_NAME,
            icon.as_str(),
        )),
        None => candidate,
    }
}

fn add_family_source_mismatch_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor.family().is_known()
        && descriptor
            .source()
            .is_some_and(|source| !source.admits_family(descriptor.family()))
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::RuntimeOutcomeFamilySourceMismatch,
            "runtime outcome projection family must preserve Query or runtime-owned source meaning",
        );
    }
    candidate
}

fn add_unknown_family_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    if !descriptor.family().is_known() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnknownRuntimeOutcomeFamily,
            "runtime outcome projection must use a platform-known outcome family",
        );
    }
    candidate
}

fn add_missing_source_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor.source().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingRuntimeOutcomeSource,
            "runtime outcome projection must preserve Query or runtime-owned outcome posture",
        );
    }
    candidate
}

fn add_local_status_enum_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor.has_local_status_enum_claim() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::LocalStatusEnumRuntimeOutcomeProjection,
            "local UI status enums cannot replace structured runtime outcome posture",
        );
    }
    candidate
}

fn add_missing_denial_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor.family().requires_denial_posture() && descriptor.denial_posture().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingRuntimeOutcomeDenialPosture,
            "denial-capable outcome families must expose denial presentation posture",
        );
    }
    candidate
}

fn add_unexpected_denial_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    if !descriptor.family().admits_denial_posture() && descriptor.denial_posture().is_some() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnexpectedRuntimeOutcomeDenialPosture,
            "denial presentation posture cannot be attached to non-denial outcome families",
        );
    }
    candidate
}

fn add_missing_recovery_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor.family().requires_recovery_posture() && descriptor.recovery_posture().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingRuntimeOutcomeRecoveryPosture,
            "recoverable outcome families must expose recovery presentation posture",
        );
    }
    candidate
}

fn add_unexpected_recovery_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &RuntimeOutcomeProjectionDescriptor,
) -> RegistrationCandidate {
    if !descriptor.family().admits_recovery_posture() && descriptor.recovery_posture().is_some() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnexpectedRuntimeOutcomeRecoveryPosture,
            "recovery presentation posture cannot be attached to non-recovery outcome families",
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
