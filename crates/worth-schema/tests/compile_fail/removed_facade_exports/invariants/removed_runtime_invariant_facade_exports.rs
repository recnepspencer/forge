use schema::facade::{
    bootstrap_invariant_plan, bootstrap_runtime_invariant_plan, BootstrapInvariantPlan,
    BootstrapRuntimeInvariant, BootstrapRuntimeInvariantPlan,
};

fn main() {
    let _ = (
        bootstrap_invariant_plan,
        bootstrap_runtime_invariant_plan,
        None::<BootstrapInvariantPlan>,
        None::<BootstrapRuntimeInvariant>,
        None::<BootstrapRuntimeInvariantPlan>,
    );
}
