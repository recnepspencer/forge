use worth_store_security::{StoreKeyScope, StoreKmsKeyIdentifier};

fn require_key_scope(_: StoreKeyScope) {}

fn main() {
    require_key_scope(StoreKmsKeyIdentifier::raw("kms-key-123"));
}
