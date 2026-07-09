use worth_store_security::{
    StoreAuthenticityRequirement, StoreCurrentAuthenticityScopeWitness,
    StoreSecurityScopeIdentity,
};

fn main() {
    let _WORTHd = StoreCurrentAuthenticityScopeWitness {
        identity: unimplemented!(),
        requirement: StoreAuthenticityRequirement::not_required(),
    };
}
