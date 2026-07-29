use super::super::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationProjectionDenialKind {
    Authorization(WorthQueryOperationAuthorizationDenialKind),
    WorkBudgetExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationProjectionDenial {
    kind: WorthQueryOperationProjectionDenialKind,
    subject: String,
}

impl WorthQueryOperationProjectionDenial {
    pub(super) fn work_budget_exceeded(subject: impl Into<String>) -> Self {
        Self {
            kind: WorthQueryOperationProjectionDenialKind::WorkBudgetExceeded,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryOperationProjectionDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl From<WorthQueryOperationAuthorizationDenial> for WorthQueryOperationProjectionDenial {
    fn from(denial: WorthQueryOperationAuthorizationDenial) -> Self {
        Self {
            kind: WorthQueryOperationProjectionDenialKind::Authorization(denial.kind()),
            subject: denial.subject().to_string(),
        }
    }
}

impl std::fmt::Display for WorthQueryOperationProjectionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application operation projection denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryOperationProjectionDenial {}
