use crate::{
    request_context::DiagnosticRichnessProfile, ForgeServerOperationReadinessDenial,
    ForgeServerOperationReadinessDenialCode, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationPlanDenialCode {
    PreparedIntentMismatch,
    WorkspaceBindingFailed,
    SupportDenied,
    PreconditionDenied,
    DownstreamDeliveryRequiresReadIntent,
    RuntimeBackedResumeUnsupported,
    DurableResumeDeferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationPlanDenial {
    code: ForgeServerOperationPlanDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
}

impl ForgeServerOperationPlanDenial {
    pub(crate) fn new(
        code: ForgeServerOperationPlanDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
        }
    }

    pub(crate) fn from_readiness_denial(
        denial: ForgeServerOperationReadinessDenial,
        diagnostics_profile: DiagnosticRichnessProfile,
    ) -> Self {
        let code = match denial.code() {
            ForgeServerOperationReadinessDenialCode::InvalidPreconditionInput
            | ForgeServerOperationReadinessDenialCode::PreconditionFailed => {
                ForgeServerOperationPlanDenialCode::PreconditionDenied
            }
            ForgeServerOperationReadinessDenialCode::DownstreamDeliveryRequiresReadIntent => {
                ForgeServerOperationPlanDenialCode::DownstreamDeliveryRequiresReadIntent
            }
            ForgeServerOperationReadinessDenialCode::RuntimeBackedResumeUnsupported => {
                ForgeServerOperationPlanDenialCode::RuntimeBackedResumeUnsupported
            }
            ForgeServerOperationReadinessDenialCode::DurableResumeDeferred => {
                ForgeServerOperationPlanDenialCode::DurableResumeDeferred
            }
            ForgeServerOperationReadinessDenialCode::MissingQuerySupport
            | ForgeServerOperationReadinessDenialCode::UnsupportedQuerySupport
            | ForgeServerOperationReadinessDenialCode::UnsupportedProductSupport
            | ForgeServerOperationReadinessDenialCode::UnknownProductSupport
            | ForgeServerOperationReadinessDenialCode::FixtureOnlyProductSupport
            | ForgeServerOperationReadinessDenialCode::IncompatibleSupportBasis => {
                ForgeServerOperationPlanDenialCode::SupportDenied
            }
        };
        Self::new(code, diagnostics_profile, denial.detail())
    }

    pub(crate) fn into_query_handoff_denial(self) -> ForgeServerQueryHandoffDenial {
        let code = match self.code {
            ForgeServerOperationPlanDenialCode::PreparedIntentMismatch => {
                ForgeServerQueryHandoffDenialCode::PreparedIntentMismatch
            }
            ForgeServerOperationPlanDenialCode::WorkspaceBindingFailed => {
                ForgeServerQueryHandoffDenialCode::WorkspaceBindingFailed
            }
            ForgeServerOperationPlanDenialCode::SupportDenied => {
                ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
            }
            ForgeServerOperationPlanDenialCode::PreconditionDenied => {
                ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed
            }
            ForgeServerOperationPlanDenialCode::DownstreamDeliveryRequiresReadIntent => {
                ForgeServerQueryHandoffDenialCode::DownstreamDeliveryRequiresReadIntent
            }
            ForgeServerOperationPlanDenialCode::RuntimeBackedResumeUnsupported => {
                ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeUnsupported
            }
            ForgeServerOperationPlanDenialCode::DurableResumeDeferred => {
                ForgeServerQueryHandoffDenialCode::DurableResumeDeferred
            }
        };
        ForgeServerQueryHandoffDenial::new(code, self.diagnostics_profile, self.detail)
    }

    pub fn code(&self) -> ForgeServerOperationPlanDenialCode {
        self.code
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
