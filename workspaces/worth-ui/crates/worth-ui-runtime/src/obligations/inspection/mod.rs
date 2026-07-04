mod admission_evidence_records;
mod dispatch_evidence_records;
mod evidence_handle;
mod evidence_authority_source;
mod evidence_index;
mod evidence_query;
mod evidence_record;
mod projection_mapping;
mod selected_evidence_projection;
mod selection_reason_mapping;
mod selection_evidence_records;
mod verdict_evidence_records;

pub use evidence_handle::{UiObligationEvidenceHandle, UiObligationEvidenceHandleKind};
pub use evidence_index::UiObligationEvidenceIndex;
pub(crate) use evidence_authority_source::UiObligationEvidenceAuthoritySource;
pub use evidence_query::UiObligationEvidenceQuery;
pub use evidence_record::{
    UiObligationEvidenceDecision, UiObligationEvidenceDenialPosture,
    UiObligationEvidenceDispatchPosture,
    UiObligationEvidencePrerequisiteSource, UiObligationEvidenceRecord,
    UiObligationEvidenceVerdictPosture, UiObligationLegalityReasonEvidence,
    UiObligationNonSelectionReason,
};
pub use selected_evidence_projection::UiSelectedObligationEvidenceProjection;
pub(crate) use admission_evidence_records::admitted_report_evidence_records;
pub(crate) use dispatch_evidence_records::dispatch_evidence_records;
pub(crate) use selection_evidence_records::{
    not_selected_obligation_evidence_record, prerequisite_sources_from_refs,
    prerequisite_sources_from_target, query_prerequisite_evidence_from_refs,
    query_prerequisite_evidence_from_target,
    selected_obligation_evidence_records,
};
pub(crate) use verdict_evidence_records::verdict_evidence_records;
