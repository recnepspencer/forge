use crate::workload_platform::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupMilestoneTwelveReplayReadinessPosture;

#[test]
fn milestone_twelve_seed_carries_lookup_scope_without_evidence_rescan() {
    let closeout = current_evidence_lookup_public_closeout().expect("public closeout");
    let seed = closeout.milestone_twelve_seed();

    assert_eq!(
        seed.replay_readiness_posture(),
        EvidenceLookupMilestoneTwelveReplayReadinessPosture::LookupScopeBoundedNoReplay
    );
    assert_ne!(seed.selected_lookup_plan_digest(), "no-covered-plan");
    assert_ne!(seed.lookup_execution_receipt_digest(), "no-covered-receipt");
    assert_ne!(seed.lookup_product_output_digest(), "no-covered-output");
    assert_eq!(
        seed.family_stage_row_count(),
        closeout.counters().family_stage_row_count()
    );
    assert_eq!(
        seed.receipt_proof_row_count(),
        closeout.counters().receipt_proof_row_count()
    );
    assert_eq!(
        seed.non_ordinary_residue_row_count(),
        closeout.counters().non_ordinary_residue_row_count()
    );
    assert_eq!(
        seed.query_imported_family_count(),
        closeout
            .family_stage_rows()
            .iter()
            .filter(|row| row.query_import_evidence_digest().is_some())
            .count()
    );
    assert_eq!(
        seed.topology_required_family_count(),
        closeout
            .family_stage_rows()
            .iter()
            .filter(|row| row
                .topology_input_summary()
                .contains("DerivedProductReceiptRequired"))
            .count()
    );
    assert_eq!(
        seed.covered_family_identities().len(),
        closeout.family_stage_rows().len()
    );
}
