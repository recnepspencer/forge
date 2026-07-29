use super::{UiRebindCancellationRequest, UiRebindSafePointPolicy, UiRebindSessionDeadline};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindDeadlinePolicy {
    NoDeadline,
    At(UiRebindSessionDeadline),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindCancellationPolicy {
    NotCancellable,
    AtSafePoints(UiRebindCancellationRequest),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindIdempotency {
    SourceEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindRetryTolerance {
    PreEffectOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindArtifactPolicy {
    Ordinary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindDisclosurePolicy {
    Ordinary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRebindExecutionPolicy {
    deadline: UiRebindDeadlinePolicy,
    cancellation: UiRebindCancellationPolicy,
    idempotency: UiRebindIdempotency,
    retry: UiRebindRetryTolerance,
    artifact: UiRebindArtifactPolicy,
    disclosure: UiRebindDisclosurePolicy,
    safe_points: UiRebindSafePointPolicy,
}

impl UiRebindExecutionPolicy {
    pub const fn ordinary() -> Self {
        Self {
            deadline: UiRebindDeadlinePolicy::NoDeadline,
            cancellation: UiRebindCancellationPolicy::NotCancellable,
            idempotency: UiRebindIdempotency::SourceEvidence,
            retry: UiRebindRetryTolerance::PreEffectOnly,
            artifact: UiRebindArtifactPolicy::Ordinary,
            disclosure: UiRebindDisclosurePolicy::Ordinary,
            safe_points: UiRebindSafePointPolicy::CanonicalPreEffect,
        }
    }

    pub const fn with_deadline(mut self, deadline: UiRebindSessionDeadline) -> Self {
        self.deadline = UiRebindDeadlinePolicy::At(deadline);
        self
    }

    pub const fn with_cancellation(mut self, cancellation: UiRebindCancellationRequest) -> Self {
        self.cancellation = UiRebindCancellationPolicy::AtSafePoints(cancellation);
        self
    }

    pub const fn with_idempotency(mut self, idempotency: UiRebindIdempotency) -> Self {
        self.idempotency = idempotency;
        self
    }

    pub const fn with_retry_tolerance(mut self, retry: UiRebindRetryTolerance) -> Self {
        self.retry = retry;
        self
    }

    pub const fn with_artifact_policy(mut self, artifact: UiRebindArtifactPolicy) -> Self {
        self.artifact = artifact;
        self
    }

    pub const fn with_disclosure_policy(mut self, disclosure: UiRebindDisclosurePolicy) -> Self {
        self.disclosure = disclosure;
        self
    }

    pub const fn with_safe_point_policy(mut self, safe_points: UiRebindSafePointPolicy) -> Self {
        self.safe_points = safe_points;
        self
    }

    pub const fn deadline(self) -> UiRebindDeadlinePolicy {
        self.deadline
    }

    pub const fn cancellation(self) -> UiRebindCancellationPolicy {
        self.cancellation
    }

    pub const fn idempotency(self) -> UiRebindIdempotency {
        self.idempotency
    }

    pub const fn retry_tolerance(self) -> UiRebindRetryTolerance {
        self.retry
    }

    pub const fn artifact_policy(self) -> UiRebindArtifactPolicy {
        self.artifact
    }

    pub const fn disclosure_policy(self) -> UiRebindDisclosurePolicy {
        self.disclosure
    }

    pub const fn safe_point_policy(self) -> UiRebindSafePointPolicy {
        self.safe_points
    }

    pub(crate) fn admits_session(
        self,
        session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    ) -> bool {
        let deadline_admits = match self.deadline {
            UiRebindDeadlinePolicy::NoDeadline => true,
            UiRebindDeadlinePolicy::At(deadline) => deadline.admits(session),
        };
        let cancellation_admits = match self.cancellation {
            UiRebindCancellationPolicy::NotCancellable => true,
            UiRebindCancellationPolicy::AtSafePoints(request) => request.admits(session),
        };
        deadline_admits && cancellation_admits
    }
}
