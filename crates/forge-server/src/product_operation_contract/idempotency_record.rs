use crate::ForgeServerCompletedProductOperation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductIdempotencyConflict {
    idempotency_key: String,
    conflicting_request_digest: String,
    bound_request_digest: String,
    canonical_digest: String,
}

impl ForgeServerProductIdempotencyConflict {
    pub(crate) fn new(
        idempotency_key: impl Into<String>,
        conflicting_request_digest: impl Into<String>,
        bound_request_digest: impl Into<String>,
    ) -> Self {
        let idempotency_key = idempotency_key.into();
        let conflicting_request_digest = conflicting_request_digest.into();
        let bound_request_digest = bound_request_digest.into();
        let canonical_digest = format!(
            "forge-server-product-idempotency-conflict-v1|key={idempotency_key}|conflicting={conflicting_request_digest}|bound={bound_request_digest}"
        );
        Self {
            idempotency_key,
            conflicting_request_digest,
            bound_request_digest,
            canonical_digest,
        }
    }

    pub fn idempotency_key(&self) -> &str {
        &self.idempotency_key
    }

    pub fn conflicting_request_digest(&self) -> &str {
        &self.conflicting_request_digest
    }

    pub fn bound_request_digest(&self) -> &str {
        &self.bound_request_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug)]
pub struct ForgeServerProductIdempotencyRecord {
    request_digest: String,
    completed_operation: ForgeServerCompletedProductOperation,
}

impl ForgeServerProductIdempotencyRecord {
    pub(crate) fn new(
        request_digest: impl Into<String>,
        completed_operation: ForgeServerCompletedProductOperation,
    ) -> Self {
        Self {
            request_digest: request_digest.into(),
            completed_operation,
        }
    }

    pub(crate) fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub(crate) fn completed_operation(&self) -> &ForgeServerCompletedProductOperation {
        &self.completed_operation
    }
}
