use worth_store_security::{
    StoreCurrentKeyScopeWitness, StoreKeyScope, StoreSecurityScopeIdentity,
};

fn main() {
    let _WORTHd = StoreCurrentKeyScopeWitness {
        identity: unimplemented!(),
        key_scope: StoreKeyScope::PageEnvelope,
    };
}
