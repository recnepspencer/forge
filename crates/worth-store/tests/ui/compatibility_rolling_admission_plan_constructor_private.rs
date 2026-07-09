use worth_store::{
    ArtifactFamilyId, CompatibilityRelation, RollingCapabilityWindow, RollingUpgradeAdmissionPlan,
    RollingUpgradePolicy, RollingUpgradeWindow, UpgradeAdmissionWitness,
};

fn main() {
    let _ = RollingUpgradeAdmissionPlan::new(
        RollingUpgradePolicy::FirstShipTwoCapability,
        window(),
        capability_window(),
        CompatibilityRelation::Native,
        witness(),
    );
}

fn window() -> RollingUpgradeWindow {
    panic!("compile-fail fixture")
}

fn capability_window() -> RollingCapabilityWindow {
    panic!("compile-fail fixture")
}

fn witness() -> UpgradeAdmissionWitness {
    panic!("compile-fail fixture")
}

fn _family_id() -> ArtifactFamilyId {
    ArtifactFamilyId::new("commit_envelope")
}
