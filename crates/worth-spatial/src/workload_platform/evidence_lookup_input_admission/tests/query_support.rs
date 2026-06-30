use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupProjectionConsumptionSurface, EvidenceLookupProjectionFactFamily,
    EvidenceLookupQueryImportEvidence,
};

use super::super::{
    admit_evidence_lookup_input, real_projection_consumption_receipt,
    EvidenceLookupInputAdmissionErrorKind, EvidenceLookupQueryAdmissionEvidenceSet,
    EvidenceLookupQuerySupportState, EvidenceLookupTopologySupportState,
};
use super::fixtures::{query_import_for_stage, AdmissionSubject};

#[test]
fn query_descriptor_and_lookup_product_are_not_interchangeable() {
    let subject = AdmissionSubject::projection_consumption();

    let denial = admit_evidence_lookup_input(subject.catalog(), subject.request())
        .expect_err("query-required family cannot admit without typed query import posture");

    assert_eq!(
        denial.kind(),
        EvidenceLookupInputAdmissionErrorKind::MissingQueryImportEvidence
    );
    assert_eq!(denial.counters().query_required_count(), 1);
    assert_eq!(denial.counters().raw_row_scan_count(), 0);

    let query_import = query_import_for_stage(
        subject.catalog(),
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
    );
    let EvidenceLookupQueryImportEvidence::ProjectionConsumptionReceipt { fact_family, .. } =
        query_import
    else {
        panic!("fixture should require a projection consumption receipt");
    };
    let admitted = admit_evidence_lookup_input(
        subject.catalog(),
        subject.request_with_query_evidence(
            EvidenceLookupQueryAdmissionEvidenceSet::from_projection_consumption_receipt(
                &real_projection_consumption_receipt(),
                fact_family,
            ),
        ),
    )
    .expect("typed query import evidence admits");

    assert!(matches!(
        admitted.query_support()[0].state(),
        EvidenceLookupQuerySupportState::Satisfied { .. }
    ));
    assert!(matches!(
        admitted.topology_support()[0].state(),
        EvidenceLookupTopologySupportState::NotRequired
    ));
    assert_eq!(
        admitted.stage_receipt_digest(),
        subject.authority().evidence_identity()
    );
    assert!(!admitted
        .product_separation()
        .claims_query_descriptor_authority());
}

#[test]
fn wrong_query_import_evidence_denies_before_lookup_construction() {
    let subject = AdmissionSubject::projection_consumption();
    let wrong_query_import = EvidenceLookupQueryImportEvidence::ProjectionConsumptionReceipt {
        surface: EvidenceLookupProjectionConsumptionSurface::ForgeQueryProjectionConsumptionReceipt,
        fact_family: EvidenceLookupProjectionFactFamily::SpatialTouchOperandProjection,
        requirement_digest: "wrong-query-import-digest".to_string(),
    };

    let denial = admit_evidence_lookup_input(
        subject.catalog(),
        subject.request_with_query_evidence(
            EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(
                &wrong_query_import,
            ),
        ),
    )
    .expect_err("wrong query evidence must not satisfy admission");

    assert_eq!(
        denial.kind(),
        EvidenceLookupInputAdmissionErrorKind::MissingQueryImportEvidence
    );
    assert_eq!(denial.counters().query_required_count(), 1);
    assert_eq!(denial.counters().lookup_product_construction_count(), 0);
}
