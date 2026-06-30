#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskPresentationFailurePosture {
    RuntimeReported,
    ApplicationReported,
    RetryOfferedByRuntime,
    RetryOfferedByApplication,
    PresentationRetriesTask,
}

impl TaskPresentationFailurePosture {
    pub fn runtime_reported() -> Self {
        Self::RuntimeReported
    }

    pub fn application_reported() -> Self {
        Self::ApplicationReported
    }

    pub fn retry_offered_by_runtime() -> Self {
        Self::RetryOfferedByRuntime
    }

    pub fn retry_offered_by_application() -> Self {
        Self::RetryOfferedByApplication
    }

    pub fn presentation_retries_task_for_diagnostics() -> Self {
        Self::PresentationRetriesTask
    }

    pub(crate) fn exposes_retry(&self) -> bool {
        matches!(
            self,
            Self::RetryOfferedByRuntime | Self::RetryOfferedByApplication
        )
    }

    pub(crate) fn claims_task_runtime_authority(&self) -> bool {
        matches!(self, Self::PresentationRetriesTask)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::RuntimeReported => "runtime_reported",
            Self::ApplicationReported => "application_reported",
            Self::RetryOfferedByRuntime => "retry_offered_by_runtime",
            Self::RetryOfferedByApplication => "retry_offered_by_application",
            Self::PresentationRetriesTask => "presentation_retries_task",
        }
    }
}
