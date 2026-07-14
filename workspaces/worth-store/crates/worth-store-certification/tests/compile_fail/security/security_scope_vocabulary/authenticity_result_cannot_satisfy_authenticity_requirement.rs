use worth_store_physical_format::PhysicalAuthenticityIdentity;
use worth_store_security::{StoreAuthenticityRequirement, StoreAuthenticityResult};

fn require_authenticity_requirement(_: StoreAuthenticityRequirement) {}

fn main() {
    let result: StoreAuthenticityResult<PhysicalAuthenticityIdentity> = todo!();
    require_authenticity_requirement(result);
}
