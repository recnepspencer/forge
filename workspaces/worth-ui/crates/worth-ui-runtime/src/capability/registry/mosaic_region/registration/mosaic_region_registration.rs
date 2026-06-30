use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, MOSAIC_REGION_KIND_FAMILY_NAME,
};

use super::super::MosaicRegionKindDescriptor;

impl MosaicRegionKindDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            MOSAIC_REGION_KIND_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_mosaic_region_descriptor_diagnostics(candidate, self)
    }
}

fn add_mosaic_region_descriptor_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    candidate = add_product_domain_role_diagnostic(candidate, descriptor);
    candidate = add_missing_sizing_behavior_diagnostic(candidate, descriptor);
    candidate = add_missing_scroll_ownership_diagnostic(candidate, descriptor);
    candidate = add_missing_focus_scope_diagnostic(candidate, descriptor);
    candidate = add_missing_child_rule_diagnostic(candidate, descriptor);
    candidate = add_missing_allowed_surface_class_diagnostic(candidate, descriptor);
    candidate = add_missing_persistence_diagnostic(candidate, descriptor);
    candidate = add_missing_clipping_diagnostic(candidate, descriptor);
    candidate = add_missing_hit_test_diagnostic(candidate, descriptor);
    add_unsupported_surface_class_diagnostic(candidate, descriptor)
}

fn add_product_domain_role_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor.role().is_product_domain_name() {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::ProductDomainMosaicRegionRole,
            "mosaic region role must stay structural instead of naming product domains",
        ));
    }
    candidate
}

fn add_missing_sizing_behavior_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .sizing_behavior()
        .is_none_or(|sizing_behavior| sizing_behavior.is_missing())
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::MissingMosaicRegionSizingBehavior,
            "mosaic region kind must declare sizing behavior",
        ));
    }
    candidate
}

fn add_missing_scroll_ownership_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .scroll_ownership()
        .is_none_or(|scroll_ownership| scroll_ownership.is_missing())
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::MissingMosaicRegionScrollOwnership,
            "mosaic region kind must declare scroll ownership",
        ));
    }
    candidate
}

fn add_missing_focus_scope_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .focus_scope()
        .is_none_or(|focus_scope| focus_scope.is_missing())
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::MissingMosaicRegionFocusScope,
            "mosaic region kind must declare focus scope",
        ));
    }
    candidate
}

fn add_missing_child_rule_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .child_rule()
        .is_none_or(|child_rule| child_rule.is_missing())
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::MissingMosaicRegionChildRule,
            "mosaic region kind must declare child rule",
        ));
    }
    candidate
}

fn add_missing_allowed_surface_class_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .child_rule()
        .is_some_and(|child_rule| child_rule.requires_allowed_surface_class())
        && descriptor.allowed_surface_classes().is_empty()
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::MissingMosaicRegionAllowedSurfaceClass,
            "mosaic region kind accepting surfaces must declare at least one surface class",
        ));
    }
    candidate
}

fn add_missing_persistence_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .persistence()
        .is_none_or(|persistence| persistence.is_missing())
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::MissingMosaicRegionPersistence,
            "mosaic region kind must declare persistence posture",
        ));
    }
    candidate
}

fn add_missing_clipping_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .clipping()
        .is_none_or(|clipping| clipping.is_missing())
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::MissingMosaicRegionClipping,
            "mosaic region kind must declare clipping posture",
        ));
    }
    candidate
}

fn add_missing_hit_test_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .hit_test()
        .is_none_or(|hit_test| hit_test.is_missing())
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::MissingMosaicRegionHitTest,
            "mosaic region kind must declare hit-test posture",
        ));
    }
    candidate
}

fn add_unsupported_surface_class_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicRegionKindDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .allowed_surface_classes()
        .iter()
        .any(|surface_class| surface_class.is_unsupported())
    {
        candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
            CapabilityDiagnosticCode::UnsupportedMosaicRegionSurfaceClass,
            "mosaic region kind cannot admit unsupported surface classes",
        ));
    }

    candidate
}
