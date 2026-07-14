use worth_store::{
    SubscriptionSupportAccuracyCertificationRow,
    SubscriptionSupportAccuracyCertificationRowKind,
};

fn main() {
    let _row = SubscriptionSupportAccuracyCertificationRow {
        row_kind: SubscriptionSupportAccuracyCertificationRowKind::ExactSupportTrustedControl,
        evidence_digest: String::new(),
        forbidden_exact_overclaim_count: 0,
        global_scan_debt_count: 0,
        row_digest: String::new(),
    };
}
