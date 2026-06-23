#[derive(Clone, Debug)]
pub struct ForgeServerProductOperationInput {
    operation_name: String,
    payload: super::ForgeServerProductOperationPayload,
    snapshot_precondition: Option<crate::ForgeServerProductSnapshotPrecondition>,
    idempotency_key: Option<crate::ForgeServerProductIdempotencyKey>,
    product_session_identity: Option<String>,
}

impl ForgeServerProductOperationInput {
    pub fn new(
        operation_name: impl Into<String>,
        payload: super::ForgeServerProductOperationPayload,
    ) -> Self {
        Self {
            operation_name: operation_name.into(),
            payload,
            snapshot_precondition: None,
            idempotency_key: None,
            product_session_identity: None,
        }
    }

    pub fn with_basis_digest(mut self, basis_digest: impl Into<String>) -> Self {
        self.snapshot_precondition = Some(
            crate::ForgeServerProductSnapshotPrecondition::at_base_digest(
                crate::ForgeServerProductOperationBaseDigest::new(basis_digest.into())
                    .expect("basis digest shim should remain canonical"),
            ),
        );
        self
    }

    pub fn with_snapshot_precondition(
        mut self,
        snapshot_precondition: crate::ForgeServerProductSnapshotPrecondition,
    ) -> Self {
        self.snapshot_precondition = Some(snapshot_precondition);
        self
    }

    pub fn with_idempotency_key(
        mut self,
        idempotency_key: crate::ForgeServerProductIdempotencyKey,
    ) -> Self {
        self.idempotency_key = Some(idempotency_key);
        self
    }

    pub fn with_product_session_identity(
        mut self,
        product_session_identity: impl Into<String>,
    ) -> Self {
        self.product_session_identity = Some(product_session_identity.into());
        self
    }

    pub fn operation_name(&self) -> &str {
        &self.operation_name
    }

    pub(crate) fn payload(&self) -> &super::ForgeServerProductOperationPayload {
        &self.payload
    }

    pub fn snapshot_precondition(&self) -> Option<&crate::ForgeServerProductSnapshotPrecondition> {
        self.snapshot_precondition.as_ref()
    }

    pub(crate) fn basis_digest(&self) -> Option<&str> {
        self.snapshot_precondition
            .as_ref()
            .map(|precondition| precondition.base_digest().value())
    }

    pub(crate) fn idempotency_key(&self) -> Option<&crate::ForgeServerProductIdempotencyKey> {
        self.idempotency_key.as_ref()
    }

    pub(crate) fn product_session_identity(&self) -> Option<&str> {
        self.product_session_identity.as_deref()
    }

    pub(crate) fn into_payload(self) -> super::ForgeServerProductOperationPayload {
        self.payload
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeServerProductOperationExecutionBoundary {
    RejectedBeforeAdapterExecution,
    AdapterExecutionAttempted,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerProductOperationSurfaceDenialFacts {
    readiness_code: Option<crate::ForgeServerOperationReadinessDenialCode>,
    stale_basis_denial: Option<crate::ForgeServerProductStaleBasisDenial>,
    rebase_required: Option<crate::ForgeServerProductRebaseRequired>,
    session_denial_code: Option<crate::ForgeServerProductSessionDenialCode>,
    idempotency_conflict: Option<crate::ForgeServerProductIdempotencyConflict>,
    execution_boundary: Option<ForgeServerProductOperationExecutionBoundary>,
}

impl ForgeServerProductOperationSurfaceDenialFacts {
    pub fn readiness_code(&self) -> Option<crate::ForgeServerOperationReadinessDenialCode> {
        self.readiness_code.clone()
    }

    pub fn expected_basis_digest(&self) -> Option<&str> {
        self.stale_basis_denial
            .as_ref()
            .map(crate::ForgeServerProductStaleBasisDenial::expected_base_digest)
    }

    pub fn observed_basis_digest(&self) -> Option<&str> {
        self.stale_basis_denial
            .as_ref()
            .map(crate::ForgeServerProductStaleBasisDenial::observed_base_digest)
    }

    pub fn session_denial_code(&self) -> Option<crate::ForgeServerProductSessionDenialCode> {
        self.session_denial_code
    }

    pub fn stale_basis_denial(&self) -> Option<&crate::ForgeServerProductStaleBasisDenial> {
        self.stale_basis_denial.as_ref()
    }

    pub fn rebase_required(&self) -> Option<&crate::ForgeServerProductRebaseRequired> {
        self.rebase_required.as_ref()
    }

    pub fn idempotency_conflict(&self) -> Option<&crate::ForgeServerProductIdempotencyConflict> {
        self.idempotency_conflict.as_ref()
    }

    pub fn execution_boundary(&self) -> Option<&ForgeServerProductOperationExecutionBoundary> {
        self.execution_boundary.as_ref()
    }

    pub(crate) fn with_readiness_code(
        mut self,
        readiness_code: crate::ForgeServerOperationReadinessDenialCode,
    ) -> Self {
        self.readiness_code = Some(readiness_code);
        self
    }

    pub(crate) fn with_basis_mismatch(
        mut self,
        stale_basis_denial: crate::ForgeServerProductStaleBasisDenial,
    ) -> Self {
        self.rebase_required = Some(crate::ForgeServerProductRebaseRequired::new(
            stale_basis_denial.clone(),
        ));
        self.stale_basis_denial = Some(stale_basis_denial);
        self
    }

    pub(crate) fn with_session_denial_code(
        mut self,
        session_denial_code: crate::ForgeServerProductSessionDenialCode,
    ) -> Self {
        self.session_denial_code = Some(session_denial_code);
        self
    }

    pub(crate) fn with_execution_boundary(
        mut self,
        execution_boundary: ForgeServerProductOperationExecutionBoundary,
    ) -> Self {
        self.execution_boundary = Some(execution_boundary);
        self
    }

    pub(crate) fn with_idempotency_conflict(
        mut self,
        idempotency_conflict: crate::ForgeServerProductIdempotencyConflict,
    ) -> Self {
        self.idempotency_conflict = Some(idempotency_conflict);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationSurfaceDenial {
    code: ForgeServerProductOperationSurfaceDenialCode,
    detail: String,
    facts: Option<ForgeServerProductOperationSurfaceDenialFacts>,
}

impl ForgeServerProductOperationSurfaceDenial {
    pub(crate) fn new(code: ForgeServerProductOperationSurfaceDenialCode, detail: String) -> Self {
        Self {
            code,
            detail,
            facts: None,
        }
    }

    pub(crate) fn from_request_denial(denial: crate::ForgeServerOperationRequestDenial) -> Self {
        Self::new(
            ForgeServerProductOperationSurfaceDenialCode::RequestDenied,
            denial.detail().to_string(),
        )
        .with_facts(
            ForgeServerProductOperationSurfaceDenialFacts::default().with_execution_boundary(
                ForgeServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution,
            ),
        )
    }

    pub(crate) fn from_admission_denial(
        denial: crate::ForgeServerOperationAdmissionDenial,
    ) -> Self {
        Self::new(
            ForgeServerProductOperationSurfaceDenialCode::AdmissionDenied,
            denial.detail().to_string(),
        )
        .with_facts(
            ForgeServerProductOperationSurfaceDenialFacts::default().with_execution_boundary(
                ForgeServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution,
            ),
        )
    }

    pub(crate) fn from_readiness_denial(
        denial: crate::ForgeServerOperationReadinessDenial,
    ) -> Self {
        let facts = denial.facts();
        let mut surface_facts = ForgeServerProductOperationSurfaceDenialFacts::default()
            .with_readiness_code(denial.code())
            .with_execution_boundary(
                ForgeServerProductOperationExecutionBoundary::RejectedBeforeAdapterExecution,
            );
        if let Some(facts) = facts {
            if let (Some(expected_basis_digest), Some(observed_basis_digest)) =
                (facts.expected_basis_digest(), facts.observed_basis_digest())
            {
                surface_facts = surface_facts.with_basis_mismatch(
                    crate::ForgeServerProductStaleBasisDenial::new(
                        expected_basis_digest,
                        observed_basis_digest,
                    ),
                );
            }
        }
        Self::new(
            ForgeServerProductOperationSurfaceDenialCode::ReadinessDenied,
            denial.detail().to_string(),
        )
        .with_facts(surface_facts)
    }

    pub(crate) fn with_facts(
        mut self,
        facts: ForgeServerProductOperationSurfaceDenialFacts,
    ) -> Self {
        self.facts = Some(facts);
        self
    }

    pub fn code(&self) -> ForgeServerProductOperationSurfaceDenialCode {
        self.code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn facts(&self) -> Option<&ForgeServerProductOperationSurfaceDenialFacts> {
        self.facts.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerProductOperationSurfaceDenialCode {
    UnknownOperationName,
    RequestDenied,
    AdmissionDenied,
    ReadinessDenied,
    PreconditionDenied,
    IdempotencyConflict,
    InvalidDeclaration,
}
