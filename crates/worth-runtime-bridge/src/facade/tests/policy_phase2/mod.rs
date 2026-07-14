use crate::facade::{
    AdmittedBridgePolicyContract, BridgePolicyDeclaration, BridgePolicyProvenanceRecord,
    BridgePolicyReplayBundle, LoweredBridgeExecutionPolicy, RuntimeBridge,
};

mod admission;
mod provenance;
mod route_policy;

fn admitted_bundle(
    runtime: &RuntimeBridge,
    declaration: BridgePolicyDeclaration,
) -> (
    AdmittedBridgePolicyContract,
    LoweredBridgeExecutionPolicy,
    BridgePolicyProvenanceRecord,
    BridgePolicyReplayBundle,
) {
    let contract = runtime
        .admit_policy_declaration(declaration)
        .expect("policy should admit");
    let lowered = runtime.lower_admitted_policy(&contract);
    let provenance = runtime.canonicalize_policy_provenance(&contract, &lowered);
    let replay_bundle = runtime.replay_policy_bundle(&contract, &lowered, &provenance);
    (contract, lowered, provenance, replay_bundle)
}
