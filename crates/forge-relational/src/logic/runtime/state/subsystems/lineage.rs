use std::collections::{BTreeMap, BTreeSet};

use crate::history::data::BranchId;
use crate::identity::data::LineageId;
use crate::lineage::data::{
    CorrespondenceCandidate, LineageDecisionRecord, LineageEventRecord, LineageNode,
};
use crate::logic::runtime::state::subsystems::RuntimeSubsystem;

#[derive(Debug, Clone, Default)]
pub(crate) struct LineageSubsystem {
    pub(crate) nodes: BTreeMap<crate::identity::data::LineageId, LineageNode>,
    pub(crate) events: Vec<LineageEventRecord>,
    pub(crate) branch_event_positions: BTreeMap<BranchId, Vec<usize>>,
    pub(crate) branch_source_event_positions: BTreeMap<BranchId, BTreeMap<LineageId, Vec<usize>>>,
    pub(crate) correspondence_candidates: Vec<CorrespondenceCandidate>,
    pub(crate) rejected_decisions: Vec<LineageDecisionRecord>,
    pub(crate) next_lineage_id: u64,
    pub(crate) next_event_id: u64,
    pub(crate) next_candidate_id: u64,
}

impl LineageSubsystem {
    fn empty() -> Self {
        Self {
            nodes: BTreeMap::new(),
            events: Vec::new(),
            branch_event_positions: BTreeMap::new(),
            branch_source_event_positions: BTreeMap::new(),
            correspondence_candidates: Vec::new(),
            rejected_decisions: Vec::new(),
            next_lineage_id: 1,
            next_event_id: 1,
            next_candidate_id: 1,
        }
    }

    pub(crate) fn record_event(&mut self, event: LineageEventRecord) {
        let event_position = self.events.len();
        self.branch_event_positions
            .entry(event.branch_id().clone())
            .or_default()
            .push(event_position);
        let source_positions = self
            .branch_source_event_positions
            .entry(event.branch_id().clone())
            .or_default();
        for source in event.sources() {
            source_positions
                .entry(*source)
                .or_default()
                .push(event_position);
        }
        self.events.push(event);
    }

    pub(crate) fn branch_events(&self, branch_id: &BranchId) -> impl Iterator<Item = &LineageEventRecord> {
        self.branch_event_positions
            .get(branch_id)
            .into_iter()
            .flat_map(|positions| positions.iter())
            .map(|position| &self.events[*position])
    }

    pub(crate) fn branch_event_positions_for_sources(
        &self,
        branch_id: &BranchId,
        lineage_ids: &BTreeSet<LineageId>,
    ) -> BTreeSet<usize> {
        self.branch_source_event_positions
            .get(branch_id)
            .into_iter()
            .flat_map(|source_positions| lineage_ids.iter().filter_map(|lineage_id| source_positions.get(lineage_id)))
            .flat_map(|positions| positions.iter().copied())
            .collect()
    }

    pub(crate) fn record_rejected_decision(&mut self, decision: LineageDecisionRecord) {
        self.rejected_decisions.push(decision);
        self.rejected_decisions.sort_by_key(|decision| {
            (
                decision.candidate_id.map(|id| id.0).unwrap_or(u64::MAX),
                format!("{:?}", decision.kind),
                format!("{:?}", decision.rejection_class),
            )
        });
    }

    pub(crate) fn rebuild_branch_event_positions(&mut self) {
        self.branch_event_positions.clear();
        self.branch_source_event_positions.clear();
        for (position, event) in self.events.iter().enumerate() {
            self.branch_event_positions
                .entry(event.branch_id().clone())
                .or_default()
                .push(position);
            let source_positions = self
                .branch_source_event_positions
                .entry(event.branch_id().clone())
                .or_default();
            for source in event.sources() {
                source_positions.entry(*source).or_default().push(position);
            }
        }
    }
}

impl RuntimeSubsystem for LineageSubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::empty()
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
