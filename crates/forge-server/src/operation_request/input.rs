use crate::ForgeServerOperationFamily;

use super::ForgeServerOperationInputEnvelope;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerOperationRequestInput {
    operation_family: Option<ForgeServerOperationFamily>,
    operation_name: Option<String>,
    basis_digest: Option<String>,
    idempotency_key: Option<String>,
    product_session_identity: Option<String>,
    payload_envelope: Option<ForgeServerOperationInputEnvelope>,
}

impl ForgeServerOperationRequestInput {
    pub fn builder() -> ForgeServerOperationRequestInputBuilder {
        ForgeServerOperationRequestInputBuilder::default()
    }

    pub(crate) fn operation_family(&self) -> Option<ForgeServerOperationFamily> {
        self.operation_family
    }

    pub(crate) fn operation_name(&self) -> Option<&str> {
        self.operation_name.as_deref()
    }

    pub(crate) fn basis_digest(&self) -> Option<&str> {
        self.basis_digest.as_deref()
    }

    pub(crate) fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }

    pub(crate) fn product_session_identity(&self) -> Option<&str> {
        self.product_session_identity.as_deref()
    }

    pub(crate) fn payload_envelope(&self) -> Option<&ForgeServerOperationInputEnvelope> {
        self.payload_envelope.as_ref()
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerOperationRequestInputBuilder {
    inner: ForgeServerOperationRequestInput,
}

impl ForgeServerOperationRequestInputBuilder {
    pub fn with_operation_family(mut self, operation_family: ForgeServerOperationFamily) -> Self {
        self.inner.operation_family = Some(operation_family);
        self
    }

    pub fn with_operation_name(mut self, operation_name: impl Into<String>) -> Self {
        self.inner.operation_name = Some(operation_name.into());
        self
    }

    pub fn with_basis_digest(mut self, basis_digest: impl Into<String>) -> Self {
        self.inner.basis_digest = Some(basis_digest.into());
        self
    }

    pub fn with_idempotency_key(mut self, idempotency_key: impl Into<String>) -> Self {
        self.inner.idempotency_key = Some(idempotency_key.into());
        self
    }

    pub fn with_product_session_identity(
        mut self,
        product_session_identity: impl Into<String>,
    ) -> Self {
        self.inner.product_session_identity = Some(product_session_identity.into());
        self
    }

    pub fn with_payload_envelope(
        mut self,
        payload_envelope: ForgeServerOperationInputEnvelope,
    ) -> Self {
        self.inner.payload_envelope = Some(payload_envelope);
        self
    }

    pub fn build(self) -> ForgeServerOperationRequestInput {
        self.inner
    }
}
