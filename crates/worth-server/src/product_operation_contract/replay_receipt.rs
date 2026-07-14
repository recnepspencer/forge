#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationReplayReceipt {
    Authoritative {
        idempotency_key: String,
        request_digest: String,
        canonical_digest: String,
    },
    Replayed {
        idempotency_key: String,
        request_digest: String,
        authoritative_operation_digest: String,
        canonical_digest: String,
    },
}

impl WorthServerProductOperationReplayReceipt {
    pub(crate) fn authoritative(idempotency_key: &str, request_digest: &str) -> Self {
        Self::Authoritative {
            idempotency_key: idempotency_key.to_string(),
            request_digest: request_digest.to_string(),
            canonical_digest: format!(
                "worth-server-product-operation-replay-v1|class=authoritative|key={idempotency_key}|request={request_digest}"
            ),
        }
    }

    pub(crate) fn replayed(
        idempotency_key: &str,
        request_digest: &str,
        authoritative_operation_digest: &str,
    ) -> Self {
        Self::Replayed {
            idempotency_key: idempotency_key.to_string(),
            request_digest: request_digest.to_string(),
            authoritative_operation_digest: authoritative_operation_digest.to_string(),
            canonical_digest: format!(
                "worth-server-product-operation-replay-v1|class=replayed|key={idempotency_key}|request={request_digest}|authoritative={authoritative_operation_digest}"
            ),
        }
    }

    pub fn canonical_digest(&self) -> &str {
        match self {
            Self::Authoritative {
                canonical_digest, ..
            }
            | Self::Replayed {
                canonical_digest, ..
            } => canonical_digest,
        }
    }

    pub fn is_replayed(&self) -> bool {
        matches!(self, Self::Replayed { .. })
    }

    pub fn idempotency_key(&self) -> &str {
        match self {
            Self::Authoritative {
                idempotency_key, ..
            }
            | Self::Replayed {
                idempotency_key, ..
            } => idempotency_key,
        }
    }

    pub fn request_digest(&self) -> &str {
        match self {
            Self::Authoritative { request_digest, .. } | Self::Replayed { request_digest, .. } => {
                request_digest
            }
        }
    }

    pub fn authoritative_operation_digest(&self) -> Option<&str> {
        match self {
            Self::Authoritative { .. } => None,
            Self::Replayed {
                authoritative_operation_digest,
                ..
            } => Some(authoritative_operation_digest),
        }
    }
}
