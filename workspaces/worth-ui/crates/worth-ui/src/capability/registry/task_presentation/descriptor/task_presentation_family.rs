use super::TaskPresentationProjectionEligibility;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TaskPresentationFamily {
    Progress,
    Cancellable,
    Retryable,
    Blocking,
    Background,
    Completed,
    Failed,
    StatusProjected,
    Unknown(String),
}

impl TaskPresentationFamily {
    pub fn progress() -> Self {
        Self::Progress
    }

    pub fn cancellable() -> Self {
        Self::Cancellable
    }

    pub fn retryable() -> Self {
        Self::Retryable
    }

    pub fn blocking() -> Self {
        Self::Blocking
    }

    pub fn background() -> Self {
        Self::Background
    }

    pub fn completed() -> Self {
        Self::Completed
    }

    pub fn failed() -> Self {
        Self::Failed
    }

    pub fn status_projected() -> Self {
        Self::StatusProjected
    }

    pub fn unknown_for_diagnostics(name: impl Into<String>) -> Self {
        Self::Unknown(name.into())
    }

    pub(crate) fn is_known(&self) -> bool {
        !matches!(self, Self::Unknown(_))
    }

    pub(crate) fn admits_projection_eligibility(
        &self,
        eligibility: &TaskPresentationProjectionEligibility,
    ) -> bool {
        match self {
            Self::Progress => matches!(
                eligibility,
                TaskPresentationProjectionEligibility::ProgressIndicator
            ),
            Self::Blocking => matches!(
                eligibility,
                TaskPresentationProjectionEligibility::BlockingIndicator
            ),
            Self::Completed => matches!(
                eligibility,
                TaskPresentationProjectionEligibility::CompletionBadge
            ),
            Self::Failed | Self::Retryable => matches!(
                eligibility,
                TaskPresentationProjectionEligibility::FailureSummary
            ),
            Self::StatusProjected => matches!(
                eligibility,
                TaskPresentationProjectionEligibility::StatusSummary
            ),
            Self::Cancellable | Self::Background | Self::Unknown(_) => true,
        }
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Progress => "progress".to_string(),
            Self::Cancellable => "cancellable".to_string(),
            Self::Retryable => "retryable".to_string(),
            Self::Blocking => "blocking".to_string(),
            Self::Background => "background".to_string(),
            Self::Completed => "completed".to_string(),
            Self::Failed => "failed".to_string(),
            Self::StatusProjected => "status_projected".to_string(),
            Self::Unknown(name) => format!("unknown:{}", length_prefixed(name)),
        }
    }
}

fn length_prefixed(value: &str) -> String {
    format!("{}:{value}", value.len())
}
