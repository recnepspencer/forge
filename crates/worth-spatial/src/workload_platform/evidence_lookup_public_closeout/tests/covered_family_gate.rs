use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;
use crate::workload_platform::evidence_lookup_public_closeout::{
    current_evidence_lookup_public_closeout, EvidenceLookupPublicCloseoutDisposition,
};

#[test]
fn milestone_eleven_closeout_requires_all_covered_lookup_families() {
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");
    let family_catalog = current_evidence_lookup_family_catalog().expect("family catalog");

    assert_eq!(closeout.counters().family_stage_row_count(), 6);
    assert_eq!(closeout.counters().receipt_proof_row_count(), 6);
    assert_eq!(closeout.counters().non_ordinary_residue_row_count(), 0);

    for row in closeout.family_stage_rows() {
        let family = family_catalog
            .family_by_identity(row.family_identity())
            .expect("declared family coverage row");
        assert_eq!(row.family_declaration_digest(), family.declaration_digest());
        assert_eq!(
            row.stage_receipt_family_identity(),
            family
                .stage_applicability()
                .stage_receipt_family_identity()
                .digest()
        );
        match row.disposition() {
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof {
                selected_lookup_plan_digest,
                lookup_execution_receipt_digest,
                lookup_product_output_digest,
            } => {
                assert!(!selected_lookup_plan_digest.is_empty());
                assert!(!lookup_execution_receipt_digest.is_empty());
                assert!(!lookup_product_output_digest.is_empty());
                if family.topology_input_posture().requires_topology_receipt() {
                    assert!(row.topology_query_backed_cutover_digest().is_some());
                    assert!(row.topology_read_family_row_digest().is_some());
                }
            }
            EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => {
                panic!("phase 13 topology-required public-closeout families must no longer publish residue rows");
            }
        }
    }
}
