use forge_store_wal::{AdmittedWalAppendLayoutRule, WalLayoutAccess};

fn main() {
    let forged = AdmittedWalAppendLayoutRule { _private: () };
    let _ = WalLayoutAccess::s8().durable_mutation_layout(&forged);
}
