use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, VIEW_BINDING_FAMILY_NAME,
};

use super::super::ViewBindingDescriptor;

impl ViewBindingDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            VIEW_BINDING_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_view_binding_diagnostics(candidate, self)
    }
}

fn add_view_binding_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    candidate = add_missing_query_support_posture(candidate, descriptor);
    candidate = add_unsupported_query_support_posture(candidate, descriptor);
    candidate = add_missing_view_shape(candidate, descriptor);
    candidate = add_missing_basis_posture(candidate, descriptor);
    candidate = add_unsupported_basis_posture(candidate, descriptor);
    candidate = add_missing_result_shape(candidate, descriptor);
    candidate = add_missing_live_compatibility(candidate, descriptor);
    candidate = add_unsupported_live_compatibility(candidate, descriptor);
    candidate = add_missing_denial_presentation(candidate, descriptor);
    add_local_pseudo_query_diagnostic(candidate, descriptor)
}

fn add_missing_query_support_posture(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor.query_capability().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingViewBindingQuerySupportPosture,
            "view binding must preserve Query-owned support posture",
        );
    }
    candidate
}

fn add_unsupported_query_support_posture(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .query_capability()
        .is_some_and(|capability| !capability.is_admitted())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnsupportedViewBindingQuerySupportPosture,
            "view binding cannot claim support from unsupported or deferred Query posture",
        );
    }
    candidate
}

fn add_missing_view_shape(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor.view_shape().is_none() || descriptor.query_composition_profile_digest().is_none()
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingViewBindingViewShape,
            "view binding must preserve Query-owned view shape and composition support posture",
        );
    }
    candidate
}

fn add_missing_basis_posture(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor.basis_posture().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingViewBindingBasisPosture,
            "view binding must preserve Query-owned basis posture",
        );
    }
    candidate
}

fn add_unsupported_basis_posture(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .basis_posture()
        .is_some_and(|basis_posture| !basis_posture.is_admitted())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnsupportedViewBindingBasisPosture,
            "view binding basis posture must be admitted or advisory through Query",
        );
    }
    candidate
}

fn add_missing_result_shape(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor.result_shape().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingViewBindingResultShape,
            "view binding must preserve Query-owned result shape metadata",
        );
    }
    candidate
}

fn add_missing_live_compatibility(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor.live_compatibility().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingViewBindingLiveCompatibility,
            "view binding must preserve Query-owned live compatibility posture",
        );
    }
    candidate
}

fn add_unsupported_live_compatibility(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .live_compatibility()
        .is_some_and(|compatibility| !compatibility.is_admitted())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnsupportedViewBindingLiveCompatibility,
            "view binding live compatibility must preserve Query-certified subscription support",
        );
    }
    candidate
}

fn add_missing_denial_presentation(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor.denial_presentation().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingViewBindingDenialPresentation,
            "view binding must declare denial or advisory presentation posture",
        );
    }
    candidate
}

fn add_local_pseudo_query_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &ViewBindingDescriptor,
) -> RegistrationCandidate {
    if descriptor.has_local_pseudo_query_claim() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::LocalPseudoQueryViewBinding,
            "view binding cannot register UI-owned query or cache descriptors",
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
