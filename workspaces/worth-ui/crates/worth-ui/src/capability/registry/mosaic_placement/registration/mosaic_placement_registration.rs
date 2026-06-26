use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
};

use super::{
    super::MosaicPlacementPolicyDescriptor,
    mosaic_placement_policy_legality::{
        is_cyclic_mosaic_containment_policy, is_illegal_mosaic_placement_policy,
    },
};

impl MosaicPlacementPolicyDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_mosaic_placement_descriptor_diagnostics(candidate, self)
    }
}

fn add_mosaic_placement_descriptor_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    candidate = add_missing_source_diagnostic(candidate, descriptor);
    candidate = add_missing_target_diagnostic(candidate, descriptor);
    candidate = add_missing_persistence_diagnostic(candidate, descriptor);
    candidate = add_missing_identity_diagnostic(candidate, descriptor);
    candidate = add_missing_conflict_diagnostic(candidate, descriptor);
    candidate = add_missing_reload_diagnostic(candidate, descriptor);
    candidate = add_imperative_mutation_diagnostic(candidate, descriptor);
    candidate = add_unsupported_float_or_overlay_diagnostic(candidate, descriptor);
    candidate = add_cyclic_containment_diagnostic(candidate, descriptor);
    add_illegal_source_target_diagnostic(candidate, descriptor)
}

fn add_missing_source_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if descriptor.source().is_none_or(|source| source.is_missing()) {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicPlacementSource,
            "mosaic placement policy must declare a source family",
        );
    }
    candidate
}

fn add_missing_target_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if descriptor.target().is_none_or(|target| target.is_missing()) {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicPlacementTarget,
            "mosaic placement policy must declare a target family",
        );
    }
    candidate
}

fn add_missing_persistence_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .persistence()
        .is_none_or(|persistence| persistence.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicPlacementPersistence,
            "mosaic placement policy must declare persistence behavior",
        );
    }
    candidate
}

fn add_missing_identity_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .stable_identity_behavior()
        .is_none_or(|identity| identity.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicPlacementStableIdentityBehavior,
            "mosaic placement policy must declare stable identity behavior",
        );
    }
    candidate
}

fn add_missing_conflict_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .conflict_behavior()
        .is_none_or(|conflict| conflict.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicPlacementConflictBehavior,
            "mosaic placement policy must declare conflict behavior",
        );
    }
    candidate
}

fn add_missing_reload_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .reload_reconciliation()
        .is_none_or(|reload| reload.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicPlacementReloadReconciliation,
            "mosaic placement policy must declare reload reconciliation posture",
        );
    }
    candidate
}

fn add_imperative_mutation_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if descriptor.action().is_imperative_mutation()
        || descriptor
            .source()
            .is_some_and(|source| source.is_imperative_mutation())
        || descriptor
            .target()
            .is_some_and(|target| target.is_imperative_mutation())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::ImperativeMosaicStateMutationPolicy,
            "mosaic placement policy must declare runtime-mediated placement, not mutation",
        );
    }
    candidate
}

fn add_unsupported_float_or_overlay_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    let supported = descriptor
        .support()
        .is_some_and(|support| support.supports_float_or_overlay());
    if descriptor.action().requires_float_or_overlay_support() && !supported {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnsupportedMosaicFloatOrOverlayPolicy,
            "float, modal, and overlay placement policies require explicit runtime support",
        );
    }
    candidate
}

fn add_cyclic_containment_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if is_cyclic_mosaic_containment_policy(descriptor.source(), descriptor.target()) {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::CyclicMosaicContainmentPolicy,
            "mosaic placement policy cannot place a region into itself",
        );
    }
    candidate
}

fn add_illegal_source_target_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicPlacementPolicyDescriptor,
) -> RegistrationCandidate {
    if is_illegal_mosaic_placement_policy(descriptor) {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::IllegalMosaicPlacementSourceTarget,
            "mosaic placement source and target families must be structurally compatible",
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
