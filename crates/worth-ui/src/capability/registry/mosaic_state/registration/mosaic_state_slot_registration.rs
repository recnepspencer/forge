use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, MOSAIC_STATE_SLOT_FAMILY_NAME,
};

use super::super::MosaicStateSlotDescriptor;

impl MosaicStateSlotDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            MOSAIC_STATE_SLOT_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_mosaic_state_slot_diagnostics(candidate, self)
    }
}

fn add_mosaic_state_slot_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicStateSlotDescriptor,
) -> RegistrationCandidate {
    candidate = add_missing_owner_identity_diagnostic(candidate, descriptor);
    candidate = add_missing_persistence_policy_diagnostic(candidate, descriptor);
    candidate = add_missing_replacement_rule_diagnostic(candidate, descriptor);
    candidate = add_missing_truth_posture_diagnostic(candidate, descriptor);
    add_authoritative_truth_diagnostic(candidate, descriptor)
}

fn add_missing_owner_identity_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicStateSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .owner_identity()
        .is_none_or(|identity| identity.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicStateSlotOwnerIdentity,
            "mosaic state slot must declare stable owner identity",
        );
    }
    candidate
}

fn add_missing_persistence_policy_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicStateSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .persistence_policy()
        .is_none_or(|policy| policy.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicStateSlotPersistencePolicy,
            "mosaic state slot must declare persistence posture",
        );
    }
    candidate
}

fn add_missing_replacement_rule_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicStateSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .replacement_rule()
        .is_none_or(|rule| rule.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicStateSlotReplacementRule,
            "mosaic state slot must declare reload replacement behavior",
        );
    }
    candidate
}

fn add_missing_truth_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicStateSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .truth_posture()
        .is_none_or(|posture| posture.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicStateSlotTruthPosture,
            "mosaic state slot must declare non-authoritative truth posture",
        );
    }
    candidate
}

fn add_authoritative_truth_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicStateSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .truth_posture()
        .is_some_and(|posture| posture.is_authoritative_truth_claim())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::AuthoritativeTruthMosaicStateSlot,
            "mosaic state slots cannot claim Query or relational truth authority",
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
