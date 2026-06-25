use super::phase_chain_fixture::{production_phase_eight_closeout, production_phase_eight_seed};

#[test]
fn closeout_keeps_receipt_and_posture_proof_visible_for_next_milestone() {
    let seed = production_phase_eight_seed();
    let closeout = production_phase_eight_closeout();

    assert!(closeout
        .receipts()
        .has_executed_receipts_or_visible_postures());
    assert_eq!(
        closeout
            .receipts()
            .has_executed_receipts_or_visible_postures(),
        closeout.receipts().executed_receipt_count() > 0
            || closeout.receipts().visible_non_executed_posture_count() > 0
    );
    assert_eq!(
        closeout.receipts().visible_non_executed_posture_count(),
        closeout.receipts().required_posture_count()
            + closeout.receipts().denied_posture_count()
            + closeout.receipts().carried_gap_count()
    );
    assert_eq!(
        closeout.receipts().report().rows().len(),
        seed.receipt_accounting_report().rows().len()
    );
    assert_eq!(
        closeout.postures().posture_projections().len(),
        seed.posture_projections().len()
    );
    assert_eq!(closeout.postures().cap_rows().len(), seed.cap_rows().len());
}

#[test]
fn closeout_keeps_hard_deletion_proof_readable_without_reopening_execution() {
    let seed = production_phase_eight_seed();
    let closeout = production_phase_eight_closeout();
    let source_deletion_rows = seed.deletion_proof_report().rows();
    let closeout_deletion_rows = closeout.deletion().report().rows();
    let source_residue_rows = seed.capped_residue_report().rows();
    let closeout_residue_rows = closeout.residue().report().rows();
    let source_firewall_rows = seed.source_firewall_report().region_rows();
    let closeout_firewall_rows = closeout.source_firewall().report().region_rows();

    assert_eq!(
        closeout.deletion().report().report_digest(),
        seed.deletion_proof_report().report_digest()
    );
    assert_eq!(closeout_deletion_rows.len(), source_deletion_rows.len());
    for (closeout_row, source_row) in closeout_deletion_rows.iter().zip(source_deletion_rows) {
        assert_eq!(closeout_row.source_path(), source_row.source_path());
        assert_eq!(closeout_row.owner(), source_row.owner());
        assert_eq!(closeout_row.blocker(), source_row.blocker());
        assert_eq!(closeout_row.removal_trigger(), source_row.removal_trigger());
        assert_eq!(closeout_row.status(), source_row.status());
    }
    assert_eq!(
        closeout.residue().report().report_digest(),
        seed.capped_residue_report().report_digest()
    );
    assert_eq!(closeout_residue_rows.len(), source_residue_rows.len());
    for (closeout_row, source_row) in closeout_residue_rows.iter().zip(source_residue_rows) {
        assert_eq!(closeout_row.source_path(), source_row.source_path());
        assert_eq!(closeout_row.owner(), source_row.owner());
        assert_eq!(closeout_row.blocker(), source_row.blocker());
        assert_eq!(closeout_row.removal_trigger(), source_row.removal_trigger());
        assert_eq!(
            closeout_row.observed_residue_count(),
            source_row.observed_residue_count()
        );
        assert_eq!(
            closeout_row.allowed_residue_count(),
            source_row.allowed_residue_count()
        );
    }
    assert_eq!(
        closeout.source_firewall().report().report_digest(),
        seed.source_firewall_report().report_digest()
    );
    assert_eq!(closeout_firewall_rows.len(), source_firewall_rows.len());
    for (closeout_row, source_row) in closeout_firewall_rows.iter().zip(source_firewall_rows) {
        assert_eq!(closeout_row.region(), source_row.region());
        assert_eq!(closeout_row.root_identity(), source_row.root_identity());
        assert_eq!(
            closeout_row.scanned_source_count(),
            source_row.scanned_source_count()
        );
    }
    assert_eq!(closeout.counters().caller_owned_graph_work_count(), 0);
}
