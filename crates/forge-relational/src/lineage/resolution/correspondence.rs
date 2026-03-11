use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::{BranchId, CommitReference};
use crate::identity::data::{EntityId, LineageId};
use crate::lineage::data::{
    CorrespondenceCandidate, CorrespondenceResolution, HistoricalLineageResolution,
    LineageEventKind, LineageEventRecord, LineageResolutionStatus,
};
use crate::logic::runtime::RelationalRuntime;
use serde_json::json;

impl RelationalRuntime {
    pub fn resolve_historical_lineage(
        &self,
        branch_id: &BranchId,
        lineage_id: LineageId,
    ) -> HistoricalLineageResolution {
        let mut current = vec![lineage_id];
        let mut traversed_event_ids = Vec::new();

        for event in self
            .lineage
            .events
            .iter()
            .filter(|event| &event.branch_id == branch_id)
        {
            if !event.sources.iter().any(|source| current.contains(source)) {
                continue;
            }
            match event.kind {
                LineageEventKind::Replace
                | LineageEventKind::Split
                | LineageEventKind::Merge
                | LineageEventKind::Correspond => {
                    traversed_event_ids.push(event.event_id);
                    current.retain(|candidate| !event.sources.contains(candidate));
                    current.extend(event.targets.iter().copied());
                    current.sort();
                    current.dedup();
                }
                LineageEventKind::Create | LineageEventKind::Retire => {}
            }
        }

        HistoricalLineageResolution {
            branch_id: branch_id.clone(),
            start: lineage_id,
            resolved: current,
            traversed_event_ids,
        }
    }

    pub fn resolve_record_history(
        &self,
        branch_id: &BranchId,
        entity_id: EntityId,
    ) -> Option<HistoricalLineageResolution> {
        let lineage = self.lineage_for_record(entity_id)?;
        Some(self.resolve_historical_lineage(branch_id, lineage.lineage_id))
    }

    pub fn record_correspondence_candidate(
        &mut self,
        branch_id: BranchId,
        sources: Vec<LineageId>,
        targets: Vec<LineageId>,
        note: impl Into<String>,
    ) -> CorrespondenceCandidate {
        let candidate = CorrespondenceCandidate {
            candidate_id: self.lineage.next_event_id,
            branch_id: branch_id.clone(),
            sources,
            targets,
            note: note.into(),
        };
        self.lineage.next_event_id += 1;
        self.lineage
            .correspondence_candidates
            .push(candidate.clone());
        self.push_bounded_diagnostic(
            DiagnosticsScope::Lineage,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "correspondence candidate recorded".to_string(),
                fields: json!({
                    "candidate_id": candidate.candidate_id,
                    "branch_id": branch_id.0,
                }),
            }],
        );
        candidate
    }

    pub fn promote_correspondence(
        &mut self,
        candidate_id: u64,
        commit: CommitReference,
    ) -> Option<CorrespondenceResolution> {
        let candidate = self
            .lineage
            .correspondence_candidates
            .iter()
            .find(|candidate| candidate.candidate_id == candidate_id)?
            .clone();
        if candidate
            .sources
            .iter()
            .chain(candidate.targets.iter())
            .any(|lineage_id| !self.lineage.nodes.contains_key(lineage_id))
        {
            self.push_bounded_diagnostic(
                DiagnosticsScope::Lineage,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry {
                    code: DiagnosticCode::InvariantViolation,
                    message: "correspondence promotion referenced missing lineage".to_string(),
                    fields: json!({ "candidate_id": candidate_id }),
                }],
            );
            return None;
        }
        let event_id = self.lineage.next_event_id;
        self.lineage.next_event_id += 1;
        let event = LineageEventRecord {
            event_id,
            commit: commit.clone(),
            branch_id: candidate.branch_id.clone(),
            kind: LineageEventKind::Correspond,
            sources: candidate.sources.clone(),
            targets: candidate.targets.clone(),
        };
        self.lineage.events.push(event);
        self.attach_lineage_events_to_commit(commit.commit_id, &[event_id]);
        let resolution = CorrespondenceResolution {
            candidate_id,
            status: LineageResolutionStatus::Promoted,
            promoted_event_id: Some(event_id),
        };
        self.push_bounded_diagnostic(
            DiagnosticsScope::Lineage,
            DiagnosticsArtifactKind::MinimalSummary,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "correspondence promoted into lineage".to_string(),
                fields: json!({
                    "candidate_id": candidate_id,
                    "event_id": event_id,
                    "commit_id": commit.commit_id.0,
                }),
            }],
        );
        Some(resolution)
    }
}
