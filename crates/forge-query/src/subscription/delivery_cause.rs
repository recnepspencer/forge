use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySubscriptionDeliveryCauseKind {
    RelationalPatch,
    FreshnessOnly,
    WindowEntry,
    WindowExit,
    Deadline,
    PreviousValueTransition,
    AsyncCompletion,
    AsyncDeniedCompletion,
    AsyncRetry,
    AsyncRevalidation,
    MixedCause,
}

impl QuerySubscriptionDeliveryCauseKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RelationalPatch => "relational_patch",
            Self::FreshnessOnly => "freshness_only",
            Self::WindowEntry => "window_entry",
            Self::WindowExit => "window_exit",
            Self::Deadline => "deadline",
            Self::PreviousValueTransition => "previous_value_transition",
            Self::AsyncCompletion => "async_completion",
            Self::AsyncDeniedCompletion => "async_denied_completion",
            Self::AsyncRetry => "async_retry",
            Self::AsyncRevalidation => "async_revalidation",
            Self::MixedCause => "mixed_cause",
        }
    }

    pub fn has_relational_patch(self) -> bool {
        matches!(self, Self::RelationalPatch)
    }

    pub fn requires_previous_value(self) -> bool {
        matches!(self, Self::PreviousValueTransition)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySubscriptionDeliveryCause {
    kind: QuerySubscriptionDeliveryCauseKind,
    evidence_digest: String,
    delivery_cause_digest: String,
}

impl QuerySubscriptionDeliveryCause {
    pub fn relational_patch(evidence_digest: impl Into<String>) -> Self {
        Self::new(
            QuerySubscriptionDeliveryCauseKind::RelationalPatch,
            evidence_digest,
        )
    }

    pub fn time_only(
        kind: QuerySubscriptionDeliveryCauseKind,
        evidence_digest: impl Into<String>,
    ) -> Self {
        assert!(
            !kind.has_relational_patch(),
            "time-only delivery causes cannot reuse the relational patch kind"
        );
        Self::new(kind, evidence_digest)
    }

    #[allow(dead_code)]
    pub(crate) fn classified(
        kind: QuerySubscriptionDeliveryCauseKind,
        evidence_digest: impl Into<String>,
    ) -> Self {
        Self::new(kind, evidence_digest)
    }

    fn new(kind: QuerySubscriptionDeliveryCauseKind, evidence_digest: impl Into<String>) -> Self {
        let evidence_digest = evidence_digest.into();
        let delivery_cause_digest = hash_parts(&[
            "query_subscription_delivery_cause_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("evidence:{evidence_digest}"),
        ]);
        Self {
            kind,
            evidence_digest,
            delivery_cause_digest,
        }
    }

    pub fn kind(&self) -> QuerySubscriptionDeliveryCauseKind {
        self.kind
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn delivery_cause_digest(&self) -> &str {
        &self.delivery_cause_digest
    }

    pub fn has_relational_patch(&self) -> bool {
        self.kind.has_relational_patch()
    }
}
