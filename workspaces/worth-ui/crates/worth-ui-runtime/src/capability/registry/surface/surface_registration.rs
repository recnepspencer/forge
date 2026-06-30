use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, RegistrationDependency, COMMAND_FAMILY_NAME,
    COMPONENT_FAMILY_NAME, ICON_FAMILY_NAME, SURFACE_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};

use super::SurfaceDescriptor;

impl SurfaceDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            SURFACE_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        let candidate = add_surface_descriptor_diagnostics(candidate, self);
        add_surface_dependencies(candidate, self)
    }
}

fn add_surface_descriptor_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &SurfaceDescriptor,
) -> RegistrationCandidate {
    if descriptor.kind().is_product_domain_name() {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::ProductDomainSurfaceKind,
            "surface kind must stay structural instead of naming product domains",
        ));
    }

    if descriptor.placement_class().is_unsupported() {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::UnsupportedSurfacePlacementClass,
            "surface placement class is not admitted by the platform vocabulary",
        ));
    }

    if descriptor.state_class().is_invalid() {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::InvalidSurfaceStateClass,
            "surface state class is not admitted by the platform vocabulary",
        ));
    }

    candidate
}

fn add_surface_dependencies(
    mut candidate: RegistrationCandidate,
    descriptor: &SurfaceDescriptor,
) -> RegistrationCandidate {
    candidate = candidate.with_dependency(RegistrationDependency::new(
        COMPONENT_FAMILY_NAME,
        COMPONENT_FAMILY_NAME,
        descriptor.component_id().as_str(),
    ));

    for command_id in descriptor.command_slots() {
        candidate = candidate.with_dependency(RegistrationDependency::new(
            COMMAND_FAMILY_NAME,
            COMMAND_FAMILY_NAME,
            command_id.as_str(),
        ));
    }

    if let Some(view_binding) = descriptor.view_binding() {
        candidate = candidate.with_dependency(RegistrationDependency::new(
            VIEW_BINDING_FAMILY_NAME,
            VIEW_BINDING_FAMILY_NAME,
            view_binding.as_str(),
        ));
    }

    if let Some(icon) = descriptor.icon() {
        candidate = candidate.with_dependency(RegistrationDependency::new(
            ICON_FAMILY_NAME,
            ICON_FAMILY_NAME,
            icon.as_str(),
        ));
    }

    candidate
}
