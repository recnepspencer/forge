use crate::{
    request_context::DiagnosticRichnessProfile, WorthServerDenial, WorthServerRequestContextDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerWorthNativeSessionDenial {
    WorthNativeSurfaceAbsent {
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
    WorthNativeSurfaceDisabled {
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
    RequestContext(WorthServerRequestContextDenial),
    Middleware(WorthServerDenial),
}

impl WorthServerWorthNativeSessionDenial {
    pub(crate) fn new(
        code: WorthServerWorthNativeSessionDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        match code {
            WorthServerWorthNativeSessionDenialCode::WorthNativeSurfaceAbsent => {
                Self::WorthNativeSurfaceAbsent {
                    diagnostics_profile,
                    detail: detail.into(),
                }
            }
            WorthServerWorthNativeSessionDenialCode::WorthNativeSurfaceDisabled => {
                Self::WorthNativeSurfaceDisabled {
                    diagnostics_profile,
                    detail: detail.into(),
                }
            }
            WorthServerWorthNativeSessionDenialCode::RequestContextDenied
            | WorthServerWorthNativeSessionDenialCode::MiddlewareDenied => {
                panic!("nested Worth-native denials must preserve their source artifact")
            }
        }
    }

    pub fn code(&self) -> WorthServerWorthNativeSessionDenialCode {
        match self {
            Self::WorthNativeSurfaceAbsent { .. } => {
                WorthServerWorthNativeSessionDenialCode::WorthNativeSurfaceAbsent
            }
            Self::WorthNativeSurfaceDisabled { .. } => {
                WorthServerWorthNativeSessionDenialCode::WorthNativeSurfaceDisabled
            }
            Self::RequestContext(_) => {
                WorthServerWorthNativeSessionDenialCode::RequestContextDenied
            }
            Self::Middleware(_) => WorthServerWorthNativeSessionDenialCode::MiddlewareDenied,
        }
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        match self {
            Self::WorthNativeSurfaceAbsent {
                diagnostics_profile,
                ..
            }
            | Self::WorthNativeSurfaceDisabled {
                diagnostics_profile,
                ..
            } => *diagnostics_profile,
            Self::RequestContext(denial) => denial.diagnostics_profile(),
            Self::Middleware(denial) => denial.diagnostics_profile(),
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::WorthNativeSurfaceAbsent { detail, .. }
            | Self::WorthNativeSurfaceDisabled { detail, .. } => detail,
            Self::RequestContext(denial) => denial.detail(),
            Self::Middleware(denial) => denial.detail(),
        }
    }

    pub fn request_context_denial(&self) -> Option<&WorthServerRequestContextDenial> {
        match self {
            Self::RequestContext(denial) => Some(denial),
            _ => None,
        }
    }

    pub fn middleware_denial(&self) -> Option<&WorthServerDenial> {
        match self {
            Self::Middleware(denial) => Some(denial),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerWorthNativeSessionDenialCode {
    WorthNativeSurfaceAbsent,
    WorthNativeSurfaceDisabled,
    RequestContextDenied,
    MiddlewareDenied,
}

impl From<WorthServerRequestContextDenial> for WorthServerWorthNativeSessionDenial {
    fn from(value: WorthServerRequestContextDenial) -> Self {
        Self::RequestContext(value)
    }
}

impl From<WorthServerDenial> for WorthServerWorthNativeSessionDenial {
    fn from(value: WorthServerDenial) -> Self {
        Self::Middleware(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerWorthNativeDeferred {
    RequestContext(crate::WorthServerRequestContextDeferred),
    Middleware(crate::WorthServerMiddlewareDeferred),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerWorthNativeStale {
    RequestContext(crate::WorthServerRequestContextStale),
    Middleware(crate::WorthServerMiddlewareStale),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerWorthNativeRebindRequired {
    RequestContext(crate::WorthServerRequestContextRebindRequired),
    Middleware(crate::WorthServerMiddlewareRebindRequired),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerWorthNativeFailure {
    RequestContext(crate::WorthServerRequestContextFailure),
    Middleware(crate::WorthServerMiddlewareFailure),
}
