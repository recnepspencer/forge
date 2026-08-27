use serde::{Deserialize, Serialize};

use crate::errors::data::ErrorContext;
use crate::publication::data::PublicationError;
use crate::transactions::data::CommitLog;

use super::{CommitConflict, CommitPreparationError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionCommitError {
    Conflict {
        error: CommitConflict,
        commit_log: CommitLog,
    },
    Publication {
        error: PublicationError,
        commit_log: CommitLog,
    },
    Preparation {
        error: CommitPreparationError,
        commit_log: CommitLog,
    },
    #[serde(skip)]
    PerformedButDurabilityDeferred {
        settlement: crate::publication::data::DeferredPublicationSettlement,
        error: PublicationError,
        commit_log: CommitLog,
    },
}

impl TransactionCommitError {
    pub fn conflict(error: CommitConflict) -> Self {
        Self::Conflict {
            error,
            commit_log: CommitLog::new(),
        }
    }

    pub fn publication(error: PublicationError) -> Self {
        Self::Publication {
            error,
            commit_log: CommitLog::new(),
        }
    }

    pub fn preparation(error: CommitPreparationError) -> Self {
        Self::Preparation {
            error,
            commit_log: CommitLog::new(),
        }
    }

    pub(crate) fn performed_but_durability_deferred(
        settlement: crate::publication::data::DeferredPublicationSettlement,
        cause: Self,
    ) -> Self {
        let commit_log = cause.commit_log().clone();
        let error = match cause {
            Self::Publication { error, .. } => error,
            other => PublicationError::new(
                crate::publication::bundle::PublicationStage::DurableAppend,
                other.detail(),
            ),
        };
        Self::PerformedButDurabilityDeferred {
            settlement,
            error,
            commit_log,
        }
    }

    pub fn with_commit_log(self, commit_log: CommitLog) -> Self {
        match self {
            Self::Conflict { error, .. } => Self::Conflict { error, commit_log },
            Self::Publication { error, .. } => Self::Publication { error, commit_log },
            Self::Preparation { error, .. } => Self::Preparation { error, commit_log },
            Self::PerformedButDurabilityDeferred {
                settlement, error, ..
            } => Self::PerformedButDurabilityDeferred {
                settlement,
                error,
                commit_log,
            },
        }
    }

    pub fn context(&self) -> &ErrorContext {
        match self {
            Self::Conflict { error, .. } => &error.context,
            Self::Publication { error, .. } => &error.context,
            Self::Preparation { error, .. } => error.context(),
            Self::PerformedButDurabilityDeferred { error, .. } => &error.context,
        }
    }

    pub fn detail(&self) -> String {
        match self {
            Self::Conflict { error, .. } => error.detail(),
            Self::Publication { error, .. } => error.detail.clone(),
            Self::Preparation { error, .. } => error.detail(),
            Self::PerformedButDurabilityDeferred { error, .. } => error.detail.clone(),
        }
    }

    pub fn commit_log(&self) -> &CommitLog {
        match self {
            Self::Conflict { commit_log, .. } => commit_log,
            Self::Publication { commit_log, .. } => commit_log,
            Self::Preparation { commit_log, .. } => commit_log,
            Self::PerformedButDurabilityDeferred { commit_log, .. } => commit_log,
        }
    }

    pub fn performed_commit(&self) -> Option<&crate::history::data::RelationalCommitReceipt> {
        match self {
            Self::PerformedButDurabilityDeferred { settlement, .. } => Some(settlement.commit()),
            _ => None,
        }
    }

    pub fn deferred_settlement(
        &self,
    ) -> Option<&crate::publication::data::DeferredPublicationSettlement> {
        match self {
            Self::PerformedButDurabilityDeferred { settlement, .. } => Some(settlement),
            _ => None,
        }
    }

    pub fn commit_summary(&self) -> &crate::transactions::data::CommitSummary {
        self.commit_log().summary()
    }
}
