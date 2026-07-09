use worth_store::{RollingUpgradeAdmissionPlan, UpgradeAdmissionWitness};

fn main() {
    let _ = witness_from_plan(&plan());
}

fn witness_from_plan(plan: &RollingUpgradeAdmissionPlan) -> &UpgradeAdmissionWitness {
    plan.witness()
}

fn plan() -> RollingUpgradeAdmissionPlan {
    panic!("compile-fail fixture")
}
