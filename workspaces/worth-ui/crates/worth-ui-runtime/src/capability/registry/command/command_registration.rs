use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, CommandDescriptor, RegistrationCandidate,
    RegistrationCandidateDiagnostic, RegistrationDependency, COMMAND_FAMILY_NAME,
    COMMAND_PROJECTION_FAMILY_NAME, ICON_FAMILY_NAME, INTENT_DEFINITION_FAMILY_NAME,
};

impl CommandDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            COMMAND_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );

        let mut candidate = match self.projection_eligibility() {
            Some(projection_id) => candidate.with_dependency(RegistrationDependency::new(
                COMMAND_PROJECTION_FAMILY_NAME,
                COMMAND_PROJECTION_FAMILY_NAME,
                projection_id.as_str(),
            )),
            None => candidate,
        };

        candidate = match self.icon() {
            Some(icon) => candidate.with_dependency(RegistrationDependency::new(
                ICON_FAMILY_NAME,
                ICON_FAMILY_NAME,
                icon.as_str(),
            )),
            None => candidate,
        };

        candidate = match self.route() {
            Some(route) => candidate.with_dependency(RegistrationDependency::new(
                INTENT_DEFINITION_FAMILY_NAME,
                INTENT_DEFINITION_FAMILY_NAME,
                route.destination().intent().as_str(),
            )),
            None => candidate,
        };

        if self.default_shortcut().is_some() && self.route().is_none() {
            candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
                CapabilityDiagnosticCode::MissingCommandRouteDestination,
                "a command default shortcut requires an explicit typed intent route destination",
            ));
        }

        if self
            .default_shortcut()
            .is_some_and(|shortcut| shortcut.has_conflicting_primary_alias())
        {
            candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
                CapabilityDiagnosticCode::ConflictingCommandShortcutAlias,
                "Primary cannot be combined with Control or Meta because the alias resolves to one of those modifiers",
            ));
        }

        if self.route().is_some_and(|route| {
            route.scope() == crate::capability::UiCommandRouteScope::ActiveRegion
        }) {
            candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
                CapabilityDiagnosticCode::UnsupportedCommandRouteScope,
                "ActiveRegion requires a runtime region-activation snapshot and is not admitted until that authority exists",
            ));
        }

        if self.route().is_some_and(|route| {
            matches!(
                route.scope(),
                crate::capability::UiCommandRouteScope::FocusedControl
                    | crate::capability::UiCommandRouteScope::ActivePortal
            ) && route.scope_identity().is_none()
        }) {
            candidate = candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(
                CapabilityDiagnosticCode::MissingCommandRouteScopeIdentity,
                "focused-control and active-portal routes require an exact authored semantic scope identity",
            ));
        }

        candidate
    }
}
