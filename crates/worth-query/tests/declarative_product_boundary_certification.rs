mod support;

#[path = "declarative_product_boundary_certification/grammar_matrix.rs"]
mod grammar_matrix;
#[path = "declarative_product_boundary_certification/hostile_matrix.rs"]
mod hostile_matrix;
#[path = "declarative_product_boundary_certification/parity_bounded.rs"]
mod parity_bounded;
#[path = "declarative_product_boundary_certification/sabotage_matrix.rs"]
mod sabotage_matrix;
#[path = "declarative_product_boundary_certification/support.rs"]
mod product_support;

#[test]
fn composed_product_boundary_bundle_is_complete() {
    let bundle = worth_query::facade::certification::certify_declarative_product_boundary()
        .expect("the complete declarative product boundary should certify");
    assert_eq!(bundle.grammar_row_count(), 10);
    assert_eq!(bundle.hostile_row_count(), 9);
    assert_eq!(bundle.sabotage_row_count(), 6);
    assert!(!bundle.closure_digest().is_empty());
}
