use super::super::{admit_evidence_lookup_input, EvidenceLookupInputAdmissionErrorKind};
use super::fixtures::AdmissionSubject;

#[test]
fn topology_receipt_narrows_but_cannot_satisfy_lookup() {
    let subject = AdmissionSubject::topology_required_shared_plane();

    let denial = admit_evidence_lookup_input(subject.catalog(), subject.request())
        .expect_err("topology-required family cannot admit without topology seed");

    assert_eq!(
        denial.kind(),
        EvidenceLookupInputAdmissionErrorKind::MissingTopologySeed
    );
    assert_eq!(denial.counters().topology_required_count(), 1);
    assert_eq!(denial.counters().raw_row_scan_count(), 0);
}
