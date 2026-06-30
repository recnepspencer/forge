use schema::facade::platform::authority::touched_graph_conflict::ConflictOverlapCategory;

use crate::workload_platform::evidence_ledger::receipt_backed_touch_authority_for_admission_tests;
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;

#[test]
fn declaration_derives_shared_evidence_routing_contract_from_spatial_touch_authority() {
    let catalog = current_evidence_lookup_family_catalog().expect("family catalog closes");
    let family = catalog
        .family_by_identity("spatial-touch.boolean.overlap-evidence.v1")
        .expect("shared plane family exists");
    let authority = receipt_backed_touch_authority_for_admission_tests(
        crate::workload_platform::evidence_ledger::BooleanEvidenceStageKind::SharedPlaneIdentity,
        "phase-2-shared-contract-evidence",
    );

    let contract = family
        .conflict_routing_contract(&authority)
        .expect("shared routing contract admits");
    let authority_participant = authority
        .conflict_participant_identity()
        .expect("authority participant admits");
    let family_participant = family
        .conflict_participant_identity()
        .expect("family participant admits");

    assert_eq!(
        contract.overlap_identity().category(),
        ConflictOverlapCategory::Evidence
    );
    assert_eq!(
        contract
            .overlap_identity()
            .locality_identity()
            .expect("evidence overlap carries locality")
            .authority_digest(),
        authority.digest().as_str()
    );
    assert_eq!(contract.overlap_identity().participants().len(), 2);
    assert_eq!(
        authority_participant.authority(),
        family_participant.authority()
    );
}
