use forge_runtime_bridge::facade::{
    bridge_truth_external_identity_token, BridgeIdentityEvidence,
};

fn main() {
    let token = bridge_truth_external_identity_token("snapshot:test").expect("token");
    let evidence = BridgeIdentityEvidence::from_external_authority(token);
    let _label = evidence.as_str();
}
