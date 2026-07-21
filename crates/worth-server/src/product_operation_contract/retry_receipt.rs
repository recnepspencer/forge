#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationRetryReceipt {
    Executed {
        idempotency_key: String,
        request_digest: String,
        canonical_digest: String,
    },
    PreviouslyCommitted {
        idempotency_key: String,
        request_digest: String,
        original_operation_digest: String,
        canonical_digest: String,
    },
}

impl WorthServerProductOperationRetryReceipt {
    pub(crate) fn executed(idempotency_key: &str, request_digest: &str) -> Self {
        let canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-product-operation-retry-v2",
        )
        .field("class", "executed")
        .field("key", idempotency_key)
        .field("request", request_digest)
        .finish();
        Self::Executed {
            idempotency_key: idempotency_key.to_string(),
            request_digest: request_digest.to_string(),
            canonical_digest,
        }
    }

    pub(crate) fn previously_committed(
        idempotency_key: &str,
        request_digest: &str,
        original_operation_digest: &str,
    ) -> Self {
        let canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-product-operation-retry-v2",
        )
        .field("class", "previously-committed")
        .field("key", idempotency_key)
        .field("request", request_digest)
        .field("original", original_operation_digest)
        .finish();
        Self::PreviouslyCommitted {
            idempotency_key: idempotency_key.to_string(),
            request_digest: request_digest.to_string(),
            original_operation_digest: original_operation_digest.to_string(),
            canonical_digest,
        }
    }

    pub fn canonical_digest(&self) -> &str {
        match self {
            Self::Executed {
                canonical_digest, ..
            }
            | Self::PreviouslyCommitted {
                canonical_digest, ..
            } => canonical_digest,
        }
    }

    pub fn is_previously_committed(&self) -> bool {
        matches!(self, Self::PreviouslyCommitted { .. })
    }

    pub fn idempotency_key(&self) -> &str {
        match self {
            Self::Executed {
                idempotency_key, ..
            }
            | Self::PreviouslyCommitted {
                idempotency_key, ..
            } => idempotency_key,
        }
    }

    pub fn request_digest(&self) -> &str {
        match self {
            Self::Executed { request_digest, .. }
            | Self::PreviouslyCommitted { request_digest, .. } => request_digest,
        }
    }

    pub fn original_operation_digest(&self) -> Option<&str> {
        match self {
            Self::Executed { .. } => None,
            Self::PreviouslyCommitted {
                original_operation_digest,
                ..
            } => Some(original_operation_digest),
        }
    }
}
