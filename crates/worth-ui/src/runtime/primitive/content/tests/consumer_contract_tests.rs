use super::support::{
    card_surface_id, content_source, row_surface_id, runtime_for_source, surface_id,
};

#[test]
fn same_content_receipt_shape_drives_distinct_primitive_consumers() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        ("flow_kind", "inline"),
    ]));

    let button_like = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("button-like primitive resolves");
    let row_like = runtime
        .resolve_primitive_proof(&row_surface_id())
        .expect("row-like primitive resolves");
    let card_like = runtime
        .resolve_primitive_proof(&card_surface_id())
        .expect("card-like primitive resolves");

    assert_eq!(
        button_like.component_id(),
        "worth.component.primitive_proof"
    );
    assert_eq!(
        row_like.component_id(),
        "worth.component.primitive_row_proof"
    );
    assert_eq!(
        card_like.component_id(),
        "worth.component.primitive_card_proof"
    );
    assert_eq!(
        button_like.content().items(),
        row_like.content().items(),
        "content item receipts are component-family independent"
    );
    assert_eq!(button_like.content().items(), card_like.content().items());
    assert_ne!(
        button_like.content().dependency_fact(),
        row_like.content().dependency_fact(),
        "each consumer keeps its own runtime fact identity"
    );
}
