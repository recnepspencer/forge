use schema::facade::{bootstrap_tracing_plan, BootstrapTracingPlan};

fn main() {
    let _ = (bootstrap_tracing_plan, None::<BootstrapTracingPlan>);
}
