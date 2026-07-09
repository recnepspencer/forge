use worth_store::{RetainedAuthorityCompatibilityWitness, TierManifestCompatibilityPlan};

fn main() {
    require_retained_authority(tier_plan());
}

fn require_retained_authority(_: RetainedAuthorityCompatibilityWitness) {}

fn tier_plan() -> TierManifestCompatibilityPlan {
    panic!("compile-fail fixture")
}
