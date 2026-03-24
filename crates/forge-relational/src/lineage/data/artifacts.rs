use serde::{Deserialize, Serialize};

use crate::history::data::{BranchId, CommitId};
use crate::identity::data::LineageId;
use crate::lineage::data::{
    CorrespondenceCandidate, CorrespondenceCandidateId, CorrespondencePromotionRejectionClass,
    LineageEventKind, LineageEventRecord, LineageFinalizationCounters, LineageNode,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineageDecisionKind {
    CreateAccepted,
    ReplaceAccepted,
    RetireAccepted,
    CorrespondencePromotionAccepted,
    CorrespondencePromotionRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDecisionRecord {
    pub(crate) branch_id: BranchId,
    pub(crate) kind: LineageDecisionKind,
    pub(crate) event_id: Option<u64>,
    pub(crate) candidate_id: Option<CorrespondenceCandidateId>,
    pub(crate) sources: Vec<LineageId>,
    pub(crate) targets: Vec<LineageId>,
    pub(crate) rejection_class: Option<CorrespondencePromotionRejectionClass>,
}

impl LineageDecisionRecord {
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn kind(&self) -> &LineageDecisionKind {
        &self.kind
    }

    pub fn event_id(&self) -> Option<u64> {
        self.event_id
    }

    pub fn candidate_id(&self) -> Option<CorrespondenceCandidateId> {
        self.candidate_id
    }

    pub fn sources(&self) -> &[LineageId] {
        &self.sources
    }

    pub fn targets(&self) -> &[LineageId] {
        &self.targets
    }

    pub fn rejection_class(&self) -> Option<CorrespondencePromotionRejectionClass> {
        self.rejection_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub(crate) struct LineageDecisionLog {
    decisions: Vec<LineageDecisionRecord>,
}

impl LineageDecisionLog {
    pub(crate) fn single(decision: LineageDecisionRecord) -> Self {
        Self::new(vec![decision])
    }

    pub(crate) fn new(mut decisions: Vec<LineageDecisionRecord>) -> Self {
        decisions.sort_by(canonical_decision_cmp);
        Self { decisions }
    }

    pub(crate) fn decisions(&self) -> &[LineageDecisionRecord] {
        &self.decisions
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FinalizedLineageEventBatch {
    event_ids: Vec<u64>,
    events: Vec<LineageEventRecord>,
    counters: LineageFinalizationCounters,
}

impl FinalizedLineageEventBatch {
    pub(crate) fn new(events: Vec<LineageEventRecord>) -> Self {
        let counters = summarize_event_counters(&events);
        let event_ids = events.iter().map(|event| event.event_id).collect();
        Self {
            event_ids,
            events,
            counters,
        }
    }

    pub(crate) fn single(event: LineageEventRecord) -> Self {
        Self::new(vec![event])
    }

    pub(crate) fn event_ids(&self) -> &[u64] {
        &self.event_ids
    }

    pub(crate) fn events(&self) -> &[LineageEventRecord] {
        &self.events
    }

    pub(crate) fn counters(&self) -> &LineageFinalizationCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LineageFinalizationArtifact {
    branch_id: BranchId,
    event_batch: FinalizedLineageEventBatch,
    decision_log: LineageDecisionLog,
    digest_basis: LineageDigestBasis,
    counters: LineageArtifactCounters,
}

impl LineageFinalizationArtifact {
    pub(crate) fn new(
        branch_id: BranchId,
        event_batch: FinalizedLineageEventBatch,
        decision_log: LineageDecisionLog,
    ) -> Self {
        let event_batch_basis = event_batch_digest_basis(&branch_id, &event_batch);
        let decision_log_basis = decision_log_digest_basis(&branch_id, &decision_log);
        let digest_basis = LineageDigestBasis::new(
            branch_id.clone(),
            event_batch.event_ids().to_vec(),
            event_batch.events().len(),
            decision_log.decisions().len(),
            event_batch_basis,
            decision_log_basis,
        );
        let counters = LineageArtifactCounters::new(
            *event_batch.counters(),
            decision_log.decisions().len(),
        );
        Self {
            branch_id,
            event_batch,
            decision_log,
            digest_basis,
            counters,
        }
    }

    pub(crate) fn single_event(
        branch_id: BranchId,
        event: LineageEventRecord,
        decision: LineageDecisionRecord,
    ) -> Self {
        Self::new(
            branch_id,
            FinalizedLineageEventBatch::single(event),
            LineageDecisionLog::single(decision),
        )
    }

    pub(crate) fn event_batch(&self) -> &FinalizedLineageEventBatch {
        &self.event_batch
    }

    pub(crate) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub(crate) fn decision_log(&self) -> &LineageDecisionLog {
        &self.decision_log
    }

    pub(crate) fn digest_basis(&self) -> &LineageDigestBasis {
        &self.digest_basis
    }

    pub(crate) fn counters(&self) -> &LineageArtifactCounters {
        &self.counters
    }

    pub(crate) fn publish(&self) -> PublishedLineageArtifact {
        PublishedLineageArtifact {
            branch_id: self.branch_id.clone(),
            lineage_event_ids: self.event_batch().event_ids().to_vec(),
            lineage_events: self.event_batch().events().to_vec(),
            lineage_decision_log: self.decision_log().decisions().to_vec(),
            digest_basis: self.digest_basis().clone(),
            counters: *self.counters(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PublishedLineageArtifact {
    branch_id: BranchId,
    lineage_event_ids: Vec<u64>,
    lineage_events: Vec<LineageEventRecord>,
    lineage_decision_log: Vec<LineageDecisionRecord>,
    digest_basis: LineageDigestBasis,
    counters: LineageArtifactCounters,
}

impl PublishedLineageArtifact {
    pub(crate) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub(crate) fn lineage_event_ids(&self) -> &[u64] {
        &self.lineage_event_ids
    }

    pub(crate) fn lineage_events(&self) -> &[LineageEventRecord] {
        &self.lineage_events
    }

    pub(crate) fn lineage_decision_log(&self) -> &[LineageDecisionRecord] {
        &self.lineage_decision_log
    }

    pub(crate) fn decisions_for_candidate(
        &self,
        candidate_id: CorrespondenceCandidateId,
    ) -> impl Iterator<Item = &LineageDecisionRecord> {
        self.lineage_decision_log
            .iter()
            .filter(move |decision| decision.candidate_id == Some(candidate_id))
    }

    pub(crate) fn decisions_for_event_id(
        &self,
        event_id: u64,
    ) -> impl Iterator<Item = &LineageDecisionRecord> {
        self.lineage_decision_log
            .iter()
            .filter(move |decision| decision.event_id == Some(event_id))
    }

    pub(crate) fn decisions_for_rejection_class(
        &self,
        rejection_class: CorrespondencePromotionRejectionClass,
    ) -> impl Iterator<Item = &LineageDecisionRecord> {
        self.lineage_decision_log
            .iter()
            .filter(move |decision| decision.rejection_class == Some(rejection_class))
    }

    pub(crate) fn digest_basis(&self) -> &LineageDigestBasis {
        &self.digest_basis
    }

    pub(crate) fn event_batch_digest_basis(&self) -> &LineageEventBatchDigestBasis {
        &self.digest_basis.event_batch
    }

    pub(crate) fn decision_log_digest_basis(&self) -> &LineageDecisionLogDigestBasis {
        &self.digest_basis.decision_log
    }

    pub(crate) fn observed_event_batch_digest_basis(&self) -> LineageEventBatchDigestBasis {
        event_batch_digest_basis_from_parts(
            &self.branch_id,
            &self.lineage_event_ids,
            &self.lineage_events,
        )
    }

    pub(crate) fn observed_decision_log_digest_basis(&self) -> LineageDecisionLogDigestBasis {
        decision_log_digest_basis_from_parts(&self.branch_id, &self.lineage_decision_log)
    }

    pub(crate) fn counters(&self) -> LineageArtifactCounters {
        self.counters
    }

    pub(crate) fn has_authority_content(&self) -> bool {
        !self.lineage_events.is_empty() || !self.lineage_decision_log.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn lineage_events_mut(&mut self) -> &mut Vec<LineageEventRecord> {
        &mut self.lineage_events
    }

    #[cfg(test)]
    pub(crate) fn lineage_decision_log_mut(&mut self) -> &mut Vec<LineageDecisionRecord> {
        &mut self.lineage_decision_log
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LineageRejectionArtifact {
    branch_id: BranchId,
    decision_log: LineageDecisionLog,
}

impl LineageRejectionArtifact {
    pub(crate) fn single_rejected_promotion(decision: LineageDecisionRecord) -> Self {
        Self {
            branch_id: decision.branch_id.clone(),
            decision_log: LineageDecisionLog::single(decision),
        }
    }

    pub(crate) fn decision_log(&self) -> &LineageDecisionLog {
        &self.decision_log
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDigestBasis {
    branch_id: BranchId,
    canonical_event_ids: Vec<u64>,
    lineage_event_count: usize,
    lineage_decision_count: usize,
    event_batch: LineageEventBatchDigestBasis,
    decision_log: LineageDecisionLogDigestBasis,
}

impl LineageDigestBasis {
    fn new(
        branch_id: BranchId,
        canonical_event_ids: Vec<u64>,
        lineage_event_count: usize,
        lineage_decision_count: usize,
        event_batch: LineageEventBatchDigestBasis,
        decision_log: LineageDecisionLogDigestBasis,
    ) -> Self {
        Self {
            branch_id,
            canonical_event_ids,
            lineage_event_count,
            lineage_decision_count,
            event_batch,
            decision_log,
        }
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn canonical_event_ids(&self) -> &[u64] {
        &self.canonical_event_ids
    }

    pub fn lineage_event_count(&self) -> usize {
        self.lineage_event_count
    }

    pub fn lineage_decision_count(&self) -> usize {
        self.lineage_decision_count
    }

    pub fn event_batch(&self) -> &LineageEventBatchDigestBasis {
        &self.event_batch
    }

    pub fn decision_log(&self) -> &LineageDecisionLogDigestBasis {
        &self.decision_log
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageEventBatchDigestBasis {
    branch_id: BranchId,
    canonical_event_ids: Vec<u64>,
    canonical_commit_ids: Vec<CommitId>,
    canonical_event_kinds: Vec<LineageEventKind>,
    canonical_source_orderings: Vec<Vec<LineageId>>,
    canonical_target_orderings: Vec<Vec<LineageId>>,
}

impl LineageEventBatchDigestBasis {
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn canonical_event_ids(&self) -> &[u64] {
        &self.canonical_event_ids
    }

    pub fn canonical_commit_ids(&self) -> &[CommitId] {
        &self.canonical_commit_ids
    }

    pub fn canonical_event_kinds(&self) -> &[LineageEventKind] {
        &self.canonical_event_kinds
    }

    pub fn canonical_source_orderings(&self) -> &[Vec<LineageId>] {
        &self.canonical_source_orderings
    }

    pub fn canonical_target_orderings(&self) -> &[Vec<LineageId>] {
        &self.canonical_target_orderings
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDecisionLogDigestBasis {
    branch_id: BranchId,
    canonical_decision_kinds: Vec<LineageDecisionKind>,
    canonical_event_ids: Vec<Option<u64>>,
    canonical_candidate_ids: Vec<Option<CorrespondenceCandidateId>>,
    canonical_rejection_classes: Vec<Option<CorrespondencePromotionRejectionClass>>,
    canonical_source_orderings: Vec<Vec<LineageId>>,
    canonical_target_orderings: Vec<Vec<LineageId>>,
}

impl LineageDecisionLogDigestBasis {
    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub fn canonical_decision_kinds(&self) -> &[LineageDecisionKind] {
        &self.canonical_decision_kinds
    }

    pub fn canonical_event_ids(&self) -> &[Option<u64>] {
        &self.canonical_event_ids
    }

    pub fn canonical_candidate_ids(&self) -> &[Option<CorrespondenceCandidateId>] {
        &self.canonical_candidate_ids
    }

    pub fn canonical_rejection_classes(
        &self,
    ) -> &[Option<CorrespondencePromotionRejectionClass>] {
        &self.canonical_rejection_classes
    }

    pub fn canonical_source_orderings(&self) -> &[Vec<LineageId>] {
        &self.canonical_source_orderings
    }

    pub fn canonical_target_orderings(&self) -> &[Vec<LineageId>] {
        &self.canonical_target_orderings
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageArtifactCounters {
    pub finalization: LineageFinalizationCounters,
    pub decision_log_width: usize,
}

impl LineageArtifactCounters {
    fn new(finalization: LineageFinalizationCounters, decision_log_width: usize) -> Self {
        Self {
            finalization,
            decision_log_width,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageCheckpointDigestBasis {
    pub published_lineage_commit_count: usize,
    pub canonical_published_event_ids: Vec<u64>,
    pub published_lineage_event_count: usize,
    pub published_lineage_decision_count: usize,
}

impl LineageCheckpointDigestBasis {
    pub fn new(
        published_lineage_commit_count: usize,
        canonical_published_event_ids: Vec<u64>,
        published_lineage_event_count: usize,
        published_lineage_decision_count: usize,
    ) -> Self {
        Self {
            published_lineage_commit_count,
            canonical_published_event_ids,
            published_lineage_event_count,
            published_lineage_decision_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageCheckpointCounters {
    pub node_count: usize,
    pub correspondence_candidate_count: usize,
    pub rejected_decision_count: usize,
}

impl LineageCheckpointCounters {
    fn new(
        node_count: usize,
        correspondence_candidate_count: usize,
        rejected_decision_count: usize,
    ) -> Self {
        Self {
            node_count,
            correspondence_candidate_count,
            rejected_decision_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageCheckpointArtifact {
    digest_basis: LineageCheckpointDigestBasis,
    counters: LineageCheckpointCounters,
    nodes: Vec<LineageNode>,
    correspondence_candidates: Vec<CorrespondenceCandidate>,
    rejected_decisions: Vec<LineageDecisionRecord>,
}

impl LineageCheckpointArtifact {
    pub fn new(
        digest_basis: LineageCheckpointDigestBasis,
        nodes: Vec<LineageNode>,
        correspondence_candidates: Vec<CorrespondenceCandidate>,
        rejected_decisions: Vec<LineageDecisionRecord>,
    ) -> Self {
        let counters = LineageCheckpointCounters::new(
            nodes.len(),
            correspondence_candidates.len(),
            rejected_decisions.len(),
        );
        Self {
            digest_basis,
            counters,
            nodes,
            correspondence_candidates,
            rejected_decisions,
        }
    }

    pub fn digest_basis(&self) -> &LineageCheckpointDigestBasis {
        &self.digest_basis
    }

    pub fn counters(&self) -> LineageCheckpointCounters {
        self.counters
    }

    pub fn nodes(&self) -> &[LineageNode] {
        &self.nodes
    }

    pub fn correspondence_candidates(&self) -> &[CorrespondenceCandidate] {
        &self.correspondence_candidates
    }

    pub fn rejected_decisions(&self) -> &[LineageDecisionRecord] {
        &self.rejected_decisions
    }
}

fn summarize_event_counters(events: &[LineageEventRecord]) -> LineageFinalizationCounters {
    let mut counters = LineageFinalizationCounters {
        event_batch_width: events.len(),
        ..LineageFinalizationCounters::default()
    };
    for event in events {
        match event.kind {
            LineageEventKind::Create => counters.created_event_count += 1,
            LineageEventKind::Replace => counters.replaced_event_count += 1,
            LineageEventKind::Retire => counters.retired_event_count += 1,
            LineageEventKind::Correspond => counters.promoted_correspondence_count += 1,
            LineageEventKind::Split | LineageEventKind::Merge => {}
        }
    }
    counters
}

fn event_batch_digest_basis(
    branch_id: &BranchId,
    event_batch: &FinalizedLineageEventBatch,
) -> LineageEventBatchDigestBasis {
    event_batch_digest_basis_from_parts(branch_id, event_batch.event_ids(), event_batch.events())
}

fn event_batch_digest_basis_from_parts(
    branch_id: &BranchId,
    event_ids: &[u64],
    events: &[LineageEventRecord],
) -> LineageEventBatchDigestBasis {
    LineageEventBatchDigestBasis {
        branch_id: branch_id.clone(),
        canonical_event_ids: event_ids.to_vec(),
        canonical_commit_ids: events
            .iter()
            .map(|event| event.commit.commit_id)
            .collect(),
        canonical_event_kinds: events.iter().map(|event| event.kind).collect(),
        canonical_source_orderings: events.iter().map(|event| event.sources.clone()).collect(),
        canonical_target_orderings: events.iter().map(|event| event.targets.clone()).collect(),
    }
}

fn decision_log_digest_basis(
    branch_id: &BranchId,
    decision_log: &LineageDecisionLog,
) -> LineageDecisionLogDigestBasis {
    decision_log_digest_basis_from_parts(branch_id, decision_log.decisions())
}

fn decision_log_digest_basis_from_parts(
    branch_id: &BranchId,
    decisions: &[LineageDecisionRecord],
) -> LineageDecisionLogDigestBasis {
    LineageDecisionLogDigestBasis {
        branch_id: branch_id.clone(),
        canonical_decision_kinds: decisions.iter().map(|decision| decision.kind.clone()).collect(),
        canonical_event_ids: decisions.iter().map(|decision| decision.event_id).collect(),
        canonical_candidate_ids: decisions.iter().map(|decision| decision.candidate_id).collect(),
        canonical_rejection_classes: decisions
            .iter()
            .map(|decision| decision.rejection_class)
            .collect(),
        canonical_source_orderings: decisions.iter().map(|decision| decision.sources.clone()).collect(),
        canonical_target_orderings: decisions.iter().map(|decision| decision.targets.clone()).collect(),
    }
}

fn canonical_decision_cmp(
    left: &LineageDecisionRecord,
    right: &LineageDecisionRecord,
) -> std::cmp::Ordering {
    left.branch_id
        .cmp(&right.branch_id)
        .then_with(|| left.event_id.unwrap_or(u64::MAX).cmp(&right.event_id.unwrap_or(u64::MAX)))
        .then_with(|| {
            left.candidate_id
                .map(|id| id.0)
                .unwrap_or(u64::MAX)
                .cmp(&right.candidate_id.map(|id| id.0).unwrap_or(u64::MAX))
        })
        .then_with(|| canonical_decision_kind_rank(left.kind.clone()).cmp(&canonical_decision_kind_rank(right.kind.clone())))
        .then_with(|| {
            left.rejection_class
                .map(canonical_rejection_class_rank)
                .unwrap_or(u8::MAX)
                .cmp(&right
                    .rejection_class
                    .map(canonical_rejection_class_rank)
                    .unwrap_or(u8::MAX))
        })
        .then_with(|| left.sources.cmp(&right.sources))
        .then_with(|| left.targets.cmp(&right.targets))
}

fn canonical_decision_kind_rank(kind: LineageDecisionKind) -> u8 {
    match kind {
        LineageDecisionKind::CreateAccepted => 0,
        LineageDecisionKind::ReplaceAccepted => 1,
        LineageDecisionKind::RetireAccepted => 2,
        LineageDecisionKind::CorrespondencePromotionAccepted => 3,
        LineageDecisionKind::CorrespondencePromotionRejected => 4,
    }
}

fn canonical_rejection_class_rank(class: CorrespondencePromotionRejectionClass) -> u8 {
    match class {
        CorrespondencePromotionRejectionClass::CandidateMissing => 0,
        CorrespondencePromotionRejectionClass::MissingLineageReference => 1,
        CorrespondencePromotionRejectionClass::EmptyEndpointSet => 2,
        CorrespondencePromotionRejectionClass::DuplicateEndpointReference => 3,
        CorrespondencePromotionRejectionClass::OverlappingSourceAndTarget => 4,
        CorrespondencePromotionRejectionClass::CommitBranchMismatch => 5,
        CorrespondencePromotionRejectionClass::BranchScopeMismatch => 6,
        CorrespondencePromotionRejectionClass::CommitNotBranchHead => 7,
        CorrespondencePromotionRejectionClass::AuthorityPublicationFailed => 8,
    }
}
