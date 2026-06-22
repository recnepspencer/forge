use forge_foundational::DiagnosticRichnessProfile;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffDenial {
    code: ForgeServerQueryHandoffDenialCode,
    diagnostics_profile: DiagnosticRichnessProfile,
    detail: String,
    facts: Option<ForgeServerQueryHandoffDenialFacts>,
    pub(crate) abuse_budget_receipt: Option<crate::ForgeServerAbuseBudgetReceipt>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryHandoffDenialFamily {
    Authority,
    Request,
    Support,
    Precondition,
    Runtime,
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
            facts: None,
            abuse_budget_receipt: None,
        }
    }

    pub(crate) fn with_facts(mut self, facts: ForgeServerQueryHandoffDenialFacts) -> Self {
        self.facts = Some(facts);
        self
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

    pub fn facts(&self) -> Option<&ForgeServerQueryHandoffDenialFacts> {
        self.facts.as_ref()
    }

    pub fn family(&self) -> ForgeServerQueryHandoffDenialFamily {
        match self.code {
            ForgeServerQueryHandoffDenialCode::AuthorityDenied
            | ForgeServerQueryHandoffDenialCode::AuthorizationDenied
            | ForgeServerQueryHandoffDenialCode::OperationFamilyNotRegistered
            | ForgeServerQueryHandoffDenialCode::OperationFamilyDisabled
            | ForgeServerQueryHandoffDenialCode::OperationFamilyNotExposedOnSurface => {
                ForgeServerQueryHandoffDenialFamily::Authority
            }
            ForgeServerQueryHandoffDenialCode::CompatibilityBasisRequestInvalid
            | ForgeServerQueryHandoffDenialCode::CompatibilityBasisRequestUnsupported
            | ForgeServerQueryHandoffDenialCode::CompatibilityConditionalRequestInvalid
            | ForgeServerQueryHandoffDenialCode::CompatibilityDownloadRequestInvalid
            | ForgeServerQueryHandoffDenialCode::CompatibilityStreamingRequestInvalid
            | ForgeServerQueryHandoffDenialCode::CompatibilityUploadRequestInvalid
            | ForgeServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid
            | ForgeServerQueryHandoffDenialCode::CompatibilityMutationFamilyUnsupported
            | ForgeServerQueryHandoffDenialCode::CompatibilityMutationFamilyForbidden
            | ForgeServerQueryHandoffDenialCode::UnknownOperationName
            | ForgeServerQueryHandoffDenialCode::DirectDeclarationBindingInvalid
            | ForgeServerQueryHandoffDenialCode::DirectDeclarationSourceNotAdmitted
            | ForgeServerQueryHandoffDenialCode::DirectProjectionBindingInvalid
            | ForgeServerQueryHandoffDenialCode::DirectMutationBindingDenied
            | ForgeServerQueryHandoffDenialCode::DirectMutationAssertionDenied
            | ForgeServerQueryHandoffDenialCode::DirectMutationContinuityDenied
            | ForgeServerQueryHandoffDenialCode::DirectMutationNamingDenied
            | ForgeServerQueryHandoffDenialCode::DirectMutationTargetReferenceDenied => {
                ForgeServerQueryHandoffDenialFamily::Request
            }
            ForgeServerQueryHandoffDenialCode::UnsupportedQueryFacadeFamily
            | ForgeServerQueryHandoffDenialCode::DownstreamDeliveryRequiresReadIntent
            | ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeUnsupported
            | ForgeServerQueryHandoffDenialCode::DurableResumeDeferred
            | ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionDenied
            | ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionDeferred
            | ForgeServerQueryHandoffDenialCode::ProjectionFactConsumptionSourceMismatch => {
                ForgeServerQueryHandoffDenialFamily::Support
            }
            ForgeServerQueryHandoffDenialCode::CompatibilityConditionalReadPreconditionFailed
            | ForgeServerQueryHandoffDenialCode::CompatibilityMutationPreconditionFailed
            | ForgeServerQueryHandoffDenialCode::CompatibilityIdempotencyConflict
            | ForgeServerQueryHandoffDenialCode::LeaseDeclarationContextMismatch
            | ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeMissingBasis
            | ForgeServerQueryHandoffDenialCode::RuntimeBackedResumeStaleBasis => {
                ForgeServerQueryHandoffDenialFamily::Precondition
            }
            ForgeServerQueryHandoffDenialCode::PreparedIntentMismatch
            | ForgeServerQueryHandoffDenialCode::CompatibilityConditionalReadNotModified
            | ForgeServerQueryHandoffDenialCode::WorkspaceBindingFailed
            | ForgeServerQueryHandoffDenialCode::RetainedQueryArtifactUnavailable => {
                ForgeServerQueryHandoffDenialFamily::Runtime
            }
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerQueryHandoffDenialFacts {
    rejected_operation_name: Option<String>,
    expected_basis_digest: Option<String>,
    observed_basis_digest: Option<String>,
    expected_validator: Option<String>,
    observed_validator: Option<String>,
    idempotency_key: Option<String>,
    conflicting_request_digest: Option<String>,
    bound_request_digest: Option<String>,
}

impl ForgeServerQueryHandoffDenialFacts {
    pub fn rejected_operation_name(&self) -> Option<&str> {
        self.rejected_operation_name.as_deref()
    }

    pub fn expected_basis_digest(&self) -> Option<&str> {
        self.expected_basis_digest.as_deref()
    }

    pub fn observed_basis_digest(&self) -> Option<&str> {
        self.observed_basis_digest.as_deref()
    }

    pub fn expected_validator(&self) -> Option<&str> {
        self.expected_validator.as_deref()
    }

    pub fn observed_validator(&self) -> Option<&str> {
        self.observed_validator.as_deref()
    }

    pub fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }

    pub fn conflicting_request_digest(&self) -> Option<&str> {
        self.conflicting_request_digest.as_deref()
    }

    pub fn bound_request_digest(&self) -> Option<&str> {
        self.bound_request_digest.as_deref()
    }

    pub(crate) fn with_rejected_operation_name(
        mut self,
        operation_name: impl Into<String>,
    ) -> Self {
        self.rejected_operation_name = Some(operation_name.into());
        self
    }

    pub(crate) fn with_basis_mismatch(
        mut self,
        expected_basis_digest: impl Into<String>,
        observed_basis_digest: impl Into<String>,
    ) -> Self {
        self.expected_basis_digest = Some(expected_basis_digest.into());
        self.observed_basis_digest = Some(observed_basis_digest.into());
        self
    }

    pub(crate) fn with_validator_mismatch(
        mut self,
        expected_validator: impl Into<String>,
        observed_validator: impl Into<String>,
    ) -> Self {
        self.expected_validator = Some(expected_validator.into());
        self.observed_validator = Some(observed_validator.into());
        self
    }

    pub(crate) fn with_idempotency_conflict(
        mut self,
        idempotency_key: impl Into<String>,
        conflicting_request_digest: impl Into<String>,
        bound_request_digest: impl Into<String>,
    ) -> Self {
        self.idempotency_key = Some(idempotency_key.into());
        self.conflicting_request_digest = Some(conflicting_request_digest.into());
        self.bound_request_digest = Some(bound_request_digest.into());
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerQueryHandoffDenialCode {
    AuthorityDenied,
    AuthorizationDenied,
    OperationFamilyNotRegistered,
    OperationFamilyDisabled,
    OperationFamilyNotExposedOnSurface,
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
    UnknownOperationName,
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
