use hadwiger_research::facade::ExperimentPlan;

fn main() {
    let _ = ExperimentPlan::from_dead_end_without_suppression_check("dead");
}
