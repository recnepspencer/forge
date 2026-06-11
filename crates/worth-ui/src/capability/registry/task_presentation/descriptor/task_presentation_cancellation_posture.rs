#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskPresentationCancellationPosture {
    NotCancellable,
    RuntimeCancellable,
    ApplicationCancellable,
    PresentationCancelsTask,
}

impl TaskPresentationCancellationPosture {
    pub fn not_cancellable() -> Self {
        Self::NotCancellable
    }

    pub fn runtime_cancellable() -> Self {
        Self::RuntimeCancellable
    }

    pub fn application_cancellable() -> Self {
        Self::ApplicationCancellable
    }

    pub fn presentation_cancels_task_for_diagnostics() -> Self {
        Self::PresentationCancelsTask
    }

    pub(crate) fn exposes_cancellation(&self) -> bool {
        matches!(
            self,
            Self::RuntimeCancellable | Self::ApplicationCancellable
        )
    }

    pub(crate) fn claims_task_runtime_authority(&self) -> bool {
        matches!(self, Self::PresentationCancelsTask)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::NotCancellable => "not_cancellable",
            Self::RuntimeCancellable => "runtime_cancellable",
            Self::ApplicationCancellable => "application_cancellable",
            Self::PresentationCancelsTask => "presentation_cancels_task",
        }
    }
}
