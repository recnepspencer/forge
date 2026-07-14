use worth_query::facade::runtime::{WorthQueryLowerRuntimeCapabilityEligibility, WorthQueryLowerRuntimeRoutePlan};

fn main() {
    let eligibility: WorthQueryLowerRuntimeCapabilityEligibility = todo!();
    let _ = WorthQueryLowerRuntimeRoutePlan::new(eligibility, "worthd-route");
}
