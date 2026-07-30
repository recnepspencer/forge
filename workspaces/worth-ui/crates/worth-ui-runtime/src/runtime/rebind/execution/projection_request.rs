pub struct UiProjectionRebindRequest {
    observation: worth_ui_query_binding::UiProjectionObservation,
    policy: crate::runtime::rebind::UiRebindExecutionPolicy,
    execution: super::UiRebindExecutionRequest,
}

impl UiProjectionRebindRequest {
    pub fn new(observation: worth_ui_query_binding::UiProjectionObservation) -> Self {
        Self {
            observation,
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

    pub(crate) fn into_parts(
        self,
    ) -> (
        worth_ui_query_binding::UiProjectionObservation,
        crate::runtime::rebind::UiRebindExecutionPolicy,
        super::UiRebindExecutionRequest,
    ) {
        (self.observation, self.policy, self.execution)
    }
}
