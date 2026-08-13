use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::BranchId;
use crate::identity::data::LineageId;
use crate::lineage::authority::diagnostic_fields::candidate_recorded_fields;
use crate::lineage::authority::LineageAuthority;
use crate::lineage::data::CorrespondenceCandidate;

impl<'runtime> LineageAuthority<'runtime> {
    pub fn record_correspondence_candidate(
        &mut self,
        branch_id: BranchId,
        sources: Vec<LineageId>,
        targets: Vec<LineageId>,
        note: impl Into<String>,
    ) -> CorrespondenceCandidate {
        let candidate = CorrespondenceCandidate {
            candidate_id: self.next_correspondence_candidate_id(),
            branch_id: branch_id.clone(),
            sources,
            targets,
            note: note.into(),
        };
        self.runtime
            .lineage
            .correspondence_candidates
            .push(candidate.clone());
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Lineage,
                DiagnosticsArtifactKind::MinimalSummary,
                vec![RelationalDiagnosticsEntry::new(
                    DiagnosticCode::LineageCandidateRecorded,
                    "correspondence candidate recorded",
                    candidate_recorded_fields(candidate.candidate_id, &branch_id),
                )],
            );
        candidate
    }
}
