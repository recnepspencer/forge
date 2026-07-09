#[derive(Clone, Debug)]
pub struct WorthServerWorthNativeProductMutationCommand {
    input: crate::WorthServerProductOperationInput,
}

impl WorthServerWorthNativeProductMutationCommand {
    pub fn new(
        operation_name: impl Into<String>,
        payload: crate::WorthServerProductOperationPayload,
    ) -> Self {
        Self {
            input: crate::WorthServerProductOperationInput::new(operation_name, payload),
        }
    }

    pub fn within(mut self, product_session: &crate::WorthServerProductSession) -> Self {
        self.input = self
            .input
            .with_product_session_identity(product_session.identity().as_str());
        self
    }

    pub fn within_identity(mut self, product_session_identity: impl Into<String>) -> Self {
        self.input = self
            .input
            .with_product_session_identity(product_session_identity);
        self
    }

    pub fn against_base_digest(
        mut self,
        base_digest: crate::WorthServerProductOperationBaseDigest,
    ) -> Self {
        self.input = self.input.with_snapshot_precondition(
            crate::WorthServerProductSnapshotPrecondition::at_base_digest(base_digest),
        );
        self
    }

    pub fn against_snapshot_precondition(
        mut self,
        snapshot_precondition: crate::WorthServerProductSnapshotPrecondition,
    ) -> Self {
        self.input = self.input.with_snapshot_precondition(snapshot_precondition);
        self
    }

    pub fn idempotent(mut self, idempotency_key: crate::WorthServerProductIdempotencyKey) -> Self {
        self.input = self.input.with_idempotency_key(idempotency_key);
        self
    }

    pub(crate) fn into_input(self) -> crate::WorthServerProductOperationInput {
        self.input
    }
}
