use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

use super::BranchDeltaLayerId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRewriteRequest {
    pub branch_id: BranchId,
    pub target_commit_id: CommitId,
}

impl BranchDeltaRewriteRequest {
    pub fn new(branch_id: BranchId, target_commit_id: CommitId) -> Self {
        Self {
            branch_id,
            target_commit_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaRewriteStrategy {
    NotNeeded,
    ReplaceContiguousSegment,
    RejectAsTooBroad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaRewritePolicyDecision {
    NoAction,
    Defer,
    CompactNow,
    RejectAsTooBroad,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RewriteEligibleDeltaSegment {
    branch_id: BranchId,
    base_frontier_commit_id: Option<CommitId>,
    target_frontier_commit_id: CommitId,
    layer_ids: Vec<BranchDeltaLayerId>,
    commit_ids: Vec<CommitId>,
}

impl RewriteEligibleDeltaSegment {
    pub(crate) fn new(
        branch_id: BranchId,
        base_frontier_commit_id: Option<CommitId>,
        target_frontier_commit_id: CommitId,
        layer_ids: Vec<BranchDeltaLayerId>,
        commit_ids: Vec<CommitId>,
    ) -> Self {
        Self {
            branch_id,
            base_frontier_commit_id,
            target_frontier_commit_id,
            layer_ids,
            commit_ids,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn base_frontier_commit_id(&self) -> Option<CommitId> {
        self.base_frontier_commit_id
    }

    pub fn target_frontier_commit_id(&self) -> CommitId {
        self.target_frontier_commit_id
    }

    pub fn layer_ids(&self) -> &[BranchDeltaLayerId] {
        &self.layer_ids
    }

    pub fn commit_ids(&self) -> &[CommitId] {
        &self.commit_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRewritePlan {
    strategy: BranchDeltaRewriteStrategy,
    segment: Option<RewriteEligibleDeltaSegment>,
    rewrite_breadth: usize,
}

impl BranchDeltaRewritePlan {
    pub(crate) fn new(
        strategy: BranchDeltaRewriteStrategy,
        segment: Option<RewriteEligibleDeltaSegment>,
        rewrite_breadth: usize,
    ) -> Self {
        Self {
            strategy,
            segment,
            rewrite_breadth,
        }
    }

    pub fn strategy(&self) -> BranchDeltaRewriteStrategy {
        self.strategy
    }

    pub fn segment(&self) -> Option<&RewriteEligibleDeltaSegment> {
        self.segment.as_ref()
    }

    pub fn rewrite_breadth(&self) -> usize {
        self.rewrite_breadth
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRewriteRecommendation {
    pub decision: BranchDeltaRewritePolicyDecision,
    pub plan: BranchDeltaRewritePlan,
    pub recommended_layer_width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BranchDeltaAutoCompactDisposition {
    NoAction,
    Deferred,
    Compacted,
    RejectedAsTooBroad,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaAutoCompactOutcome {
    pub disposition: BranchDeltaAutoCompactDisposition,
    pub recommendation: BranchDeltaRewriteRecommendation,
    pub rewrite_receipt: Option<BranchDeltaRewriteReceipt>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRewriteReceipt {
    pub branch_id: BranchId,
    pub target_frontier_commit_id: CommitId,
    pub replacement_layer_id: Option<BranchDeltaLayerId>,
    pub replaced_layer_ids: Vec<BranchDeltaLayerId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchDeltaRebuildReceipt {
    pub branch_id: BranchId,
    pub rebuilt_layer_count: usize,
}
