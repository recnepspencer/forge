#[derive(Clone, Debug)]
pub struct ForgeServerForgeNativeProductMutationCommand {
    input: crate::ForgeServerProductOperationInput,
}

impl ForgeServerForgeNativeProductMutationCommand {
    pub fn new(
        operation_name: impl Into<String>,
        payload: crate::ForgeServerProductOperationPayload,
    ) -> Self {
        Self {
            input: crate::ForgeServerProductOperationInput::new(operation_name, payload),
        }
    }

    pub fn within(mut self, product_session: &crate::ForgeServerProductSession) -> Self {
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
        base_digest: crate::ForgeServerProductOperationBaseDigest,
    ) -> Self {
        self.input = self.input.with_snapshot_precondition(
            crate::ForgeServerProductSnapshotPrecondition::at_base_digest(base_digest),
        );
        self
    }

    pub fn against_snapshot_precondition(
        mut self,
        snapshot_precondition: crate::ForgeServerProductSnapshotPrecondition,
    ) -> Self {
        self.input = self.input.with_snapshot_precondition(snapshot_precondition);
        self
    }

    pub fn idempotent(mut self, idempotency_key: crate::ForgeServerProductIdempotencyKey) -> Self {
        self.input = self.input.with_idempotency_key(idempotency_key);
        self
    }

    pub(crate) fn into_input(self) -> crate::ForgeServerProductOperationInput {
        self.input
    }
}
