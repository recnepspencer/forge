use super::{record_layout_observation, LayoutOwnerObservationLedger};
use crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceKind as Evidence;

impl LayoutOwnerObservationLedger {
    pub fn record_corruption_classification(
        &mut self,
        observed: worth_store_layout_indexes::OwnerCaseObservation<
            worth_store_layout_indexes::integrity::CorruptionClassificationCaseId,
        >,
    ) {
        let case = observed.case_id();
        if case.as_str() == "layout.integrity.classification.quarantined" {
            self.record_executed_evidence(Evidence::CorruptionQuarantined);
        }
        self.record(
            super::LayoutOwnerFamily::CorruptionClassification,
            case.as_str(),
        );
    }
    record_layout_observation!(
        record_quarantine_readmission,
        QuarantineReadmission,
        worth_store_layout_indexes::integrity::QuarantineReadmissionCaseId,
        as_str
    );
    record_layout_observation!(
        record_offline_readmission,
        OfflineReadmission,
        worth_store_layout_indexes::integrity::OfflineReadmissionCaseId,
        as_str
    );
    record_layout_observation!(
        record_import_readmission,
        ImportReadmission,
        worth_store_layout_indexes::integrity::ImportReadmissionCaseId,
        as_str
    );
}
