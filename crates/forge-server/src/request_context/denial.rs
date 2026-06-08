use forge_foundational::facade::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextDenial {
    code: ForgeServerRequestContextDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
}

impl ForgeServerRequestContextDenial {
    pub(crate) fn new(
        code: ForgeServerRequestContextDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> ForgeServerRequestContextDenialCode {
        self.code
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerRequestContextDenialCode {
    InvalidAuthenticatedPrincipal,
    InvalidWorkspaceTarget,
    InvalidBranchTarget,
    IncompatibleSurfaceTransportBinding,
    BranchTargetingDisabled,
    PreviewTargetingDisabled,
    DiagnosticsProfileExceedsMaximum,
}

pub(crate) fn incompatible_surface_transport_binding_detail(
    surface_family: crate::ForgeServerSurfaceFamily,
    transport_class: crate::request_context::ForgeServerTransportClass,
) -> String {
    format!(
        "surface family {:?} cannot resolve transport class {:?}",
        surface_family, transport_class
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextDeferred {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextStale {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextRebindRequired {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerRequestContextFailure {
    reason: &'static str,
}

pub(crate) fn diagnostics_profile_exceeds_maximum_detail(
    requested: DiagnosticRichnessProfile,
    maximum: DiagnosticRichnessProfile,
) -> String {
    format!(
        "requested diagnostics profile {:?} exceeds configured maximum {:?}",
        requested, maximum
    )
}
