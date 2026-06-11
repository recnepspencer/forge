#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeOutcomeFamily {
    Loading,
    Ready,
    Denied,
    Advisory,
    Violation,
    Stopped,
    Recoverable,
    Stale,
    Failed,
    Completed,
    Cancelled,
    Retrying,
    Revalidating,
    UnknownForDiagnostics(String),
}

impl RuntimeOutcomeFamily {
    pub fn loading() -> Self {
        Self::Loading
    }

    pub fn ready() -> Self {
        Self::Ready
    }

    pub fn denied() -> Self {
        Self::Denied
    }

    pub fn advisory() -> Self {
        Self::Advisory
    }

    pub fn violation() -> Self {
        Self::Violation
    }

    pub fn stopped() -> Self {
        Self::Stopped
    }

    pub fn recoverable() -> Self {
        Self::Recoverable
    }

    pub fn stale() -> Self {
        Self::Stale
    }

    pub fn failed() -> Self {
        Self::Failed
    }

    pub fn completed() -> Self {
        Self::Completed
    }

    pub fn cancelled() -> Self {
        Self::Cancelled
    }

    pub fn retrying() -> Self {
        Self::Retrying
    }

    pub fn revalidating() -> Self {
        Self::Revalidating
    }

    pub fn unknown_for_diagnostics(name: impl Into<String>) -> Self {
        Self::UnknownForDiagnostics(name.into())
    }

    pub(crate) fn is_known(&self) -> bool {
        !matches!(self, Self::UnknownForDiagnostics(_))
    }

    pub(crate) fn requires_denial_posture(&self) -> bool {
        self.admits_denial_posture()
    }

    pub(crate) fn admits_denial_posture(&self) -> bool {
        matches!(self, Self::Denied | Self::Violation)
    }

    pub(crate) fn requires_recovery_posture(&self) -> bool {
        self.admits_recovery_posture()
    }

    pub(crate) fn admits_recovery_posture(&self) -> bool {
        matches!(
            self,
            Self::Recoverable | Self::Stale | Self::Failed | Self::Retrying | Self::Revalidating
        )
    }

    pub(crate) fn digest_basis(&self) -> String {
        match self {
            Self::Loading => "loading".to_string(),
            Self::Ready => "ready".to_string(),
            Self::Denied => "denied".to_string(),
            Self::Advisory => "advisory".to_string(),
            Self::Violation => "violation".to_string(),
            Self::Stopped => "stopped".to_string(),
            Self::Recoverable => "recoverable".to_string(),
            Self::Stale => "stale".to_string(),
            Self::Failed => "failed".to_string(),
            Self::Completed => "completed".to_string(),
            Self::Cancelled => "cancelled".to_string(),
            Self::Retrying => "retrying".to_string(),
            Self::Revalidating => "revalidating".to_string(),
            Self::UnknownForDiagnostics(name) => format!("unknown:{name}"),
        }
    }
}
