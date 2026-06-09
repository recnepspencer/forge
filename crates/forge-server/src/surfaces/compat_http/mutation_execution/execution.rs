use forge_proof::TransitionOutcome;
use serde_json::Value;

use crate::{
    ForgeServerCompatibilityPreparedRequest, ForgeServerQueryHandoffDeferred,
    ForgeServerQueryHandoffDenial, ForgeServerQueryHandoffFailure,
    ForgeServerQueryHandoffRebindRequired, ForgeServerQueryHandoffStale,
};

pub type ForgeServerCompatibilityMutationOutcome<T> = TransitionOutcome<
    T,
    ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDeferred,
    ForgeServerQueryHandoffStale,
    ForgeServerQueryHandoffRebindRequired,
    ForgeServerQueryHandoffFailure,
>;

#[derive(Clone, Debug)]
pub struct ForgeServerCompatibilityMutationExecutionInput {
    prepared_request: ForgeServerCompatibilityPreparedRequest,
    operation_name: String,
    body: Value,
}

impl ForgeServerCompatibilityMutationExecutionInput {
    pub fn new(
        prepared_request: ForgeServerCompatibilityPreparedRequest,
        operation_name: impl Into<String>,
        body: Value,
    ) -> Self {
        Self {
            prepared_request,
            operation_name: operation_name.into().trim().to_string(),
            body,
        }
    }

    pub(crate) fn into_parts(self) -> (ForgeServerCompatibilityPreparedRequest, String, Value) {
        (self.prepared_request, self.operation_name, self.body)
    }
}
