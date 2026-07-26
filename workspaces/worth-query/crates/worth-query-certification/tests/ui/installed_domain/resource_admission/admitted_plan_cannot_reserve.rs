use worth_query_host::facade::admission::resource_admission::WorthQueryAdmittedExecutionResourcePlan;

fn reserve(plan: WorthQueryAdmittedExecutionResourcePlan) {
    let _ = plan.reserve_capacity();
}

fn main() {}
