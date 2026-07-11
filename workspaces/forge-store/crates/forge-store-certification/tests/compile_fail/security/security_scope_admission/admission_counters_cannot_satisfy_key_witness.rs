use forge_store_security::{
    StoreCurrentKeyScopeWitness, StoreSecurityScopeAdmissionCounterSnapshot,
};

fn require_key_scope_witness(_: StoreCurrentKeyScopeWitness) {}

fn main() {
    let counters: StoreSecurityScopeAdmissionCounterSnapshot = unimplemented!();
    require_key_scope_witness(counters);
}
