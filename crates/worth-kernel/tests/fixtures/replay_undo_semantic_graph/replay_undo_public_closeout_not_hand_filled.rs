use worth_kernel::replay_undo_consumer_cutover::ReplayUndoMilestoneTwelvePublicCloseout;

fn public_closeout_is_not_hand_filled() {
    let _ = ReplayUndoMilestoneTwelvePublicCloseout {
        closeout_identity: String::new(),
        transaction_packet_identity: String::new(),
        replay_scope_identity: String::new(),
        undo_scope_identity: String::new(),
        inventory_rows: Vec::new(),
        counters: todo!(),
        milestone_thirteen_seed: todo!(),
    };
}

fn main() {}
