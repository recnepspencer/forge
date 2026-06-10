use forge_foundational::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffDenial {
    code: ForgeServerQueryHandoffDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
    pub(crate) abuse_budget_receipt: Option<crate::ForgeServerAbuseBudgetReceipt>,
}

impl ForgeServerQueryHandoffDenial {
    pub(crate) fn new(
        code: ForgeServerQueryHandoffDenialCode,
        diagnostics_profile: DiagnosticRichnessProfile,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            diagnostics_profile,
            detail: detail.into(),
            abuse_budget_receipt: None,
        }
    }

    pub(crate) fn with_abuse_budget_receipt(
        mut self,
        abuse_budget_receipt: crate::ForgeServerAbuseBudgetReceipt,
    ) -> Self {
        self.abuse_budget_receipt = Some(abuse_budget_receipt);
        self
    }

    pub fn code(&self) -> ForgeServerQueryHandoffDenialCode {
        self.code
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn abuse_budget_receipt(&self) -> Option<&crate::ForgeServerAbuseBudgetReceipt> {
        self.abuse_budget_receipt.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryHandoffDenialCode {
    PreparedIntentMismatch,
    UnsupportedQueryFacadeFamily,
    DownstreamDeliveryRequiresReadIntent,
    DirectDeclarationBindingInvalid,
    DirectDeclarationSourceNotAdmitted,
    CompatibilityBasisRequestInvalid,
    CompatibilityBasisRequestUnsupported,
    CompatibilityConditionalRequestInvalid,
    CompatibilityConditionalReadNotModified,
    CompatibilityConditionalReadPreconditionFailed,
    CompatibilityDownloadRequestInvalid,
    CompatibilityStreamingRequestInvalid,
    CompatibilityUploadRequestInvalid,
    CompatibilityMutationRequestInvalid,
    CompatibilityMutationFamilyUnsupported,
    CompatibilityMutationFamilyForbidden,
    CompatibilityMutationPreconditionFailed,
    CompatibilityIdempotencyConflict,
    LeaseDeclarationContextMismatch,
    RuntimeBackedResumeMissingBasis,
    RuntimeBackedResumeStaleBasis,
    RuntimeBackedResumeUnsupported,
    DurableResumeDeferred,
    WorkspaceBindingFailed,
    RetainedQueryArtifactUnavailable,
    DirectProjectionBindingInvalid,
    DirectMutationBindingDenied,
    DirectMutationAssertionDenied,
    DirectMutationContinuityDenied,
    DirectMutationNamingDenied,
    DirectMutationTargetReferenceDenied,
    ProjectionFactConsumptionDenied,
    ProjectionFactConsumptionDeferred,
    ProjectionFactConsumptionSourceMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffDeferred {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffStale {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffRebindRequired {
    reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffFailure {
    reason: &'static str,
}

impl ForgeServerQueryHandoffFailure {
    pub(crate) fn new(reason: &'static str) -> Self {
        Self { reason }
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}
