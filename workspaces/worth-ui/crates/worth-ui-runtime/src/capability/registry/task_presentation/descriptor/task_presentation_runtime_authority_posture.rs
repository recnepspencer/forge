#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskPresentationRuntimeAuthorityPosture {
    PresentationOnly,
    RuntimeStateReference,
    ApplicationStateReference,
    OwnsTaskRuntime,
}

impl TaskPresentationRuntimeAuthorityPosture {
    pub fn presentation_only() -> Self {
        Self::PresentationOnly
    }

    pub fn runtime_state_reference() -> Self {
        Self::RuntimeStateReference
    }

    pub fn application_state_reference() -> Self {
        Self::ApplicationStateReference
    }

    pub fn owns_task_runtime_for_diagnostics() -> Self {
        Self::OwnsTaskRuntime
    }

    pub(crate) fn claims_task_runtime_authority(&self) -> bool {
        matches!(self, Self::OwnsTaskRuntime)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::PresentationOnly => "presentation_only",
            Self::RuntimeStateReference => "runtime_state_reference",
            Self::ApplicationStateReference => "application_state_reference",
            Self::OwnsTaskRuntime => "owns_task_runtime",
        }
    }
}
