use forge_runtime_bridge::facade::BridgeCausalEnvelopeReceipt;

fn main() {
    let _ = BridgeCausalEnvelopeReceipt {
        envelope_identity_digest: "identity".into(),
        envelope_digest: "envelope".into(),
        counter_digest: "counter".into(),
        receipt_digest: "receipt".into(),
    };
}
