use worth_relational::facade::history::CommitId;
use serde::Serialize;

use crate::live_query::basis::StableBasisReadScope;
use crate::live_query::continuation::ContinuationStrategy;
use crate::live_query::evidence::LiveQueryComplexityStatus;
use crate::ForegroundIsolationOutcome;

use super::{
    AdmittedNarrowBatchReceipt, BroadenedBatchReceipt, CaughtUpContinuationBatch,
    ControlLaneBatchReceipt,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ContinuationBatchResult {
    AdmittedNarrow(AdmittedNarrowBatchReceipt),
    Broadened(BroadenedBatchReceipt),
    ControlLane(ControlLaneBatchReceipt),
    CaughtUp(CaughtUpContinuationBatch),
}

impl ContinuationBatchResult {
    pub fn resolved_strategy(&self) -> ContinuationStrategy {
        match self {
            Self::AdmittedNarrow(_) => ContinuationStrategy::AdmittedLayoutNarrow,
            Self::Broadened(_) => ContinuationStrategy::ExplicitBroadened,
            Self::ControlLane(_) => ContinuationStrategy::AuthorityReplayControlLane,
            Self::CaughtUp(batch) => batch.resolved_strategy(),
        }
    }

    pub fn resolved_scope(&self) -> &StableBasisReadScope {
        match self {
            Self::AdmittedNarrow(receipt) => receipt.resolved_scope(),
            Self::Broadened(receipt) => receipt.resolved_scope(),
            Self::ControlLane(receipt) => receipt.resolved_scope(),
            Self::CaughtUp(batch) => batch.resolved_scope(),
        }
    }

    pub fn covered_commit_range(&self) -> Option<(CommitId, CommitId)> {
        match self {
            Self::AdmittedNarrow(receipt) => Some(receipt.covered_commit_range()),
            Self::Broadened(receipt) => Some(receipt.covered_commit_range()),
            Self::ControlLane(receipt) => Some(receipt.covered_commit_range()),
            Self::CaughtUp(_) => None,
        }
    }

    pub fn from_frontier_commit_id(&self) -> Option<CommitId> {
        match self {
            Self::AdmittedNarrow(receipt) => Some(receipt.from_frontier_commit_id()),
            Self::Broadened(receipt) => Some(receipt.from_frontier_commit_id()),
            Self::ControlLane(receipt) => Some(receipt.from_frontier_commit_id()),
            Self::CaughtUp(_) => None,
        }
    }

    pub fn to_frontier_commit_id(&self) -> Option<CommitId> {
        match self {
            Self::AdmittedNarrow(receipt) => Some(receipt.to_frontier_commit_id()),
            Self::Broadened(receipt) => Some(receipt.to_frontier_commit_id()),
            Self::ControlLane(receipt) => Some(receipt.to_frontier_commit_id()),
            Self::CaughtUp(_) => None,
        }
    }

    pub fn covered_commit_count(&self) -> u64 {
        match self {
            Self::AdmittedNarrow(receipt) => receipt.covered_commit_count(),
            Self::Broadened(receipt) => receipt.covered_commit_count(),
            Self::ControlLane(receipt) => receipt.covered_commit_count(),
            Self::CaughtUp(_) => 0,
        }
    }

    pub fn covered_commit_ids(&self) -> &[CommitId] {
        match self {
            Self::AdmittedNarrow(receipt) => receipt.covered_commit_ids(),
            Self::Broadened(receipt) => receipt.covered_commit_ids(),
            Self::ControlLane(receipt) => receipt.covered_commit_ids(),
            Self::CaughtUp(_) => &[],
        }
    }

    pub fn narrowed_item_count(&self) -> u64 {
        match self {
            Self::AdmittedNarrow(receipt) => receipt.narrowed_item_count(),
            Self::Broadened(_) | Self::ControlLane(_) | Self::CaughtUp(_) => 0,
        }
    }

    pub fn broadened_item_count(&self) -> u64 {
        match self {
            Self::AdmittedNarrow(_) | Self::CaughtUp(_) => 0,
            Self::Broadened(receipt) => receipt.broadened_item_count(),
            Self::ControlLane(receipt) => receipt.control_replay_breadth(),
        }
    }

    pub fn support_rows_read(&self) -> u64 {
        match self {
            Self::AdmittedNarrow(receipt) => receipt.support_rows_read(),
            Self::Broadened(receipt) => receipt.support_rows_read(),
            Self::ControlLane(receipt) => receipt.support_rows_read(),
            Self::CaughtUp(_) => 0,
        }
    }

    pub fn scope_lookup_count(&self) -> u64 {
        match self {
            Self::AdmittedNarrow(receipt) => receipt.scope_lookup_count(),
            Self::Broadened(receipt) => receipt.scope_lookup_count(),
            Self::ControlLane(receipt) => receipt.scope_lookup_count(),
            Self::CaughtUp(_) => 0,
        }
    }

    pub fn fallback_class(&self) -> Option<&str> {
        match self {
            Self::AdmittedNarrow(_) | Self::CaughtUp(_) => None,
            Self::Broadened(receipt) => Some(receipt.fallback_class()),
            Self::ControlLane(receipt) => Some(receipt.fallback_class()),
        }
    }

    pub fn complexity_status(&self) -> LiveQueryComplexityStatus {
        match self {
            Self::AdmittedNarrow(_) | Self::CaughtUp(_) => LiveQueryComplexityStatus::Verified,
            Self::Broadened(_) | Self::ControlLane(_) => LiveQueryComplexityStatus::Debt,
        }
    }

    pub fn foreground_isolation(&self) -> &ForegroundIsolationOutcome {
        match self {
            Self::AdmittedNarrow(receipt) => receipt.foreground_isolation(),
            Self::Broadened(receipt) => receipt.foreground_isolation(),
            Self::ControlLane(receipt) => receipt.foreground_isolation(),
            Self::CaughtUp(batch) => batch.foreground_isolation(),
        }
    }
}
