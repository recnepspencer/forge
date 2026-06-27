#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskPresentationLifecyclePosture {
    RuntimeOwned,
    ApplicationOwned,
    ExternallyCompleted,
    PresentationOwnsLifecycle,
}

impl TaskPresentationLifecyclePosture {
    pub fn runtime_owned() -> Self {
        Self::RuntimeOwned
    }

    pub fn application_owned() -> Self {
        Self::ApplicationOwned
    }

    pub fn externally_completed() -> Self {
        Self::ExternallyCompleted
    }

    pub fn presentation_owns_lifecycle_for_diagnostics() -> Self {
        Self::PresentationOwnsLifecycle
    }

    pub(crate) fn claims_task_runtime_authority(&self) -> bool {
        matches!(self, Self::PresentationOwnsLifecycle)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::RuntimeOwned => "runtime_owned",
            Self::ApplicationOwned => "application_owned",
            Self::ExternallyCompleted => "externally_completed",
            Self::PresentationOwnsLifecycle => "presentation_owns_lifecycle",
        }
    }
}
