use worth_store_security::{
    StoreCurrentKeyScopeWitness, StoreSecurityScopeAdmissionReceiptId,
};

fn require_key_scope_witness(_: StoreCurrentKeyScopeWitness) {}

fn main() {
    let receipt_id: StoreSecurityScopeAdmissionReceiptId = unimplemented!();
    require_key_scope_witness(receipt_id);
}
