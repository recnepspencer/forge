use worth_kernel::replay_undo_consumer_cutover::ReplayUndoHardDeletionCloseout;

fn hard_deletion_closeout_is_not_hand_filled() {
    let _ = ReplayUndoHardDeletionCloseout {
        deletion_ledger: todo!(),
        residue_cap_audit: todo!(),
        source_firewall: todo!(),
        counters: todo!(),
        milestone_thirteen_seed: todo!(),
    };
}

fn main() {}
