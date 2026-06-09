use crate::{
    request_context::DiagnosticRichnessProfile, ForgeServerDenial, ForgeServerRequestContextDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerForgeNativeSessionDenial {
    ForgeNativeSurfaceAbsent {
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
    ForgeNativeSurfaceDisabled {
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
    RequestContext(ForgeServerRequestContextDenial),
    Middleware(ForgeServerDenial),
}

impl ForgeServerForgeNativeSessionDenial {
    pub(crate) fn new(
        code: ForgeServerForgeNativeSessionDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        match code {
            ForgeServerForgeNativeSessionDenialCode::ForgeNativeSurfaceAbsent => {
                Self::ForgeNativeSurfaceAbsent {
                    diagnostics_profile,
                    detail: detail.into(),
                }
            }
            ForgeServerForgeNativeSessionDenialCode::ForgeNativeSurfaceDisabled => {
                Self::ForgeNativeSurfaceDisabled {
                    diagnostics_profile,
                    detail: detail.into(),
                }
            }
            ForgeServerForgeNativeSessionDenialCode::RequestContextDenied
            | ForgeServerForgeNativeSessionDenialCode::MiddlewareDenied => {
                panic!("nested forge-native denials must preserve their source artifact")
            }
        }
    }

    pub fn code(&self) -> ForgeServerForgeNativeSessionDenialCode {
        match self {
            Self::ForgeNativeSurfaceAbsent { .. } => {
                ForgeServerForgeNativeSessionDenialCode::ForgeNativeSurfaceAbsent
            }
            Self::ForgeNativeSurfaceDisabled { .. } => {
                ForgeServerForgeNativeSessionDenialCode::ForgeNativeSurfaceDisabled
            }
            Self::RequestContext(_) => {
                ForgeServerForgeNativeSessionDenialCode::RequestContextDenied
            }
            Self::Middleware(_) => ForgeServerForgeNativeSessionDenialCode::MiddlewareDenied,
        }
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        match self {
            Self::ForgeNativeSurfaceAbsent {
                diagnostics_profile,
                ..
            }
            | Self::ForgeNativeSurfaceDisabled {
                diagnostics_profile,
                ..
            } => *diagnostics_profile,
            Self::RequestContext(denial) => denial.diagnostics_profile(),
            Self::Middleware(denial) => denial.diagnostics_profile(),
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::ForgeNativeSurfaceAbsent { detail, .. }
            | Self::ForgeNativeSurfaceDisabled { detail, .. } => detail,
            Self::RequestContext(denial) => denial.detail(),
            Self::Middleware(denial) => denial.detail(),
        }
    }

    pub fn request_context_denial(&self) -> Option<&ForgeServerRequestContextDenial> {
        match self {
            Self::RequestContext(denial) => Some(denial),
            _ => None,
        }
    }

    pub fn middleware_denial(&self) -> Option<&ForgeServerDenial> {
        match self {
            Self::Middleware(denial) => Some(denial),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerForgeNativeSessionDenialCode {
    ForgeNativeSurfaceAbsent,
    ForgeNativeSurfaceDisabled,
    RequestContextDenied,
    MiddlewareDenied,
}

impl From<ForgeServerRequestContextDenial> for ForgeServerForgeNativeSessionDenial {
    fn from(value: ForgeServerRequestContextDenial) -> Self {
        Self::RequestContext(value)
    }
}

impl From<ForgeServerDenial> for ForgeServerForgeNativeSessionDenial {
    fn from(value: ForgeServerDenial) -> Self {
        Self::Middleware(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerForgeNativeDeferred {
    RequestContext(crate::ForgeServerRequestContextDeferred),
    Middleware(crate::ForgeServerMiddlewareDeferred),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerForgeNativeStale {
    RequestContext(crate::ForgeServerRequestContextStale),
    Middleware(crate::ForgeServerMiddlewareStale),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerForgeNativeRebindRequired {
    RequestContext(crate::ForgeServerRequestContextRebindRequired),
    Middleware(crate::ForgeServerMiddlewareRebindRequired),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerForgeNativeFailure {
    RequestContext(crate::ForgeServerRequestContextFailure),
    Middleware(crate::ForgeServerMiddlewareFailure),
}
