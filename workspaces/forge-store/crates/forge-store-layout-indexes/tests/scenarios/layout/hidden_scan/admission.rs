use forge_store_layout_indexes::{
    access_shapes, AccessLaneClassification, FullDeclaredScanBasis, FullDeclaredScanView,
};

#[test]
fn foreground_full_scan_is_denied_by_the_production_shape_owner() {
    let outcome = access_shapes().full_declared_scan(
        AccessLaneClassification::Foreground,
        FullDeclaredScanBasis::DeclaredFullTraversal,
    );

    assert!(matches!(
        outcome.view(),
        FullDeclaredScanView::HiddenDenied(_)
    ));
}

#[test]
fn verifier_full_scan_is_explicitly_admitted_by_the_production_shape_owner() {
    let outcome = access_shapes().full_declared_scan(
        AccessLaneClassification::Verifier,
        FullDeclaredScanBasis::DeclaredFullTraversal,
    );

    assert!(matches!(outcome.view(), FullDeclaredScanView::Success(_)));
}
