use worth_runtime_bridge::facade::{
    BridgeBoundExecutionBasis, BridgeExecutionBasisReadmissionPending, RuntimeBridge,
};

fn discard_owner_evidence(
    runtime: &RuntimeBridge,
    pending: BridgeExecutionBasisReadmissionPending,
) -> BridgeBoundExecutionBasis {
    runtime.commit_yielded_execution_basis_readmission(pending)
}

fn main() {}
