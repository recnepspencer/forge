use super::fixtures::{complete_ledger_for_plan, IndexProductSubject};
use crate::workload_platform::evidence_lookup_index_product::{
    require_persistent_evidence_lookup_index_product, EvidenceLookupIndexLifecyclePostureKind,
    EvidenceLookupIndexProductErrorKind,
};

#[test]
fn persistent_index_claim_requires_admitted_support() {
    let selected_plan = IndexProductSubject::dense_projection_consumption().select_plan();
    let ledger = complete_ledger_for_plan(&selected_plan);

    let error = require_persistent_evidence_lookup_index_product(&selected_plan, &ledger)
        .expect_err("persistent posture must deny without admitted support");
    assert_eq!(
        error.kind(),
        EvidenceLookupIndexProductErrorKind::PersistentCapabilitySupportRequired
    );
    assert_eq!(
        error
            .required_lifecycle_posture()
            .expect("persistent denial should expose required lifecycle posture")
            .kind(),
        EvidenceLookupIndexLifecyclePostureKind::PersistentCapabilityRequired
    );
}
