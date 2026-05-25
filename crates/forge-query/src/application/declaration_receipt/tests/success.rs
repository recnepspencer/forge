use crate::application::{ForgeQueryDeclarationReceiptInput, ForgeQueryDeclarationReceiptKind};

use super::support::{
    admitted_handle, progressed, route_checked_with_intent, MixedReceiptFamily, ReceiptInput,
    RelationalReceiptFamily,
};

#[test]
fn receipt_common_lane_reads_like_crossing_intent() {
    let receipt = admitted_handle("primary")
        .declare_review_progress_describe_plan_and_receipt(
            ReceiptInput::<RelationalReceiptFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("receipt common lane should issue"));

    assert_eq!(receipt.declaration_family_key(), "RelationalReceiptFamily");
    assert_eq!(receipt.kind(), ForgeQueryDeclarationReceiptKind::Relational);
    assert!(receipt.explain().crossing_posture().contains("successful"));
}

#[test]
fn explicit_and_common_receipt_paths_converge_on_one_digest() {
    let handle = admitted_handle("primary");
    let explicit = handle
        .receipt_routes_from_progressed(progressed(
            &handle,
            ReceiptInput::<MixedReceiptFamily>::new("edge:42"),
        ))
        .unwrap_or_else(|_| panic!("explicit receipt path should issue"));
    let common = handle
        .declare_review_progress_describe_plan_and_receipt(ReceiptInput::<MixedReceiptFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("common receipt path should issue"));

    assert_eq!(explicit.receipt_digest(), common.receipt_digest());
}

#[test]
fn mixed_route_receipts_remain_mixed() {
    let receipt = admitted_handle("primary")
        .declare_review_progress_describe_plan_and_receipt(ReceiptInput::<MixedReceiptFamily>::new(
            "edge:42",
        ))
        .unwrap_or_else(|_| panic!("mixed receipt should issue"));

    assert_eq!(receipt.kind(), ForgeQueryDeclarationReceiptKind::Mixed);
    assert_eq!(
        receipt
            .route_plan()
            .expect("route plan should be retained")
            .route_count(),
        2
    );
}

#[test]
fn advanced_lane_planned_receipt_issues_without_checked_wrapper_loss() {
    let handle = admitted_handle("primary");

    match route_checked_with_intent(
        &handle,
        ReceiptInput::<RelationalReceiptFamily>::new("edge:42"),
        crate::application::ForgeQueryDeclarationRouteIntent::Auto,
    ) {
        crate::application::ForgeQueryDeclarationRoutePlanChecked::Planned(plan) => {
            let receipt = handle
                .receipt_routes(ForgeQueryDeclarationReceiptInput::planned(plan))
                .unwrap_or_else(|_| panic!("advanced receipt lane should issue"));
            assert_eq!(receipt.kind(), ForgeQueryDeclarationReceiptKind::Relational);
        }
        _ => panic!("relational route plan should be planned"),
    }
}

#[test]
fn receipt_digest_changes_when_admitted_world_changes() {
    let primary = admitted_handle("primary")
        .declare_review_progress_describe_plan_and_receipt(
            ReceiptInput::<RelationalReceiptFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("primary world should issue receipt"));
    let alternate = admitted_handle("alternate")
        .declare_review_progress_describe_plan_and_receipt(
            ReceiptInput::<RelationalReceiptFamily>::new("edge:42"),
        )
        .unwrap_or_else(|_| panic!("alternate world should issue receipt"));

    assert_ne!(primary.receipt_digest(), alternate.receipt_digest());
}
