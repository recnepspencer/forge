use worth_kernel::replay_undo_consumer_cutover::{
    ReplayUndoMilestoneThirteenSeedPosture, ReplayUndoPublicCloseoutClassification,
    ReplayUndoPublicCloseoutInventoryRow,
};
use worth_kernel::replay_undo_inventory::{
    current_replay_undo_inventory_report, ReplayUndoInventoryDisposition,
    ReplayUndoInventoryReportRow,
};
use worth_kernel::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket;
use worth_kernel::workload_composition::BooleanChainReplayUndoBoundaryHandoff;

pub(crate) fn assert_hard_deletion_closeout_binds_ordinary_chain(
    chain: &BooleanChainReplayUndoBoundaryHandoff,
    packet: &ReplayUndoTransactionBoundaryPacket,
) {
    let closeout = chain.hard_deletion_closeout();
    let source_firewall = closeout.source_firewall();
    let deletion_ledger = closeout.deletion_ledger();
    let residue_cap_audit = closeout.residue_cap_audit();
    let counters = closeout.counters();

    assert_eq!(source_firewall.violation_count(), 0);
    assert!(
        source_firewall.scanned_source_count() > 1_000,
        "hard deletion firewall must cover the phase production source scope"
    );
    assert_eq!(
        counters.scanned_source_count(),
        source_firewall.scanned_source_count()
    );
    assert_eq!(counters.source_firewall_violation_count(), 0);
    assert_eq!(counters.deletion_row_count(), deletion_ledger.row_count());
    assert_eq!(
        counters.residue_cap_row_count(),
        residue_cap_audit.row_count()
    );
    assert_eq!(closeout.uncapped_residue_count(), 0);
    assert!(deletion_ledger
        .rows()
        .iter()
        .all(|row| !row.removal_trigger().is_empty()));
    assert!(residue_cap_audit
        .rows()
        .iter()
        .all(|row| { row.observed_count() <= row.cap() && !row.removal_trigger().is_empty() }));

    let seed = closeout.milestone_thirteen_seed();
    assert_eq!(seed.transaction_packet_identity(), packet.packet_identity());
    assert_eq!(
        seed.replay_scope_identity(),
        packet.replay_scope_identity().digest()
    );
    assert_eq!(
        seed.undo_scope_identity(),
        packet.undo_scope_identity().digest()
    );
    assert_eq!(
        seed.hard_deletion_ledger_digest(),
        Some(deletion_ledger.ledger_digest())
    );
    assert_eq!(
        seed.residue_cap_audit_digest(),
        Some(residue_cap_audit.audit_digest())
    );
    assert_eq!(
        seed.hard_deletion_source_firewall_digest(),
        Some(source_firewall.report_digest())
    );
    assert_eq!(
        seed.posture(),
        ReplayUndoMilestoneThirteenSeedPosture::ReplayUndoOnlyNoConflictOrCacheClaim
    );

    let public_closeout = chain.public_closeout();
    assert_eq!(
        public_closeout.transaction_packet_identity(),
        packet.packet_identity()
    );
    assert_eq!(
        public_closeout.replay_scope_identity(),
        packet.replay_scope_identity().digest()
    );
    assert_eq!(
        public_closeout.undo_scope_identity(),
        packet.undo_scope_identity().digest()
    );
    assert_eq!(
        public_closeout.milestone_thirteen_seed(),
        closeout.milestone_thirteen_seed()
    );
    assert_eq!(
        public_closeout
            .milestone_thirteen_seed()
            .hard_deletion_ledger_digest(),
        Some(deletion_ledger.ledger_digest())
    );
    assert_eq!(
        public_closeout
            .milestone_thirteen_seed()
            .residue_cap_audit_digest(),
        Some(residue_cap_audit.audit_digest())
    );
    assert_eq!(
        public_closeout
            .milestone_thirteen_seed()
            .hard_deletion_source_firewall_digest(),
        Some(source_firewall.report_digest())
    );
    assert_eq!(
        public_closeout.counters().deletion_row_count(),
        deletion_ledger.row_count()
    );
    assert_eq!(
        public_closeout.counters().residue_cap_row_count(),
        residue_cap_audit.row_count()
    );
    assert_eq!(
        public_closeout
            .counters()
            .hard_deletion_firewall_row_count(),
        0
    );
    assert_public_closeout_classifies_current_inventory(public_closeout.inventory_rows());
}

fn assert_public_closeout_classifies_current_inventory(
    public_rows: &[ReplayUndoPublicCloseoutInventoryRow],
) {
    let inventory = current_replay_undo_inventory_report().expect("replay/undo inventory report");
    assert_eq!(
        public_rows.len(),
        inventory.rows().len(),
        "public closeout must classify every current replay/undo inventory row"
    );

    for source_row in inventory.rows() {
        let public_row = public_rows
            .iter()
            .find(|row| row.source_identity() == source_row.source_identity())
            .unwrap_or_else(|| {
                panic!(
                    "public closeout omitted inventory source `{}`",
                    source_row.source_identity().as_str()
                )
            });
        assert_public_row_matches_source_inventory(public_row, source_row);
    }
}

fn assert_public_row_matches_source_inventory(
    public_row: &ReplayUndoPublicCloseoutInventoryRow,
    source_row: &ReplayUndoInventoryReportRow,
) {
    assert_eq!(public_row.source_kind(), source_row.source_kind());
    assert_eq!(public_row.owner(), source_row.owner());
    assert_eq!(
        public_row.classification(),
        expected_public_closeout_classification(source_row.disposition())
    );
    assert_eq!(public_row.residue_cap(), source_row.residue_cap());
    assert_eq!(
        public_row.observed_residue_count(),
        source_row.observed_residue_count()
    );
    assert_eq!(public_row.removal_trigger(), source_row.removal_trigger());
    match public_row.classification() {
        ReplayUndoPublicCloseoutClassification::Migrated => {}
        ReplayUndoPublicCloseoutClassification::Deleted => {
            assert!(public_row.removal_trigger().is_some());
        }
        ReplayUndoPublicCloseoutClassification::Capped
        | ReplayUndoPublicCloseoutClassification::QueryGap => {
            assert!(public_row.removal_trigger().is_some());
            assert!(public_row.residue_cap().is_some());
        }
    }
}

fn expected_public_closeout_classification(
    disposition: ReplayUndoInventoryDisposition,
) -> ReplayUndoPublicCloseoutClassification {
    match disposition {
        ReplayUndoInventoryDisposition::Migrate => ReplayUndoPublicCloseoutClassification::Migrated,
        ReplayUndoInventoryDisposition::Delete => ReplayUndoPublicCloseoutClassification::Deleted,
        ReplayUndoInventoryDisposition::Cap => ReplayUndoPublicCloseoutClassification::Capped,
        ReplayUndoInventoryDisposition::QueryGap => {
            ReplayUndoPublicCloseoutClassification::QueryGap
        }
    }
}
