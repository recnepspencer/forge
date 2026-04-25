use forge_store::{
    SubscriptionSupportAccuracyCertificationRow,
    SubscriptionSupportAccuracyCertificationRowKind,
};

fn main() {
    let _row = SubscriptionSupportAccuracyCertificationRow::new(
        SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
        "synthetic:row:evidence",
        0,
        0,
    );
}
