use forge_store_security::{
    StoreAuthenticityRequirement, StoreCurrentAuthenticityScopeWitness,
    StoreSecurityScopeIdentity,
};

fn main() {
    let _forged = StoreCurrentAuthenticityScopeWitness {
        identity: unimplemented!(),
        requirement: StoreAuthenticityRequirement::not_required(),
    };
}
