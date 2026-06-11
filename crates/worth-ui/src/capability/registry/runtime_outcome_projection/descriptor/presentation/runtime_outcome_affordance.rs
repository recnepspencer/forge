#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeOutcomeAffordance {
    None,
    Wait,
    Inspect,
    Retry,
    RecoverableAction,
    Stop,
}

impl RuntimeOutcomeAffordance {
    pub fn none() -> Self {
        Self::None
    }

    pub fn wait() -> Self {
        Self::Wait
    }

    pub fn inspect() -> Self {
        Self::Inspect
    }

    pub fn retry() -> Self {
        Self::Retry
    }

    pub fn recoverable_action() -> Self {
        Self::RecoverableAction
    }

    pub fn stop() -> Self {
        Self::Stop
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Wait => "wait",
            Self::Inspect => "inspect",
            Self::Retry => "retry",
            Self::RecoverableAction => "recoverable_action",
            Self::Stop => "stop",
        }
    }
}
