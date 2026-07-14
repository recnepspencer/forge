use super::*;
use crate::courtroom::layout::adjudication::{
    adjudicate_layout_courtroom, assemble_layout_evidence_bundle, LayoutHazardEvidencePosture,
};
use crate::courtroom::layout::owner_coverage::{
    require_exact_owner_family_coverage, LayoutOwnerCaseDeclarations, LayoutOwnerFamily,
};
use crate::courtroom::layout::owner_evidence::certify_layout_owner_execution_evidence;

#[test]
fn every_declared_owner_case_is_observed_through_production_execution() {
    let transcript = execute_declaration_owner_scenarios().unwrap();
    require_exact_owner_family_coverage(
        &LayoutOwnerCaseDeclarations::from_owner_inventories(),
        transcript.observations(),
        LayoutOwnerFamily::all(),
    )
    .unwrap();
}

#[test]
fn exact_owner_evidence_adjudicates_hazards_without_new_authority() {
    let transcript = execute_declaration_owner_scenarios().unwrap();
    let owner_evidence = certify_layout_owner_execution_evidence(transcript).unwrap();
    let bundle = assemble_layout_evidence_bundle(
        owner_evidence,
        crate::courtroom::foundational::handoff_contract_tests::native_boundary_handoff_verdict(),
    )
    .unwrap();
    let report = adjudicate_layout_courtroom(bundle).unwrap();
    let hazards = report.hazards();
    let proof = report.proof_outcomes();
    let formal = report.formal_observation();

    assert_eq!(hazards.rows().len(), 14);
    assert!(hazards.rows().iter().all(|row| {
        row.transcript_identity() == report.transcript_identity()
            && (row.evidence_posture() != LayoutHazardEvidencePosture::ExecutedOwnerEvidence
                || row.residual_risk().is_none())
    }));
    assert_eq!(proof.transcript_identity(), report.transcript_identity());
    assert_eq!(proof.outcomes().len(), 4);
    assert_eq!(formal.transcript_identity(), report.transcript_identity());
    assert_eq!(
        formal.owner_case_count(),
        report.evidence().coverage().owner_case_count()
    );
    assert_eq!(formal.artifacts().len(), 9);
    assert_eq!(formal.orderings().len(), 6);
    assert_eq!(formal.invariants().len(), 7);
}
