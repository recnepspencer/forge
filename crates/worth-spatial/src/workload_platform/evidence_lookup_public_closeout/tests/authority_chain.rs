use crate::workload_platform::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;
use crate::workload_platform::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutDisposition;

#[test]
fn closeout_digests_bind_lookup_authority_chain() {
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");

    assert_eq!(
        closeout
            .milestone_twelve_seed()
            .milestone_eleven_closeout_digest(),
        closeout.closeout_digest()
    );
    assert_eq!(
        closeout
            .milestone_twelve_seed()
            .query_surface_matrix_digest(),
        closeout.query_surface_matrix().matrix_digest()
    );
    assert_eq!(
        closeout
            .milestone_twelve_seed()
            .query_consumer_kit_closeout_digest(),
        closeout.query_consumer_kit().closeout_digest()
    );
    assert!(!closeout.query_boundary_support_digest().is_empty());
    assert_eq!(
        closeout.milestone_twelve_seed().source_firewall_digest(),
        closeout.source_firewall_report().firewall_digest()
    );
    assert_eq!(
        closeout.milestone_twelve_seed().residue_audit_digest(),
        closeout.residue_audit_digest()
    );
    assert_eq!(
        closeout.milestone_twelve_seed().family_coverage_digest(),
        closeout.family_coverage_digest()
    );
    assert!(!closeout.spatial_compiled_product_family_digest().is_empty());
    assert_eq!(
        closeout.counters().firewall_forbidden_row_count(),
        closeout
            .source_firewall_report()
            .counters()
            .forbidden_row_count()
    );

    for row in closeout.family_stage_rows() {
        assert!(!row.family_declaration_digest().is_empty());
        assert!(!row.stage_receipt_family_identity().is_empty());
        assert!(!row.topology_input_summary().is_empty());

        match row.disposition() {
            EvidenceLookupPublicCloseoutDisposition::ReceiptProof { .. } => {
                assert!(row.spatial_touch_digest().is_some());
                assert!(row.spatial_compiled_product_identity().is_some());
                assert!(row.spatial_compiled_product_identity_digest().is_some());
                assert!(row.spatial_equivalence_policy_identity().is_some());
                assert!(row.spatial_equivalence_policy_identity_digest().is_some());
                assert!(row
                    .spatial_selected_equivalence_family_identity_kind()
                    .is_some());
                if row
                    .topology_input_summary()
                    .contains("DerivedProductReceiptRequired")
                {
                    assert!(row.topology_query_backed_cutover_digest().is_some());
                    assert!(row.topology_read_family_row_digest().is_some());
                }
            }
            EvidenceLookupPublicCloseoutDisposition::NonOrdinaryResidue { .. } => {
                panic!("phase 13 public closeout should not carry topology-seed residue on the ordinary path");
            }
        }
    }

    let projection_row = closeout
        .family_stage_rows()
        .iter()
        .find(|row| {
            row.family_identity() == "spatial-touch.boolean.projection-consumption-evidence.v1"
        })
        .expect("projection family row");
    assert!(projection_row.query_import_evidence_digest().is_some());
}
