use serde::{Deserialize, Serialize};

use crate::lineage::data::{CorrespondenceCandidate, LineageNode};

use super::LineageDecisionRecord;

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
