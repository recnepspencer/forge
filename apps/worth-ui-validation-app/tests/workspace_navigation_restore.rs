use worth_ui_validation_app::{
    ValidationDynamicPageRequest, ValidationPageHandle, ValidationStaticPageId,
    ValidationWorkspaceNavigation,
};

#[test]
fn open_dynamic_pages_survive_static_page_switches_and_close_back_to_owner() {
    let mut navigation = ValidationWorkspaceNavigation::default();

    let product_detail = navigation
        .open_dynamic_page(
            ValidationDynamicPageRequest::product_detail("P-1001")
                .expect("product detail request should be valid"),
        )
        .expect("product detail request should open");
    let order_detail = navigation
        .open_dynamic_page(
            ValidationDynamicPageRequest::order_detail("O-4902")
                .expect("order detail request should be valid"),
        )
        .expect("order detail request should open");

    navigation.select_static_page(ValidationStaticPageId::Customers);
    assert_eq!(
        navigation.open_dynamic_pages().len(),
        2,
        "static page switches must not discard open dynamic instances"
    );

    assert!(navigation.select_dynamic_page(product_detail));
    assert_eq!(
        navigation.active_page(),
        ValidationPageHandle::Dynamic(product_detail)
    );

    assert!(navigation.close_dynamic_page(product_detail));
    assert_eq!(
        navigation.active_page(),
        ValidationPageHandle::Static(ValidationStaticPageId::Products),
        "closing an active product detail page should return to its owning static page"
    );
    assert!(navigation.select_dynamic_page(order_detail));
}
