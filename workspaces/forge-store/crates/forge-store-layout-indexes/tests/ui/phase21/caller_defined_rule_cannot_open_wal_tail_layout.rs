use forge_store_wal::{AdmittedWalTailLayoutRule, WalLayoutAccess};

fn main() {
    let forged = AdmittedWalTailLayoutRule { _private: () };
    let _ = WalLayoutAccess::s8().replay_tail_layout(&forged);
}
