use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeCapabilityEligibility, ForgeQueryLowerRuntimeRoutePlan,
};

fn main() {
    let eligibility: ForgeQueryLowerRuntimeCapabilityEligibility = todo!();
    let _ = ForgeQueryLowerRuntimeRoutePlan::new(eligibility, "forged-route");
}
