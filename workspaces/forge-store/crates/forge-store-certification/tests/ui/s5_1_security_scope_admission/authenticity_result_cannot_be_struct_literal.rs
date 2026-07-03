use forge_store_security::{
    StoreAuthenticityResult, StoreAuthenticityResultKind, StoreAuthenticityRequirement,
};

fn main() {
    let _forged = StoreAuthenticityResult {
        kind: StoreAuthenticityResultKind::Verified,
        requirement: StoreAuthenticityRequirement::not_required(),
        scope_identity: todo!(),
        counters: todo!(),
    };
}
