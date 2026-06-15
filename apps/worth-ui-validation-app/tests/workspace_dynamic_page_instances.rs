use worth_ui_validation_app::{ValidationDynamicPageRequest, ValidationWorkspaceNavigation};

#[test]
fn dynamic_pages_dedupe_identical_template_instances_and_keep_distinct_keys() {
    let mut navigation = ValidationWorkspaceNavigation::default();

    let product_a = navigation
        .open_dynamic_page(
            ValidationDynamicPageRequest::product_detail("P-1001")
                .expect("product detail request should be valid"),
        )
        .expect("product detail request should open");
    let product_a_repeat = navigation
        .open_dynamic_page(
            ValidationDynamicPageRequest::product_detail("P-1001")
                .expect("product detail request should be valid"),
        )
        .expect("product detail request should dedupe");
    let product_b = navigation
        .open_dynamic_page(
            ValidationDynamicPageRequest::product_detail("P-1002")
                .expect("product detail request should be valid"),
        )
        .expect("second product detail request should open");
    let order_a = navigation
        .open_dynamic_page(
            ValidationDynamicPageRequest::order_detail("O-4902")
                .expect("order detail request should be valid"),
        )
        .expect("order detail request should open");

    assert_eq!(product_a, product_a_repeat);
    assert_ne!(product_a, product_b);
    assert_ne!(product_a, order_a);
    assert_eq!(navigation.open_dynamic_pages().len(), 3);
}

#[test]
fn invalid_dynamic_page_request_is_rejected_before_navigation_state_changes() {
    let navigation = ValidationWorkspaceNavigation::default();

    let denial = ValidationDynamicPageRequest::product_detail("   ")
        .expect_err("blank product ids must fail at the typed request boundary");

    assert_eq!(
        denial,
        worth_ui_validation_app::ValidationDynamicPageRequestDenial::EmptyParameter {
            kind: worth_ui_validation_app::ValidationDynamicPageKind::ProductDetail,
            parameter_name: "product_id",
        }
    );
    assert!(navigation.open_dynamic_pages().is_empty());
}
