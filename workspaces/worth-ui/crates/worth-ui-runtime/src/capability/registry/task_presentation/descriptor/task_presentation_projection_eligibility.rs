#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskPresentationProjectionEligibility {
    ProgressIndicator,
    StatusSummary,
    BlockingIndicator,
    CompletionBadge,
    FailureSummary,
    HiddenFromProjection,
}

impl TaskPresentationProjectionEligibility {
    pub fn progress_indicator() -> Self {
        Self::ProgressIndicator
    }

    pub fn status_summary() -> Self {
        Self::StatusSummary
    }

    pub fn blocking_indicator() -> Self {
        Self::BlockingIndicator
    }

    pub fn completion_badge() -> Self {
        Self::CompletionBadge
    }

    pub fn failure_summary() -> Self {
        Self::FailureSummary
    }

    pub fn hidden_from_projection() -> Self {
        Self::HiddenFromProjection
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::ProgressIndicator => "progress_indicator",
            Self::StatusSummary => "status_summary",
            Self::BlockingIndicator => "blocking_indicator",
            Self::CompletionBadge => "completion_badge",
            Self::FailureSummary => "failure_summary",
            Self::HiddenFromProjection => "hidden_from_projection",
        }
    }
}
