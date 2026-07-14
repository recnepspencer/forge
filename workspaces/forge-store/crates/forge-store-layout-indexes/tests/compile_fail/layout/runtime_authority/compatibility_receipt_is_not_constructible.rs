use forge_store_compatibility::{
    RollingUpgradeAdmissionPlan, RollingWindowCompatibilityReceipt,
};

fn forge_receipt(plan: RollingUpgradeAdmissionPlan) -> RollingWindowCompatibilityReceipt {
    RollingWindowCompatibilityReceipt::new(plan)
}

fn main() {}
