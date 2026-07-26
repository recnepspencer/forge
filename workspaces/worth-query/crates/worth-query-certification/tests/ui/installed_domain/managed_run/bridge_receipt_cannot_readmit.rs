use worth_runtime_bridge::facade::{
    BridgeExecutionBasisFinalizationReceipt, BridgeYieldedExecutionBasisPreflight,
};

fn require_readmission_preflight(_: BridgeYieldedExecutionBasisPreflight) {}

fn readmit(receipt: BridgeExecutionBasisFinalizationReceipt) {
    require_readmission_preflight(receipt);
}

fn main() {}
