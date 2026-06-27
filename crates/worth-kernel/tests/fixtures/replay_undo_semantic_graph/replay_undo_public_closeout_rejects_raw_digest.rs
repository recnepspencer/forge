use worth_kernel::replay_undo_consumer_cutover::ReplayUndoMilestoneTwelvePublicCloseoutInput;

fn raw_digest_cannot_enter_public_closeout_input() {
    let digest = String::from("copied-public-closeout-digest");
    let _ = ReplayUndoMilestoneTwelvePublicCloseoutInput::from_parts(&digest, &digest, &digest);
}

fn main() {}
