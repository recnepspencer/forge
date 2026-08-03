pub struct UiSourceRebindRequest {
    snapshot: crate::runtime::WorthUiSettledSourceSnapshot,
    policy: crate::runtime::rebind::UiRebindExecutionPolicy,
    execution: super::UiRebindExecutionRequest,
}

impl UiSourceRebindRequest {
    pub fn new(snapshot: crate::runtime::WorthUiSettledSourceSnapshot) -> Self {
        Self {
            snapshot,
            policy: crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
            execution: super::UiRebindExecutionRequest::new(0),
        }
    }

    pub const fn observed_at_tick(mut self, tick: u64) -> Self {
        self.execution = self.execution.with_now_tick(tick);
        self
    }

    pub const fn with_deadline(
        mut self,
        deadline: crate::runtime::rebind::UiRebindSessionDeadline,
    ) -> Self {
        self.policy = self.policy.with_deadline(deadline);
        self
    }

    pub const fn with_cancellation(
        mut self,
        cancellation: crate::runtime::rebind::UiRebindCancellationRequest,
    ) -> Self {
        self.policy = self.policy.with_cancellation(cancellation);
        self.execution = self.execution.with_cancellation_requested();
        self
    }

    pub const fn with_idempotency(
        mut self,
        idempotency: crate::runtime::rebind::UiRebindIdempotency,
    ) -> Self {
        self.policy = self.policy.with_idempotency(idempotency);
        self
    }

    pub const fn with_retry_tolerance(
        mut self,
        retry: crate::runtime::rebind::UiRebindRetryTolerance,
    ) -> Self {
        self.policy = self.policy.with_retry_tolerance(retry);
        self
    }

    pub const fn with_artifact_policy(
        mut self,
        artifact: crate::runtime::rebind::UiRebindArtifactPolicy,
    ) -> Self {
        self.policy = self.policy.with_artifact_policy(artifact);
        self
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        crate::runtime::WorthUiSettledSourceSnapshot,
        crate::runtime::rebind::UiRebindExecutionPolicy,
        super::UiRebindExecutionRequest,
    ) {
        (self.snapshot, self.policy, self.execution)
    }
}
