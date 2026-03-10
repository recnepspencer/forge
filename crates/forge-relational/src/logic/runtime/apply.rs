#[path = "apply_execution.rs"]
mod apply_execution;
#[path = "apply_mutation.rs"]
mod apply_mutation;
#[path = "apply_patching.rs"]
mod apply_patching;

pub(crate) use apply_execution::apply_plan_to_staged_state;
