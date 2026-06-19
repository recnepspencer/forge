use forge_foundational::facade::DiagnosticRichnessProfile;

use crate::{
    ForgeServerDenial, ForgeServerDenialCode, ForgeServerDenialPriority, ForgeServerPipelineStep,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffDenialCode,
    ForgeServerQueryHandoffDenialFamily, ForgeServerRequestContextDenial,
    ForgeServerRequestContextDenialCode,
};

use super::{receipt::ForgeServerResponseReceipt, ForgeServerResponseTransform};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerDenialBoundary {
    RequestContext,
    Middleware,
    QueryHandoff,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerDenialCause {
    RequestContext {
        code: ForgeServerRequestContextDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
    Middleware {
        code: ForgeServerDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        priority: ForgeServerDenialPriority,
        step: ForgeServerPipelineStep,
        detail: String,
    },
    QueryHandoff {
        code: ForgeServerQueryHandoffDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: String,
    },
}

impl ForgeServerDenialCause {
    pub(crate) fn from_request_context(denial: ForgeServerRequestContextDenial) -> Self {
        Self::RequestContext {
            code: denial.code(),
            diagnostics_profile: denial.diagnostics_profile(),
            detail: denial.detail().to_string(),
        }
    }

    pub(crate) fn from_middleware(denial: ForgeServerDenial) -> Self {
        Self::Middleware {
            code: denial.code(),
            diagnostics_profile: denial.diagnostics_profile(),
            priority: denial.priority(),
            step: denial.step(),
            detail: denial.detail().to_string(),
        }
    }

    pub(crate) fn from_query_handoff(denial: ForgeServerQueryHandoffDenial) -> Self {
        Self::QueryHandoff {
            code: denial.code(),
            diagnostics_profile: denial.diagnostics_profile(),
            detail: denial.detail().to_string(),
        }
    }

    pub fn boundary(&self) -> ForgeServerDenialBoundary {
        match self {
            Self::RequestContext { .. } => ForgeServerDenialBoundary::RequestContext,
            Self::Middleware { .. } => ForgeServerDenialBoundary::Middleware,
            Self::QueryHandoff { .. } => ForgeServerDenialBoundary::QueryHandoff,
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
pub struct ForgeServerDenialEnvelope {
    transform: ForgeServerResponseTransform,
    diagnostics_profile: DiagnosticRichnessProfile,
    cause: ForgeServerDenialCause,
    provenance: forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
    receipt: ForgeServerResponseReceipt,
    canonical_digest: String,
}

impl ForgeServerDenialEnvelope {
    pub(crate) fn new(
        transform: ForgeServerResponseTransform,
        diagnostics_profile: DiagnosticRichnessProfile,
        cause: ForgeServerDenialCause,
        provenance: forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact,
        receipt: ForgeServerResponseReceipt,
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

    pub fn transform(&self) -> ForgeServerResponseTransform {
        self.transform
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn cause(&self) -> &ForgeServerDenialCause {
        &self.cause
    }

    pub fn provenance(
        &self,
    ) -> &forge_foundational::facade::FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub fn receipt(&self) -> &ForgeServerResponseReceipt {
        &self.receipt
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn request_context_code(&self) -> Option<ForgeServerRequestContextDenialCode> {
        match self.cause {
            ForgeServerDenialCause::RequestContext { code, .. } => Some(code),
            _ => None,
        }
    }

    pub fn middleware_code(&self) -> Option<ForgeServerDenialCode> {
        match self.cause {
            ForgeServerDenialCause::Middleware { code, .. } => Some(code),
            _ => None,
        }
    }

    pub fn middleware_priority(&self) -> Option<ForgeServerDenialPriority> {
        match self.cause {
            ForgeServerDenialCause::Middleware { priority, .. } => Some(priority),
            _ => None,
        }
    }

    pub fn middleware_step(&self) -> Option<ForgeServerPipelineStep> {
        match self.cause {
            ForgeServerDenialCause::Middleware { step, .. } => Some(step),
            _ => None,
        }
    }

    pub fn query_handoff_code(&self) -> Option<ForgeServerQueryHandoffDenialCode> {
        match self.cause {
            ForgeServerDenialCause::QueryHandoff { code, .. } => Some(code),
            _ => None,
        }
    }

    pub fn query_handoff_family(&self) -> Option<ForgeServerQueryHandoffDenialFamily> {
        match self.cause {
            ForgeServerDenialCause::QueryHandoff { code, .. } => Some(
                ForgeServerQueryHandoffDenial::new(
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
