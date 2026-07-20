#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductAuthorityScope {
    value: String,
}

impl WorthServerProductAuthorityScope {
    pub fn new(value: impl Into<String>) -> Result<Self, String> {
        let value = value.into().trim().to_string();
        if value.is_empty() {
            return Err("durable product authority scope must be non-blank".to_string());
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductIdempotencyRetention {
    AtLeastSeconds(u64),
    Indefinite,
}

impl WorthServerProductIdempotencyRetention {
    pub fn at_least_seconds(seconds: u64) -> Result<Self, String> {
        if seconds == 0 {
            return Err("durable product idempotency retention must be nonzero".to_string());
        }
        Ok(Self::AtLeastSeconds(seconds))
    }

    pub fn canonical_label(&self) -> String {
        match self {
            Self::AtLeastSeconds(seconds) => format!("at-least-seconds:{seconds}"),
            Self::Indefinite => "indefinite".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerProductDurabilityCapability {
    MutationWithoutAtomicCompletionV1,
    AtomicMutationCompletionV1,
}

impl WorthServerProductDurabilityCapability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MutationWithoutAtomicCompletionV1 => "mutation-without-atomic-completion-v1",
            Self::AtomicMutationCompletionV1 => "atomic-mutation-completion-v1",
        }
    }

    pub(crate) fn satisfies(self, required: Self) -> bool {
        self == required
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDurableProductMutationContract {
    authority_scope: WorthServerProductAuthorityScope,
    idempotency_retention: WorthServerProductIdempotencyRetention,
    required_capability: WorthServerProductDurabilityCapability,
    canonical_digest: String,
}

impl WorthServerDurableProductMutationContract {
    pub fn atomic(
        authority_scope: WorthServerProductAuthorityScope,
        idempotency_retention: WorthServerProductIdempotencyRetention,
    ) -> Self {
        let required_capability =
            WorthServerProductDurabilityCapability::AtomicMutationCompletionV1;
        let retention = idempotency_retention.canonical_label();
        let canonical_digest = crate::canonical_digest::WorthServerCanonicalDigestBuilder::new(
            "worth-server-durable-product-mutation-contract-v2",
        )
        .field("scope", authority_scope.value())
        .field("retention", &retention)
        .field("capability", required_capability.as_str())
        .finish();
        Self {
            authority_scope,
            idempotency_retention,
            required_capability,
            canonical_digest,
        }
    }

    pub fn authority_scope(&self) -> &WorthServerProductAuthorityScope {
        &self.authority_scope
    }

    pub fn idempotency_retention(&self) -> &WorthServerProductIdempotencyRetention {
        &self.idempotency_retention
    }

    pub fn required_capability(&self) -> WorthServerProductDurabilityCapability {
        self.required_capability
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
