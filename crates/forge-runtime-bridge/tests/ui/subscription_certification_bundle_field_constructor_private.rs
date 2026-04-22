use forge_runtime_bridge::facade::{
    BridgeSubscriptionBundleField, BridgeSubscriptionBundleFieldState,
};

fn main() {
    let _field = BridgeSubscriptionBundleField {
        field_name: "comparison_inputs".into(),
        field_state: BridgeSubscriptionBundleFieldState::Present,
        field_digest: "forged-digest".into(),
        canonical_basis: "forged-basis".into(),
        digest: "forged-field-digest".into(),
    };
}
