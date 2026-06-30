use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::super::{admit_evidence_lookup_input, EvidenceLookupQueryAdmissionEvidenceSet};
use super::fixtures::{query_import_for_stage, AdmissionSubject};

#[test]
fn admission_digest_binds_query_support_posture() {
    let subject = AdmissionSubject::projection_consumption();
    let query_import = query_import_for_stage(
        subject.catalog(),
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
    );

    let admitted = admit_evidence_lookup_input(
        subject.catalog(),
        subject.request_with_query_evidence(
            EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(&query_import),
        ),
    )
    .expect("query-backed admission succeeds");
    let repeated = admit_evidence_lookup_input(
        subject.catalog(),
        subject.request_with_query_evidence(
            EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(&query_import),
        ),
    )
    .expect("query-backed admission is stable");
    let event_ledger_admitted = AdmissionSubject::event_ledger().admit();

    assert_eq!(admitted.admission_digest(), repeated.admission_digest());
    assert_ne!(
        admitted.admission_digest(),
        event_ledger_admitted.admission_digest()
    );
}
