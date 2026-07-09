use worth_store::{AuthoritativeCompatibilityWitness, TierManifestCompatibilityPlan};

fn main() {
    require_authority(tier_plan());
}

fn require_authority(_: AuthoritativeCompatibilityWitness) {}

fn tier_plan() -> TierManifestCompatibilityPlan {
    panic!("compile-fail fixture")
}
