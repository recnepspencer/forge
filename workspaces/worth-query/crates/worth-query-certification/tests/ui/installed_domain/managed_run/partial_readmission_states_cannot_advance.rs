use worth_runtime_bridge::facade::BridgeExecutionBasisReadmissionPending;

fn advance_bridge(pending: BridgeExecutionBasisReadmissionPending) {
    pending.commit();
}

fn main() {}
