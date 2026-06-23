use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct QuerySubscriptionAsyncRequestIdentityPart {
    key: String,
    value: String,
}

impl QuerySubscriptionAsyncRequestIdentityPart {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum QuerySubscriptionFutureSelectionClass {
    Ordinary,
    Temporal,
    AsyncResource,
    TemporalAsync,
}

impl QuerySubscriptionFutureSelectionClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::Temporal => "temporal",
            Self::AsyncResource => "async_resource",
            Self::TemporalAsync => "temporal_async",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionFutureSelection {
    class: QuerySubscriptionFutureSelectionClass,
    requests_completion_lifecycle: bool,
    async_request_identity: Vec<QuerySubscriptionAsyncRequestIdentityPart>,
    projection_identity: ForgeQueryEvidenceIdentity,
}

impl QuerySubscriptionFutureSelection {
    pub fn ordinary() -> Self {
        Self::new(
            QuerySubscriptionFutureSelectionClass::Ordinary,
            false,
            Vec::new(),
        )
    }

    pub fn temporal() -> Self {
        Self::new(
            QuerySubscriptionFutureSelectionClass::Temporal,
            false,
            Vec::new(),
        )
    }

    pub fn async_resource(requests_completion_lifecycle: bool) -> Self {
        Self::async_resource_with_identity(requests_completion_lifecycle, Vec::new())
    }

    pub fn async_resource_with_identity(
        requests_completion_lifecycle: bool,
        async_request_identity: Vec<QuerySubscriptionAsyncRequestIdentityPart>,
    ) -> Self {
        Self::new(
            QuerySubscriptionFutureSelectionClass::AsyncResource,
            requests_completion_lifecycle,
            async_request_identity,
        )
    }

    pub fn temporal_async(requests_completion_lifecycle: bool) -> Self {
        Self::temporal_async_with_identity(requests_completion_lifecycle, Vec::new())
    }

    pub fn temporal_async_with_identity(
        requests_completion_lifecycle: bool,
        async_request_identity: Vec<QuerySubscriptionAsyncRequestIdentityPart>,
    ) -> Self {
        Self::new(
            QuerySubscriptionFutureSelectionClass::TemporalAsync,
            requests_completion_lifecycle,
            async_request_identity,
        )
    }

    fn new(
        class: QuerySubscriptionFutureSelectionClass,
        requests_completion_lifecycle: bool,
        async_request_identity: Vec<QuerySubscriptionAsyncRequestIdentityPart>,
    ) -> Self {
        let async_request_identity = normalize_async_request_identity(async_request_identity);
        let mut projection_identity = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "query_subscription_future_selection_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("class"), class.as_str())
        .field_usize(
            ForgeQueryEvidenceTag::new("async_request_width"),
            async_request_identity.len(),
        );
        if requests_completion_lifecycle {
            projection_identity = projection_identity.field_shape(
                ForgeQueryEvidenceTag::new("completion_lifecycle"),
                "requested",
            );
        }
        projection_identity = projection_identity.field_value_sequence(
            ForgeQueryEvidenceTag::new("async_keys"),
            async_request_identity.iter().map(|part| part.key()),
        );
        projection_identity = projection_identity.field_value_sequence(
            ForgeQueryEvidenceTag::new("async_values"),
            async_request_identity.iter().map(|part| part.value()),
        );
        let projection_identity = projection_identity.seal();
        Self {
            class,
            requests_completion_lifecycle,
            async_request_identity,
            projection_identity,
        }
    }

    pub fn class(&self) -> QuerySubscriptionFutureSelectionClass {
        self.class
    }

    pub fn requests_completion_lifecycle(&self) -> bool {
        self.requests_completion_lifecycle
    }

    pub fn async_request_identity(&self) -> &[QuerySubscriptionAsyncRequestIdentityPart] {
        &self.async_request_identity
    }

    pub fn projection_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.projection_identity
    }

    #[allow(dead_code)]
    pub(crate) fn projection_digest(&self) -> &str {
        self.projection_identity.as_str()
    }

    pub fn retained_facts(&self) -> Vec<String> {
        let mut facts = vec![format!("future-selection-class:{}", self.class.as_str())];
        if self.requests_completion_lifecycle {
            facts.push("future-selection-completion-lifecycle:requested".to_string());
        }
        facts.extend(self.async_request_identity.iter().map(|part| {
            format!(
                "future-selection-async-request-identity:{}={}",
                part.key(),
                part.value()
            )
        }));
        facts
    }
}

fn normalize_async_request_identity(
    async_request_identity: Vec<QuerySubscriptionAsyncRequestIdentityPart>,
) -> Vec<QuerySubscriptionAsyncRequestIdentityPart> {
    let mut async_request_identity = async_request_identity;
    async_request_identity.sort();
    async_request_identity.dedup();
    async_request_identity
}
