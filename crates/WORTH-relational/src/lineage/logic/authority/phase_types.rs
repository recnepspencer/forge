use crate::history::data::{BranchId, CommitReference};
use crate::identity::data::LineageId;
use crate::lineage::data::{CorrespondenceCandidate, CorrespondenceCandidateId};

#[derive(Debug, Clone)]
pub(in crate::lineage::logic::authority) struct RecordedCorrespondenceCandidate {
    pub(super) candidate: CorrespondenceCandidate,
}

impl RecordedCorrespondenceCandidate {
    pub(in crate::lineage::logic::authority) fn candidate(&self) -> &CorrespondenceCandidate {
        &self.candidate
    }
}

#[derive(Debug, Clone)]
pub(in crate::lineage::logic::authority) struct ValidatedCorrespondenceCandidate {
    pub(super) candidate: CorrespondenceCandidate,
    pub(super) branch_scoped_sources: Vec<BranchScopedLineageRef>,
    pub(super) branch_scoped_targets: Vec<BranchScopedLineageRef>,
}

impl ValidatedCorrespondenceCandidate {
    pub(in crate::lineage::logic::authority) fn candidate(&self) -> &CorrespondenceCandidate {
        &self.candidate
    }

    pub(in crate::lineage::logic::authority) fn branch_scoped_sources(
        &self,
    ) -> &[BranchScopedLineageRef] {
        &self.branch_scoped_sources
    }

    pub(in crate::lineage::logic::authority) fn branch_scoped_targets(
        &self,
    ) -> &[BranchScopedLineageRef] {
        &self.branch_scoped_targets
    }
}

#[derive(Debug, Clone)]
pub(in crate::lineage::logic::authority) struct PromotionEligibleCorrespondenceCandidate {
    pub(super) candidate: CorrespondenceCandidate,
    pub(super) authority: PromotionAuthority,
    pub(super) branch_scoped_sources: Vec<BranchScopedLineageRef>,
    pub(super) branch_scoped_targets: Vec<BranchScopedLineageRef>,
}

impl PromotionEligibleCorrespondenceCandidate {
    pub(in crate::lineage::logic::authority) fn candidate(&self) -> &CorrespondenceCandidate {
        &self.candidate
    }

    pub(in crate::lineage::logic::authority) fn authority(&self) -> &PromotionAuthority {
        &self.authority
    }

    pub(in crate::lineage::logic::authority) fn branch_scoped_sources(
        &self,
    ) -> &[BranchScopedLineageRef] {
        &self.branch_scoped_sources
    }

    pub(in crate::lineage::logic::authority) fn branch_scoped_targets(
        &self,
    ) -> &[BranchScopedLineageRef] {
        &self.branch_scoped_targets
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::lineage::logic::authority) struct BranchScopedLineageRef {
    pub(super) branch_id: BranchId,
    pub(super) lineage_id: LineageId,
}

impl BranchScopedLineageRef {
    pub(in crate::lineage::logic::authority) fn lineage_id(&self) -> LineageId {
        self.lineage_id
    }

    pub(in crate::lineage::logic::authority) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
}

#[derive(Debug, Clone)]
pub(in crate::lineage::logic::authority) struct PromotionAuthority {
    pub(super) branch_id: BranchId,
}

impl PromotionAuthority {
    pub(in crate::lineage::logic::authority) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }
}

#[derive(Debug, Clone)]
pub(in crate::lineage::logic::authority) struct BranchScopedCommitReference {
    pub(super) commit: CommitReference,
}

impl BranchScopedCommitReference {
    pub(in crate::lineage::logic::authority) fn commit(&self) -> &CommitReference {
        &self.commit
    }
}

#[derive(Debug, Clone)]
pub(in crate::lineage::logic::authority) struct LoweredPromotionPlan {
    pub(super) candidate_id: CorrespondenceCandidateId,
    pub(super) authority: PromotionAuthority,
    pub(super) commit: BranchScopedCommitReference,
    pub(super) sources: Vec<BranchScopedLineageRef>,
    pub(super) targets: Vec<BranchScopedLineageRef>,
}

impl LoweredPromotionPlan {
    pub(in crate::lineage::logic::authority) fn candidate_id(&self) -> CorrespondenceCandidateId {
        self.candidate_id
    }

    pub(in crate::lineage::logic::authority) fn branch_id(&self) -> &BranchId {
        self.authority.branch_id()
    }

    pub(in crate::lineage::logic::authority) fn commit(&self) -> &CommitReference {
        self.commit.commit()
    }

    pub(in crate::lineage::logic::authority) fn sources(&self) -> &[BranchScopedLineageRef] {
        &self.sources
    }

    pub(in crate::lineage::logic::authority) fn targets(&self) -> &[BranchScopedLineageRef] {
        &self.targets
    }
}

#[derive(Debug, Clone)]
pub(in crate::lineage::logic::authority) struct ExecutionAuthorizedPromotionPlan {
    pub(super) lowered: LoweredPromotionPlan,
    pub(super) authoritative_anchor: CommitReference,
}

impl ExecutionAuthorizedPromotionPlan {
    pub(in crate::lineage::logic::authority) fn candidate_id(&self) -> CorrespondenceCandidateId {
        self.lowered.candidate_id()
    }

    pub(in crate::lineage::logic::authority) fn branch_id(&self) -> &BranchId {
        self.lowered.branch_id()
    }

    pub(in crate::lineage::logic::authority) fn commit(&self) -> &CommitReference {
        self.lowered.commit()
    }

    pub(in crate::lineage::logic::authority) fn sources(&self) -> &[BranchScopedLineageRef] {
        self.lowered.sources()
    }

    pub(in crate::lineage::logic::authority) fn targets(&self) -> &[BranchScopedLineageRef] {
        self.lowered.targets()
    }

    pub(in crate::lineage::logic::authority) fn authoritative_anchor(&self) -> &CommitReference {
        &self.authoritative_anchor
    }
}
