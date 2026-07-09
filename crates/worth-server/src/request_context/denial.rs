use worth_foundational::facade::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRequestContextDenial {
    code: WorthServerRequestContextDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
}

impl WorthServerRequestContextDenial {
    pub(crate) fn new(
        code: WorthServerRequestContextDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> WorthServerRequestContextDenialCode {
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
pub enum WorthServerRequestContextDenialCode {
    InvalidAuthenticatedPrincipal,
    InvalidWorkspaceTarget,
    InvalidBranchTarget,
    IncompatibleSurfaceTransportBinding,
    BranchTargetingDisabled,
    PreviewTargetingDisabled,
    DiagnosticsProfileExceedsMaximum,
}

pub(crate) fn incompatible_surface_transport_binding_detail(
    surface_family: crate::WorthServerSurfaceFamily,
    transport_class: crate::request_context::WorthServerTransportClass,
) -> String {
    format!(
        "surface family {:?} cannot resolve transport class {:?}",
        surface_family, transport_class
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRequestContextDeferred {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRequestContextStale {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRequestContextRebindRequired {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerRequestContextFailure {
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
