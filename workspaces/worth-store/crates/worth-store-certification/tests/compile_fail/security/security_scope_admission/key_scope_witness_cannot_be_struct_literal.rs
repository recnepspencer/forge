use worth_store_security::{
    StoreCurrentKeyScopeWitness, StoreKeyScope, StoreSecurityScopeIdentity,
};

fn main() {
    let _forged = StoreCurrentKeyScopeWitness {
        identity: unimplemented!(),
        key_scope: StoreKeyScope::PageEnvelope,
    };
}
