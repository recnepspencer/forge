use worth_kernel::workload_composition::SelectedTopologyConflictPlan;

fn bypass(plan: SelectedTopologyConflictPlan<'_>) {
    let _ = plan.admitted_input();
}

fn main() {}
