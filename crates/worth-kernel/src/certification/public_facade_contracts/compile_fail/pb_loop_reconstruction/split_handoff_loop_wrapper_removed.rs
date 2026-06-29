use worth_kernel::workload_composition::{
    CompletedBooleanSplitHandoff, PlanarBooleanLoopReconstructionCloseoutInput,
};

fn call_removed_wrapper(
    handoff: &CompletedBooleanSplitHandoff,
    input: PlanarBooleanLoopReconstructionCloseoutInput<'_>,
) {
    let _ = handoff.complete_boolean_loop_reconstruction_from_replay_undo_boundary(input);
}

fn main() {
    panic!("compile-fail fixture should never execute")
}
