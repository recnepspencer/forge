use worth_store_security::{StoreKeyScope, StoreKeyVersionPosture};

fn require_key_scope(_: StoreKeyScope) {}

fn main() {
    require_key_scope(StoreKeyVersionPosture::Current);
}
