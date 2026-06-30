use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout, EvidenceLookupPublicCloseoutDisposition,
};

#[test]
fn milestone_eleven_closeout_requires_all_covered_lookup_families() {
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");

    assert_eq!(closeout.counters().family_stage_row_count(), 6);
    assert_eq!(closeout.counters().receipt_proof_row_count(), 4);
    assert_eq!(closeout.counters().non_ordinary_residue_row_count(), 2);

    for row in closeout.family_stage_rows() {
        match row.disposition() {
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
                selected_lookup_plan_digest,
                lookup_execution_receipt_digest,
                lookup_product_output_digest,
            } => {
                assert!(!selected_lookup_plan_digest.is_empty());
                assert!(!lookup_execution_receipt_digest.is_empty());
                assert!(!lookup_product_output_digest.is_empty());
            }
            EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue {
                reason,
                removal_trigger,
            } => {
                assert!(matches!(
                    row.stage(),
                    WorkloadEvidenceStage::BooleanSharedPlaneIdentity
                        | WorkloadEvidenceStage::BooleanLocalFrameSelection
                ));
                assert!(reason.contains("non-ordinary"));
                assert!(!removal_trigger.is_empty());
            }
        }
    }
}
