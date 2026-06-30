use worth_kernel::workload_composition::{
    CompletedBooleanLoopReconstructionHandoff, CompletedBooleanSplitHandoff,
};

fn packetless_chain_helper_is_not_public_authority(
    loop_handoff: &CompletedBooleanLoopReconstructionHandoff,
    split_handoff: &CompletedBooleanSplitHandoff,
) {
    let _ = loop_handoff.complete_boolean_chain_integration_handoff(split_handoff);
}

fn main() {}
