use worth_store_layout_indexes::{
    AccessPlanIdentity, CurrentLayoutMaterialization, DegradedScanLoweringBasis,
    DegradedScanRebindAdmission, DegradedScanRebindTrace,
};

fn worth(
    stale_basis: DegradedScanLoweringBasis,
    replacement_plan: AccessPlanIdentity,
    current: CurrentLayoutMaterialization,
    trace: DegradedScanRebindTrace,
) -> DegradedScanRebindAdmission {
    DegradedScanRebindAdmission {
        stale_basis,
        replacement_plan,
        current,
        trace,
    }
}

fn main() {}
