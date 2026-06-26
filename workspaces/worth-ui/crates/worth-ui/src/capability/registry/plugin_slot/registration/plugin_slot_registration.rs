use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, RegistrationDependency, PLUGIN_SLOT_FAMILY_NAME,
};

use super::super::PluginSlotDescriptor;

impl PluginSlotDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            PLUGIN_SLOT_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_plugin_slot_diagnostics(candidate, self)
    }
}

fn add_plugin_slot_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    candidate = add_contribution_reference_dependency(candidate, descriptor);
    candidate = add_missing_family_diagnostic(candidate, descriptor);
    candidate = add_unsupported_family_diagnostic(candidate, descriptor);
    candidate = add_missing_permission_diagnostic(candidate, descriptor);
    candidate = add_missing_ordering_diagnostic(candidate, descriptor);
    candidate = add_missing_diagnostics_posture_diagnostic(candidate, descriptor);
    candidate = add_missing_support_posture_diagnostic(candidate, descriptor);
    add_global_mutation_hook_diagnostic(candidate, descriptor)
}

fn add_contribution_reference_dependency(
    candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    match descriptor.contribution_reference() {
        Some(reference) => candidate.with_dependency(RegistrationDependency::new(
            PLUGIN_SLOT_FAMILY_NAME,
            PLUGIN_SLOT_FAMILY_NAME,
            reference.slot_id().as_str(),
        )),
        None => candidate,
    }
}

fn add_missing_family_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor.allowed_families().is_empty() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingPluginSlotContributionFamily,
            "plugin slots must declare at least one admitted contribution family",
        );
    }
    candidate
}

fn add_unsupported_family_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .allowed_families()
        .iter()
        .any(|family| !family.is_supported())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnsupportedPluginContributionFamily,
            "plugin slots must use built-in domain-agnostic contribution families",
        );
    }
    candidate
}

fn add_missing_permission_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor.permission().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingPluginSlotPermission,
            "plugin slots must declare the permission posture that bounds contribution power",
        );
    }
    candidate
}

fn add_missing_ordering_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor.ordering().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingPluginSlotOrdering,
            "plugin slots must declare deterministic contribution ordering posture",
        );
    }
    candidate
}

fn add_missing_diagnostics_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor.diagnostics().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingPluginSlotDiagnostics,
            "plugin slots must declare how contribution diagnostics are materialized",
        );
    }
    candidate
}

fn add_missing_support_posture_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    if descriptor.support().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingPluginSlotSupportPosture,
            "plugin slots must declare runtime support posture",
        );
    }
    candidate
}

fn add_global_mutation_hook_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &PluginSlotDescriptor,
) -> RegistrationCandidate {
    let family_claims_global_mutation = descriptor
        .allowed_families()
        .iter()
        .any(|family| family.is_global_mutation_hook());
    if family_claims_global_mutation || !descriptor.global_mutation_hooks().is_empty() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::PluginSlotArbitraryGlobalMutationHook,
            "plugin slots cannot become arbitrary global UI mutation hooks",
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
