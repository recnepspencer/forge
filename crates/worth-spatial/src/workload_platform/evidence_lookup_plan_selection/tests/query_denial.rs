use crate::workload_platform::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, EvidenceLookupInputAdmissionErrorKind,
};

use super::fixtures::PlanSelectionSubject;

#[test]
fn missing_query_projection_consumption_denies_before_lookup() {
    let subject = PlanSelectionSubject::projection_consumption();

    let error = admit_evidence_lookup_input(subject.catalog(), subject.request())
        .expect_err("projection consumption needs typed Query import evidence before selection");

    assert_eq!(
        error.kind(),
        EvidenceLookupInputAdmissionErrorKind::MissingQueryImportEvidence
    );
    assert_eq!(error.counters().lookup_product_construction_count(), 0);
    assert_eq!(error.counters().raw_row_scan_count(), 0);
}
