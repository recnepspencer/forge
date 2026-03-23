use serde::{Deserialize, Serialize};

use crate::history::data::BranchId;
use crate::identity::data::LineageId;
use crate::lineage::data::{
    CorrespondenceCandidateId, CorrespondencePromotionRejectionClass, LineageEventKind,
    LineageEventRecord, LineageFinalizationCounters,
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
    pub branch_id: BranchId,
    pub kind: LineageDecisionKind,
    pub event_id: Option<u64>,
    pub candidate_id: Option<CorrespondenceCandidateId>,
    pub sources: Vec<LineageId>,
    pub targets: Vec<LineageId>,
    pub rejection_class: Option<CorrespondencePromotionRejectionClass>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub(crate) struct LineageDecisionLog {
    decisions: Vec<LineageDecisionRecord>,
}

impl LineageDecisionLog {
    pub(crate) fn single(decision: LineageDecisionRecord) -> Self {
        Self {
            decisions: vec![decision],
        }
    }

    pub(crate) fn new(decisions: Vec<LineageDecisionRecord>) -> Self {
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
}

impl LineageFinalizationArtifact {
    pub(crate) fn new(
        branch_id: BranchId,
        event_batch: FinalizedLineageEventBatch,
        decision_log: LineageDecisionLog,
    ) -> Self {
        Self {
            branch_id,
            event_batch,
            decision_log,
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

    pub(crate) fn publish(&self) -> PublishedLineageArtifact {
        PublishedLineageArtifact {
            branch_id: self.branch_id.clone(),
            lineage_event_ids: self.event_batch.event_ids().to_vec(),
            lineage_events: self.event_batch.events().to_vec(),
            lineage_decision_log: self.decision_log.decisions().to_vec(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PublishedLineageArtifact {
    branch_id: BranchId,
    lineage_event_ids: Vec<u64>,
    lineage_events: Vec<LineageEventRecord>,
    lineage_decision_log: Vec<LineageDecisionRecord>,
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
