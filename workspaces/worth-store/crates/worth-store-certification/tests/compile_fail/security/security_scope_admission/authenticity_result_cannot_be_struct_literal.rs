use worth_store_security::{
    StoreAuthenticityResult, StoreAuthenticityResultKind, StoreAuthenticityRequirement,
};
use worth_store_physical_format::PhysicalAuthenticityIdentity;

fn main() {
    let _forged = StoreAuthenticityResult::<PhysicalAuthenticityIdentity> {
        kind: StoreAuthenticityResultKind::Verified,
        requirement: StoreAuthenticityRequirement::not_required(),
        scope_identity: todo!(),
        counters: todo!(),
    };
}
