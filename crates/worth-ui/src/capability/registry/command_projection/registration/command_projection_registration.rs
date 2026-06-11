use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, CommandProjectionDescriptor,
    RegistrationCandidate, RegistrationCandidateDiagnostic, RegistrationDependency,
    COMMAND_FAMILY_NAME, COMMAND_PROJECTION_FAMILY_NAME, MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
};

impl CommandProjectionDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            COMMAND_PROJECTION_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_command_projection_diagnostics(candidate, self)
    }
}

fn add_command_projection_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    candidate = add_command_dependencies(candidate, descriptor);
    candidate = add_mosaic_scope_dependency(candidate, descriptor);
    candidate = add_unsupported_surface_diagnostic(candidate, descriptor);
    candidate = add_required_mosaic_scope_diagnostic(candidate, descriptor);
    candidate = add_rejected_mosaic_scope_diagnostic(candidate, descriptor);
    candidate = add_missing_eligibility_diagnostic(candidate, descriptor);
    candidate = add_grouping_diagnostics(candidate, descriptor);
    add_command_meaning_override_diagnostic(candidate, descriptor)
}

fn add_command_dependencies(
    candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    let mut command_ids = descriptor
        .command_references()
        .iter()
        .map(|reference| reference.command_id().as_str())
        .collect::<Vec<_>>();
    command_ids.sort_unstable();
    command_ids.dedup();

    command_ids
        .into_iter()
        .fold(candidate, |candidate, command_id| {
            candidate.with_dependency(RegistrationDependency::new(
                COMMAND_FAMILY_NAME,
                COMMAND_FAMILY_NAME,
                command_id,
            ))
        })
}

fn add_mosaic_scope_dependency(
    candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    match descriptor.mosaic_scope() {
        Some(scope) => candidate.with_dependency(RegistrationDependency::new(
            MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
            MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
            scope.placement_policy_id().as_str(),
        )),
        None => candidate,
    }
}

fn add_unsupported_surface_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    if !descriptor.surface().is_supported() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnsupportedCommandProjectionSurface,
            "command projection surface must be a built-in domain-agnostic surface",
        );
    }
    candidate
}

fn add_required_mosaic_scope_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor.surface().requires_mosaic_scope() && descriptor.mosaic_scope().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingCommandProjectionMosaicScope,
            "mosaic-bound command projection surfaces must reference a placement policy",
        );
    }
    candidate
}

fn add_rejected_mosaic_scope_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor.surface().rejects_mosaic_scope() && descriptor.mosaic_scope().is_some() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnexpectedCommandProjectionMosaicScope,
            "global command projection surfaces cannot claim mosaic placement scope",
        );
    }
    candidate
}

fn add_missing_eligibility_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor.command_references().is_empty() && descriptor.eligible_categories().is_empty() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingCommandProjectionEligibility,
            "command projections must declare command references or eligible categories",
        );
    }
    candidate
}

fn add_grouping_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .groupings()
        .iter()
        .any(|grouping| grouping.is_missing_group_key())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingCommandProjectionGrouping,
            "command projection grouping keys must be non-empty semantic names",
        );
    }

    if descriptor
        .groupings()
        .iter()
        .enumerate()
        .any(|(index, left)| {
            descriptor
                .groupings()
                .iter()
                .skip(index + 1)
                .any(|right| left.conflicts_with(right))
        })
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::ConflictingCommandProjectionGrouping,
            "a command projection cannot require multiple incompatible grouping keys",
        );
    }
    candidate
}

fn add_command_meaning_override_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &CommandProjectionDescriptor,
) -> RegistrationCandidate {
    if !descriptor.meaning_overrides().is_empty() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::CommandProjectionDefinesCommandMeaning,
            "command projections cannot define labels, readiness, handlers, or command meaning",
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
