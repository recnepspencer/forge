use crate::facade::retained_obligation_registry::WorthUiRetainedObligationRegistry;
use crate::obligations::inspection::UiObligationEvidenceRecord;
use worth_ui_inspection::{
    UiInspectionForeignEvidenceCitation, UiInspectionForeignEvidenceRef,
    UiInspectionQueryForeignEvidenceCitation,
};

pub(crate) fn cite_foreign_evidence(
    _registry: &WorthUiRetainedObligationRegistry,
    foreign_ref: UiInspectionForeignEvidenceRef,
) -> UiInspectionForeignEvidenceCitation {
    match foreign_ref {
        UiInspectionForeignEvidenceRef::Query(query_ref) => {
            UiInspectionForeignEvidenceCitation::Query(
                UiInspectionQueryForeignEvidenceCitation::new(query_ref, false),
            )
        }
    }
}

pub(crate) fn foreign_evidence_refs_for_obligation_record(
    record: &UiObligationEvidenceRecord,
) -> Box<[UiInspectionForeignEvidenceRef]> {
    let _ = record;
    Box::new([])
}
