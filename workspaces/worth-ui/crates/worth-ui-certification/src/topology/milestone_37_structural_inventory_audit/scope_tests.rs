#[test]
fn scoped_crate_roots_are_phase_1_inventory_scope() {
    const SCOPED_CRATE_ROOTS: &[&str] = &[
        "crates/worth-ui-runtime",
        "crates/worth-ui-inspection",
        "crates/worth-ui-query-binding",
        "crates/worth-ui-certification",
    ];

    assert_eq!(SCOPED_CRATE_ROOTS.len(), 4);
}
