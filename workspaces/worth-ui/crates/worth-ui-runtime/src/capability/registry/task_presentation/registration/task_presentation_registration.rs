use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, TASK_PRESENTATION_FAMILY_NAME,
};

use super::super::{
    TaskPresentationCancellationPosture, TaskPresentationDescriptor,
    TaskPresentationFailurePosture, TaskPresentationFamily,
};

impl TaskPresentationDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            TASK_PRESENTATION_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_task_presentation_diagnostics(candidate, self)
    }
}

fn add_task_presentation_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    candidate = add_unknown_family_diagnostic(candidate, descriptor);
    candidate = add_missing_lifecycle_posture_diagnostic(candidate, descriptor);
    candidate = add_missing_cancellation_posture_diagnostic(candidate, descriptor);
    candidate = add_missing_failure_posture_diagnostic(candidate, descriptor);
    candidate = add_missing_projection_eligibility_diagnostic(candidate, descriptor);
    candidate = add_missing_runtime_authority_posture_diagnostic(candidate, descriptor);
    candidate = add_family_posture_mismatch_diagnostic(candidate, descriptor);
    candidate = add_family_projection_mismatch_diagnostic(candidate, descriptor);
    add_runtime_authority_claim_diagnostic(candidate, descriptor)
}

fn add_unknown_family_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    if !descriptor.family().is_known() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnknownTaskPresentationFamily,
            "task presentation family must be a built-in domain-agnostic presentation family",
        );
    }
    candidate
}

fn add_missing_lifecycle_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    if descriptor.lifecycle_posture().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingTaskPresentationLifecyclePosture,
            "task presentation must declare lifecycle posture",
        );
    }
    candidate
}

fn add_missing_cancellation_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    if descriptor.cancellation_posture().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingTaskPresentationCancellationPosture,
            "task presentation must declare cancellation posture",
        );
    }
    candidate
}

fn add_missing_failure_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    if descriptor.failure_posture().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingTaskPresentationFailurePosture,
            "task presentation must declare failure posture",
        );
    }
    candidate
}

fn add_missing_projection_eligibility_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    if descriptor.projection_eligibility().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingTaskPresentationProjectionEligibility,
            "task presentation must declare projection eligibility",
        );
    }
    candidate
}

fn add_missing_runtime_authority_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    if descriptor.runtime_authority_posture().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingTaskPresentationRuntimeAuthorityPosture,
            "task presentation must declare that it does not own task execution",
        );
    }
    candidate
}

fn add_family_posture_mismatch_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    if family_posture_mismatch(descriptor) {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::TaskPresentationFamilyPostureMismatch,
            "task presentation family must match its explicit cancellation and failure posture",
        );
    }
    candidate
}

fn add_family_projection_mismatch_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    if family_projection_eligibility_mismatch(descriptor) {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::TaskPresentationFamilyProjectionMismatch,
            "task presentation family must match its explicit projection eligibility",
        );
    }
    candidate
}

fn family_posture_mismatch(descriptor: &TaskPresentationDescriptor) -> bool {
    cancellable_family_without_cancellation_posture(descriptor)
        || retryable_family_without_retry_posture(descriptor)
}

fn cancellable_family_without_cancellation_posture(
    descriptor: &TaskPresentationDescriptor,
) -> bool {
    matches!(descriptor.family(), TaskPresentationFamily::Cancellable)
        && !descriptor
            .cancellation_posture()
            .is_some_and(TaskPresentationCancellationPosture::exposes_cancellation)
}

fn retryable_family_without_retry_posture(descriptor: &TaskPresentationDescriptor) -> bool {
    matches!(descriptor.family(), TaskPresentationFamily::Retryable)
        && !descriptor
            .failure_posture()
            .is_some_and(TaskPresentationFailurePosture::exposes_retry)
}

fn family_projection_eligibility_mismatch(descriptor: &TaskPresentationDescriptor) -> bool {
    let Some(eligibility) = descriptor.projection_eligibility() else {
        return false;
    };

    !descriptor
        .family()
        .admits_projection_eligibility(eligibility)
}

fn add_runtime_authority_claim_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &TaskPresentationDescriptor,
) -> RegistrationCandidate {
    let claims_runtime_authority = descriptor
        .lifecycle_posture()
        .is_some_and(|posture| posture.claims_task_runtime_authority())
        || descriptor
            .cancellation_posture()
            .is_some_and(|posture| posture.claims_task_runtime_authority())
        || descriptor
            .failure_posture()
            .is_some_and(|posture| posture.claims_task_runtime_authority())
        || descriptor
            .runtime_authority_posture()
            .is_some_and(|posture| posture.claims_task_runtime_authority());

    if claims_runtime_authority {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::TaskPresentationClaimsTaskRuntimeAuthority,
            "task presentation cannot own execution, cancellation, retry, or task lifecycle truth",
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
