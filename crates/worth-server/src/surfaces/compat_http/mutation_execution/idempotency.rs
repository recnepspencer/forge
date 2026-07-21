use crate::{
    WorthServerCompatibilityPreparedRequest, WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffDenialCode,
};

use super::response::WorthServerCompatibilityMutation;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerIdempotencyKey {
    value: String,
    canonical_digest: String,
}

impl WorthServerIdempotencyKey {
    pub(crate) fn from_prepared_request(
        prepared_request: &WorthServerCompatibilityPreparedRequest,
    ) -> Result<Option<Self>, WorthServerQueryHandoffDenial> {
        let request_context = prepared_request.admission().request_context();
        let Some(values) = prepared_request
            .request_contract()
            .canonical_headers()
            .values("idempotency-key")
        else {
            return Ok(None);
        };
        if values.len() != 1 {
            return Err(WorthServerQueryHandoffDenial::new(
                WorthServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
                request_context.diagnostics_profile(),
                "compatibility mutation requires a single canonical `idempotency-key` header value",
            ));
        }
        let value = values[0].trim();
        if value.is_empty() {
            return Err(WorthServerQueryHandoffDenial::new(
                WorthServerQueryHandoffDenialCode::CompatibilityMutationRequestInvalid,
                request_context.diagnostics_profile(),
                "compatibility mutation `idempotency-key` header may not be blank",
            ));
        }
        Ok(Some(Self {
            value: value.to_string(),
            canonical_digest: format!("compat-http-idempotency-key-v1|{value}"),
        }))
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub(crate) fn scoped_storage_key(
        &self,
        prepared_request: &WorthServerCompatibilityPreparedRequest,
    ) -> String {
        let request_context = prepared_request.admission().request_context();
        format!(
            "compat-http-idempotency-scope-v1|principal:{}|tenant:{}|workspace:{}|branch:{}|key:{}",
            request_context.authenticated_principal().principal_id(),
            request_context.workspace_target().tenant_id(),
            request_context.workspace_target().workspace_id(),
            request_context.branch_target().canonical_label(),
            self.canonical_digest,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerIdempotentRetryReceipt {
    Authoritative {
        idempotency_key: String,
        request_digest: String,
        canonical_digest: String,
    },
    PreviouslyCompleted {
        idempotency_key: String,
        request_digest: String,
        authoritative_mutation_digest: String,
        canonical_digest: String,
    },
}

impl WorthServerIdempotentRetryReceipt {
    pub(crate) fn authoritative(key: &WorthServerIdempotencyKey, request_digest: &str) -> Self {
        Self::Authoritative {
            idempotency_key: key.value().to_string(),
            request_digest: request_digest.to_string(),
            canonical_digest: format!(
                "compat-http-idempotent-retry-v1|class=executed|key:{}|request:{}",
                key.canonical_digest(),
                request_digest,
            ),
        }
    }

    pub(crate) fn previously_completed(
        key: &WorthServerIdempotencyKey,
        request_digest: &str,
        authoritative_mutation_digest: &str,
    ) -> Self {
        Self::PreviouslyCompleted {
            idempotency_key: key.value().to_string(),
            request_digest: request_digest.to_string(),
            authoritative_mutation_digest: authoritative_mutation_digest.to_string(),
            canonical_digest: format!(
                "compat-http-idempotent-retry-v1|class=previously-completed|key:{}|request:{}|authoritative:{}",
                key.canonical_digest(),
                request_digest,
                authoritative_mutation_digest,
            ),
        }
    }

    pub fn canonical_digest(&self) -> &str {
        match self {
            Self::Authoritative {
                canonical_digest, ..
            }
            | Self::PreviouslyCompleted {
                canonical_digest, ..
            } => canonical_digest,
        }
    }

    pub fn is_previously_completed(&self) -> bool {
        matches!(self, Self::PreviouslyCompleted { .. })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct WorthServerStoredCompatibilityMutation {
    request_digest: String,
    mutation: WorthServerCompatibilityMutation,
}

impl WorthServerStoredCompatibilityMutation {
    pub(crate) fn new(request_digest: String, mutation: WorthServerCompatibilityMutation) -> Self {
        Self {
            request_digest,
            mutation,
        }
    }

    pub(crate) fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub(crate) fn mutation(&self) -> &WorthServerCompatibilityMutation {
        &self.mutation
    }
}
