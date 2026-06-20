use forge_query::facade::runtime::ForgeQueryGraphReadAccessPlanConsumption;

fn main() {
    let _ = ForgeQueryGraphReadAccessPlanConsumption {
        digest: String::new(),
        admitted_plan_digest: String::new(),
        admission_digest: String::new(),
        execution_binding_digest: String::new(),
        execution_strategy: String::new(),
        execution_counters: todo!(),
    };
}
