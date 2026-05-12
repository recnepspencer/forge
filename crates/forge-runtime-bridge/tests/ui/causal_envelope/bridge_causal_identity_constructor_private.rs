use forge_runtime_bridge::facade::BridgeCausalEnvelopeIdentity;

fn main() {
    let _ = BridgeCausalEnvelopeIdentity {
        request_digest: "request".into(),
        causal_observation_anchor_digest: "anchor".into(),
        evidence_binding_digest: "bindings".into(),
        counter_digest: "counter".into(),
        identity_digest: "identity".into(),
    };
}
