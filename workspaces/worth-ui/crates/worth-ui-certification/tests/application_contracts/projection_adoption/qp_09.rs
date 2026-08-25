#[test]
fn product_backend_support_rows_are_query_owned_and_digest_pinned() {
    let report =
        worth_ui_query_binding::certification::certify_product_projection_support_contract()
            .expect("the production projection backend must satisfy its Query support pins");

    assert!(report.satisfied());
    assert_eq!(report.requirement_count(), 5);
    assert_eq!(report.matched_required_count(), 5);
    assert_eq!(report.blocking_finding_count(), 0);
    assert!(!report.contract_digest().is_empty());
    assert!(!report.report_digest().is_empty());
}
