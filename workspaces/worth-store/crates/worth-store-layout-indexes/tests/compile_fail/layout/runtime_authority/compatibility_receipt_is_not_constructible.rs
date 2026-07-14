use worth_store_compatibility::{
    RollingUpgradeAdmissionPlan, RollingWindowCompatibilityReceipt,
};

fn worth_receipt(plan: RollingUpgradeAdmissionPlan) -> RollingWindowCompatibilityReceipt {
    RollingWindowCompatibilityReceipt::new(plan)
}

fn main() {}
