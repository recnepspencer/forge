use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, IconDescriptor, RegistrationCandidate,
    RegistrationCandidateDiagnostic, RegistrationDependency, ICON_FAMILY_NAME,
    THEME_TOKEN_FAMILY_NAME,
};

impl IconDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            ICON_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_icon_descriptor_diagnostics(candidate, self)
    }
}

fn add_icon_descriptor_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    candidate = add_unknown_family_diagnostic(candidate, descriptor);
    candidate = add_missing_source_diagnostic(candidate, descriptor);
    candidate = add_missing_source_metadata_diagnostic(candidate, descriptor);
    candidate = add_unsupported_source_kind_diagnostic(candidate, descriptor);
    candidate = add_missing_source_posture_diagnostics(candidate, descriptor);
    candidate = add_missing_theme_token_reference_diagnostic(candidate, descriptor);
    candidate = add_unexpected_theme_token_reference_diagnostic(candidate, descriptor);
    candidate = add_theme_incompatibility_diagnostic(candidate, descriptor);
    candidate = add_raw_asset_reference_diagnostic(candidate, descriptor);
    candidate = add_missing_public_posture_diagnostics(candidate, descriptor);
    add_theme_token_dependency(candidate, descriptor)
}

fn add_unknown_family_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if !descriptor.family().is_known() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnknownIconFamily,
            "icon family must be a built-in domain-agnostic icon family",
        );
    }
    candidate
}

fn add_missing_source_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor.source().is_none() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingIconSource,
            "icon descriptor must declare source/provider metadata",
        );
    }
    candidate
}

fn add_unsupported_source_kind_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .source()
        .is_some_and(|source| !source.kind().is_supported())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnsupportedIconSourceKind,
            "icon source kind is not admitted by the platform vocabulary",
        );
    }
    candidate
}

fn add_missing_source_metadata_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .source()
        .is_some_and(|source| source.has_missing_source_metadata())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingIconSourceMetadata,
            "icon source provider and source key must be explicit non-empty metadata",
        );
    }
    candidate
}

fn add_missing_source_posture_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .source()
        .is_some_and(|source| source.size_support().is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingIconSizeSupport,
            "icon source must declare size support",
        );
    }
    if descriptor
        .source()
        .is_some_and(|source| source.color_support().is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingIconColorSupport,
            "icon source must declare fill/stroke color support",
        );
    }
    candidate
}

fn add_theme_incompatibility_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor.source().is_some_and(|source| {
        !source.color_support().is_missing()
            && !descriptor.theme_posture().is_missing()
            && !source
                .color_support()
                .admits_theme_posture(descriptor.theme_posture())
    }) {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::ThemeIncompatibleIconDescriptor,
            "icon source color support must satisfy declared theme posture",
        );
    }
    candidate
}

fn add_unexpected_theme_token_reference_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .source()
        .is_some_and(|source| source.has_unexpected_theme_token_reference())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnexpectedIconThemeTokenReference,
            "icon theme token references must match theme-token-driven color support",
        );
    }
    candidate
}

fn add_missing_theme_token_reference_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .source()
        .is_some_and(|source| source.requires_missing_theme_token_reference())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingIconThemeTokenReference,
            "theme-token-driven icon sources must reference a registered theme token",
        );
    }
    candidate
}

fn add_theme_token_dependency(
    candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    match descriptor
        .source()
        .and_then(|source| source.declared_theme_token_dependency())
    {
        Some(theme_token) => candidate.with_dependency(RegistrationDependency::new(
            THEME_TOKEN_FAMILY_NAME,
            THEME_TOKEN_FAMILY_NAME,
            theme_token.as_str(),
        )),
        None => candidate,
    }
}

fn add_raw_asset_reference_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor.has_raw_asset_reference() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::RawIconAssetPathOutsideIconSource,
            "raw icon asset paths cannot stand in for stable icon capability IDs",
        );
    }
    candidate
}

fn add_missing_public_posture_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &IconDescriptor,
) -> RegistrationCandidate {
    if descriptor.theme_posture().is_missing() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingIconThemePosture,
            "icon descriptor must declare theme posture",
        );
    }
    if descriptor.accessibility_posture().is_missing() {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingIconAccessibilityPosture,
            "icon descriptor must declare accessibility posture",
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
