use worth_foundational::facade::DiagnosticRichnessProfile;

use crate::{
    WorthServerDenial, WorthServerDenialCode, WorthServerDenialPriority, WorthServerPipelineStep,
    WorthServerQueryHandoffDenial, WorthServerQueryHandoffDenialCode,
    WorthServerQueryHandoffDenialFamily, WorthServerRequestContextDenial,
    WorthServerRequestContextDenialCode,
};

use super::{receipt::WorthServerResponseReceipt, WorthServerResponseTransform};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerDenialBoundary {
    RequestContext,
    Middleware,
    QueryHandoff,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerDenialCause {
    RequestContext {
        code: WorthServerRequestContextDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
    Middleware {
        code: WorthServerDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        priority: WorthServerDenialPriority,
        step: WorthServerPipelineStep,
        detail: String,
    },
    QueryHandoff {
        code: WorthServerQueryHandoffDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
}

impl WorthServerDenialCause {
    pub(crate) fn from_request_context(denial: WorthServerRequestContextDenial) -> Self {
        Self::RequestContext {
            code: denial.code(),
            diagnostics_profile: denial.diagnostics_profile(),
            detail: denial.detail().to_string(),
        }
    }

    pub(crate) fn from_middleware(denial: WorthServerDenial) -> Self {
        Self::Middleware {
            code: denial.code(),
            diagnostics_profile: denial.diagnostics_profile(),
            priority: denial.priority(),
            step: denial.step(),
            detail: denial.detail().to_string(),
        }
    }

    pub(crate) fn from_query_handoff(denial: WorthServerQueryHandoffDenial) -> Self {
        Self::QueryHandoff {
            code: denial.code(),
            diagnostics_profile: denial.diagnostics_profile(),
            detail: denial.detail().to_string(),
        }
    }

    pub fn boundary(&self) -> WorthServerDenialBoundary {
        match self {
            Self::RequestContext { .. } => WorthServerDenialBoundary::RequestContext,
            Self::Middleware { .. } => WorthServerDenialBoundary::Middleware,
            Self::QueryHandoff { .. } => WorthServerDenialBoundary::QueryHandoff,
        }
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        match self {
            Self::RequestContext {
                diagnostics_profile,
                ..
            }
            | Self::Middleware {
                diagnostics_profile,
                ..
            }
            | Self::QueryHandoff {
                diagnostics_profile,
                ..
            } => *diagnostics_profile,
        }
    }

    pub fn detail(&self) -> &str {
        match self {
            Self::RequestContext { detail, .. }
            | Self::Middleware { detail, .. }
            | Self::QueryHandoff { detail, .. } => detail,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthServerDenialEnvelope {
    transform: WorthServerResponseTransform,
    diagnostics_profile: DiagnosticRichnessProfile,
    cause: WorthServerDenialCause,
    provenance: worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    receipt: WorthServerResponseReceipt,
    canonical_digest: String,
}

impl WorthServerDenialEnvelope {
    pub(crate) fn new(
        transform: WorthServerResponseTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        cause: WorthServerDenialCause,
        provenance: worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
        receipt: WorthServerResponseReceipt,
        canonical_digest: String,
    ) -> Self {
        Self {
            transform,
            diagnostics_profile,
            cause,
            provenance,
            receipt,
            canonical_digest,
        }
    }

    pub fn transform(&self) -> WorthServerResponseTransform {
        self.transform
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn cause(&self) -> &WorthServerDenialCause {
        &self.cause
    }

    pub fn provenance(
        &self,
    ) -> &worth_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn receipt(&self) -> &WorthServerResponseReceipt {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn request_context_code(&self) -> Option<WorthServerRequestContextDenialCode> {
        match self.cause {
            WorthServerDenialCause::RequestContext { code, .. } => Some(code),
            _ => None,
        }
    }

    pub fn middleware_code(&self) -> Option<WorthServerDenialCode> {
        match self.cause {
            WorthServerDenialCause::Middleware { code, .. } => Some(code),
            _ => None,
        }
    }

    pub fn middleware_priority(&self) -> Option<WorthServerDenialPriority> {
        match self.cause {
            WorthServerDenialCause::Middleware { priority, .. } => Some(priority),
            _ => None,
        }
    }

    pub fn middleware_step(&self) -> Option<WorthServerPipelineStep> {
        match self.cause {
            WorthServerDenialCause::Middleware { step, .. } => Some(step),
            _ => None,
        }
    }

    pub fn query_handoff_code(&self) -> Option<WorthServerQueryHandoffDenialCode> {
        match self.cause {
            WorthServerDenialCause::QueryHandoff { code, .. } => Some(code),
            _ => None,
        }
    }

    pub fn query_handoff_family(&self) -> Option<WorthServerQueryHandoffDenialFamily> {
        match self.cause {
            WorthServerDenialCause::QueryHandoff { code, .. } => Some(
                WorthServerQueryHandoffDenial::new(
                    code,
                    self.diagnostics_profile,
                    self.cause.detail(),
                )
                .family(),
            ),
            _ => None,
        }
    }
}
