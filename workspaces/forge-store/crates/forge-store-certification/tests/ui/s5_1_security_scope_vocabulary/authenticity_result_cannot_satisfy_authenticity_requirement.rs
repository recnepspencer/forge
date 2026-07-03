use forge_store_security::{StoreAuthenticityRequirement, StoreAuthenticityResult};

fn require_authenticity_requirement(_: StoreAuthenticityRequirement) {}

fn main() {
    let result: StoreAuthenticityResult = todo!();
    require_authenticity_requirement(result);
}
