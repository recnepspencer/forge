#![allow(dead_code)]

use crate::retention::{
    ConservativeRetentionPolicy, RetainedHeadSet, RetentionPolicyClass, StableBasisSet,
};
use worth_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionClosureSummary {
    retained_head_count: u64,
    stable_basis_count: u64,
    closure_commit_count: u64,
    closure_frontier_count: u64,
}

impl RetentionClosureSummary {
    pub(crate) fn new(
        retained_head_count: u64,
        stable_basis_count: u64,
        closure_commit_count: u64,
        closure_frontier_count: u64,
    ) -> Self {
        Self {
            retained_head_count,
            stable_basis_count,
            closure_commit_count,
            closure_frontier_count,
        }
    }

    pub fn from_witness(witness: &crate::RetentionClosureWitness) -> Self {
        Self::new(
            witness.retained_heads().branch_ids().len() as u64,
            witness.stable_bases().basis_labels().len() as u64,
            witness.closure_commit_ids().len() as u64,
            witness.frontier_commit_ids().len() as u64,
        )
    }

    pub fn retained_head_count(&self) -> u64 {
        self.retained_head_count
    }

    pub fn stable_basis_count(&self) -> u64 {
        self.stable_basis_count
    }

    pub fn closure_commit_count(&self) -> u64 {
        self.closure_commit_count
    }

    pub fn closure_frontier_count(&self) -> u64 {
        self.closure_frontier_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetentionCandidatePlan {
    policy_class: RetentionPolicyClass,
    closure_summary: RetentionClosureSummary,
}

impl RetentionCandidatePlan {
    pub(crate) fn new(
        policy_class: RetentionPolicyClass,
        closure_summary: RetentionClosureSummary,
    ) -> Self {
        Self {
            policy_class,
            closure_summary,
        }
    }

    pub fn policy_class(&self) -> &RetentionPolicyClass {
        &self.policy_class
    }

    pub fn closure_summary(&self) -> &RetentionClosureSummary {
        &self.closure_summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetainedAuthoritativeRange {
    branch_id: BranchId,
    commit_ids: Vec<CommitId>,
}

impl RetainedAuthoritativeRange {
    pub(crate) fn new(branch_id: BranchId, mut commit_ids: Vec<CommitId>) -> Self {
        commit_ids.sort();
        commit_ids.dedup();
        Self {
            branch_id,
            commit_ids,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn commit_ids(&self) -> &[CommitId] {
        &self.commit_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConservativeRetentionPlan {
    candidate_plan: RetentionCandidatePlan,
    retained_heads: RetainedHeadSet,
    stable_bases: StableBasisSet,
    policy: ConservativeRetentionPolicy,
}

impl ConservativeRetentionPlan {
    pub(crate) fn new(
        candidate_plan: RetentionCandidatePlan,
        retained_heads: RetainedHeadSet,
        stable_bases: StableBasisSet,
        policy: ConservativeRetentionPolicy,
    ) -> Self {
        Self {
            candidate_plan,
            retained_heads,
            stable_bases,
            policy,
        }
    }

    pub fn candidate_plan(&self) -> &RetentionCandidatePlan {
        &self.candidate_plan
    }

    pub fn retained_heads(&self) -> &RetainedHeadSet {
        &self.retained_heads
    }

    pub fn stable_bases(&self) -> &StableBasisSet {
        &self.stable_bases
    }

    pub fn policy(&self) -> &ConservativeRetentionPolicy {
        &self.policy
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompactionBackedRetentionPlan {
    conservative_plan: ConservativeRetentionPlan,
    compacted_family_labels: Vec<String>,
}

impl CompactionBackedRetentionPlan {
    pub(crate) fn new(
        conservative_plan: ConservativeRetentionPlan,
        compacted_family_labels: Vec<String>,
    ) -> Self {
        Self {
            conservative_plan,
            compacted_family_labels,
        }
    }

    pub fn conservative_plan(&self) -> &ConservativeRetentionPlan {
        &self.conservative_plan
    }

    pub fn compacted_family_labels(&self) -> &[String] {
        &self.compacted_family_labels
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RebuildRequiredRetentionPlan {
    conservative_plan: ConservativeRetentionPlan,
    rebuild_family_labels: Vec<String>,
}

impl RebuildRequiredRetentionPlan {
    pub(crate) fn new(
        conservative_plan: ConservativeRetentionPlan,
        rebuild_family_labels: Vec<String>,
    ) -> Self {
        Self {
            conservative_plan,
            rebuild_family_labels,
        }
    }

    pub fn conservative_plan(&self) -> &ConservativeRetentionPlan {
        &self.conservative_plan
    }

    pub fn rebuild_family_labels(&self) -> &[String] {
        &self.rebuild_family_labels
    }
}
