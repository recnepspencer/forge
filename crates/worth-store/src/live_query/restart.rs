use crate::live_query::basis::{StableBasisHandle, StableBasisReadRequest};
use crate::live_query::evidence::LiveQueryComplexityStatus;
use crate::live_query::retention_descriptor::ContinuationRetentionStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StableBasisSurvival {
    Retained,
    DegradedButRecoverable { fallback_class: String },
    Rejected { reason: String },
}

impl StableBasisSurvival {
    pub(crate) fn from_request(request: &StableBasisReadRequest) -> Self {
        Self::from_retention_status(request.retention_status())
    }

    pub(crate) fn from_handle(handle: &StableBasisHandle) -> Self {
        Self::from_retention_status(handle.retention_status())
    }

    pub(crate) fn from_retention_status(status: &ContinuationRetentionStatus) -> Self {
        match status {
            ContinuationRetentionStatus::Retained => Self::Retained,
            ContinuationRetentionStatus::Degraded { fallback_class } => {
                Self::DegradedButRecoverable {
                    fallback_class: fallback_class.clone(),
                }
            }
            ContinuationRetentionStatus::Rejected { reason } => Self::Rejected {
                reason: reason.clone(),
            },
        }
    }

    pub(crate) fn complexity_status(&self) -> LiveQueryComplexityStatus {
        match self {
            Self::Retained => LiveQueryComplexityStatus::Verified,
            Self::DegradedButRecoverable { .. } | Self::Rejected { .. } => {
                LiveQueryComplexityStatus::Debt
            }
        }
    }

    pub(crate) fn to_retention_status(&self) -> ContinuationRetentionStatus {
        match self {
            Self::Retained => ContinuationRetentionStatus::Retained,
            Self::DegradedButRecoverable { fallback_class } => {
                ContinuationRetentionStatus::Degraded {
                    fallback_class: fallback_class.clone(),
                }
            }
            Self::Rejected { reason } => ContinuationRetentionStatus::Rejected {
                reason: reason.clone(),
            },
        }
    }
}
