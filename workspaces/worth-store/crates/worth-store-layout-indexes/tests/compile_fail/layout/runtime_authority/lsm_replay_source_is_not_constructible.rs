use worth_store_lsm_authority::{AdmittedLsmReplaySource, LsmReplaySourceKind};

fn worth() -> AdmittedLsmReplaySource {
    AdmittedLsmReplaySource {
        membership: panic!(),
        selected_source: LsmReplaySourceKind::WalFrame,
        selected_first_lsn: 1,
        selected_last_lsn: 2,
    }
}

fn main() {
    let _ = worth();
}
