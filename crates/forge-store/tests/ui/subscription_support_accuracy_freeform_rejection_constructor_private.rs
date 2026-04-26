use forge_store::{
    SubscriptionSupportAccuracyCertificationRowKind, SubscriptionSupportAccuracyLaneEvidence,
    SupportCatalogEpoch,
};

fn main() {
    let failure = SupportCatalogEpoch::new(0).unwrap_err();
    let _lane = SubscriptionSupportAccuracyLaneEvidence::typed_rejection(
        SubscriptionSupportAccuracyCertificationRowKind::StaleSupportRejected,
        &failure,
        "synthetic:source",
        "synthetic:diagnostics",
        "synthetic:counter",
    );
}
