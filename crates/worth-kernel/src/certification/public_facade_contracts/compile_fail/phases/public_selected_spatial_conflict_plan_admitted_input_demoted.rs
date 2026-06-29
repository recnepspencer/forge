use worth_kernel::workload_composition::SelectedSpatialConflictPlan;

fn bypass(plan: SelectedSpatialConflictPlan<'_>) {
    let _ = plan.admitted_input();
}

fn main() {}
