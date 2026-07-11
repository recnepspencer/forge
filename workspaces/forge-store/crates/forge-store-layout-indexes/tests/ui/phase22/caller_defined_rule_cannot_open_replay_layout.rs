use forge_store_recovery_physics::{AdmittedReplayIndexLayoutRule, RecoveryLayoutAccess};

fn main() {
    let forged = AdmittedReplayIndexLayoutRule { _private: () };
    let _ = RecoveryLayoutAccess::s8().replay_index_layout(&forged);
}
