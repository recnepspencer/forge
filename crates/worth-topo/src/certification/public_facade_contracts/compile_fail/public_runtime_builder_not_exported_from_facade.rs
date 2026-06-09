use topology::facade::{
    build_milestone_one_runtime, configure_milestone_one_runtime_builder,
    milestone_one_runtime_builder, MilestoneOneRuntimeSetupError,
};

fn main() {
    let _ = milestone_one_runtime_builder;
    let _ = build_milestone_one_runtime;
    let _ = configure_milestone_one_runtime_builder;
    let _: Option<MilestoneOneRuntimeSetupError> = None;
}
