use crate::history::data::RelationalCommitReceipt;
use crate::identity::data::LineageId;
use crate::lineage::authority::LineageAuthority;
#[cfg(test)]
use crate::lineage::data::PublishedLineageArtifact;
use crate::lineage::data::{
    CorrespondenceCandidateId, LineageDecisionKind, LineageDecisionRecord, LineageEventKind,
    LineageEventRecord,
};

impl<'runtime> LineageAuthority<'runtime> {
    pub(super) fn next_lineage_event_id(&mut self) -> u64 {
        let event_id = self.runtime.lineage.next_event_id;
        self.runtime.lineage.next_event_id += 1;
        event_id
    }

    #[cfg(test)]
    pub(super) fn next_correspondence_candidate_id(&mut self) -> CorrespondenceCandidateId {
        let candidate_id = CorrespondenceCandidateId(self.runtime.lineage.next_candidate_id);
        self.runtime.lineage.next_candidate_id += 1;
        candidate_id
    }

    pub(super) fn emit_authoritative_lineage_event(
        &mut self,
        commit: &RelationalCommitReceipt,
        kind: LineageEventKind,
        sources: Vec<LineageId>,
        targets: Vec<LineageId>,
    ) -> LineageEventRecord {
        let event = self.prepare_authoritative_lineage_event(commit, kind, sources, targets);
        self.runtime.lineage.record_event(event.clone());
        event
    }

    pub(super) fn prepare_authoritative_lineage_event(
        &mut self,
        commit: &RelationalCommitReceipt,
        kind: LineageEventKind,
        sources: Vec<LineageId>,
        targets: Vec<LineageId>,
    ) -> LineageEventRecord {
        LineageEventRecord {
            event_id: self.next_lineage_event_id(),
            commit: commit.clone(),
            branch_id: commit.branch_id.clone(),
            kind,
            sources,
            targets,
        }
    }

    #[cfg(test)]
    pub(super) fn record_published_lineage_events(&mut self, artifact: &PublishedLineageArtifact) {
        let existing_event_ids = self
            .runtime
            .lineage
            .events
            .iter()
            .map(|candidate| candidate.event_id())
            .collect::<std::collections::BTreeSet<u64>>();
        for event in artifact.lineage_events() {
            if existing_event_ids.contains(&event.event_id()) {
                continue;
            }
            self.runtime.lineage.record_event(event.clone());
        }
    }

    pub(super) fn accepted_decision_record(
        &self,
        kind: LineageDecisionKind,
        event: &LineageEventRecord,
        candidate_id: Option<CorrespondenceCandidateId>,
    ) -> LineageDecisionRecord {
        LineageDecisionRecord {
            branch_id: event.branch_id().clone(),
            kind,
            event_id: Some(event.event_id()),
            candidate_id,
            sources: event.sources().to_vec(),
            targets: event.targets().to_vec(),
            rejection_class: None,
        }
    }
}
