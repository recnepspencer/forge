use worth_store_security::{
    StoreAuthenticityResult, StoreAuthenticityResultKind, StoreAuthenticityRequirement,
};

fn main() {
    let _WORTHd = StoreAuthenticityResult {
        kind: StoreAuthenticityResultKind::Verified,
        requirement: StoreAuthenticityRequirement::not_required(),
        scope_identity: todo!(),
        counters: todo!(),
    };
}
