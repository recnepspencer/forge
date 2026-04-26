use forge_store::{
    SubscriptionSupportAccuracyCertificationRowKind, SubscriptionSupportAccuracyLaneEvidence,
};

fn main() {
    let _lane = SubscriptionSupportAccuracyLaneEvidence::certified_pass(
        SubscriptionSupportAccuracyCertificationRowKind::ReplicatedSupportExactEquivalence,
        "synthetic:source",
        "synthetic:diagnostics",
        "synthetic:counter",
    );
}
