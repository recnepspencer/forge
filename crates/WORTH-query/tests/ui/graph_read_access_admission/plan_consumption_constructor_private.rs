use worth_query::facade::runtime::WorthQueryGraphReadAccessPlanConsumption;

fn main() {
    let _ = WorthQueryGraphReadAccessPlanConsumption {
        digest: String::new(),
        admitted_plan_digest: String::new(),
        admission_digest: String::new(),
        execution_binding_digest: String::new(),
        execution_strategy: String::new(),
        execution_counters: todo!(),
    };
}
