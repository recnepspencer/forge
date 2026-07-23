#[test]
fn one_real_row_successor_retains_unrelated_catalog_storage_at_scale() {
    const ROWS: usize = 128;
    let (runtime, roots, _) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_viewport_catalog(ROWS);
    let predecessor = runtime
        .allocation_invalidation_index
        .borrow()
        .catalog
        .clone();
    let changed_root = roots[0];
    let changed_row = predecessor
        .row_for_root(changed_root)
        .expect("real activated catalog contains the changed root")
        .clone();
    let unaffected_root = roots[ROWS - 1];
    let unaffected_scope = predecessor
        .row_for_root(unaffected_root)
        .expect("real activated catalog contains unrelated truth")
        .scope();

    let mut successor = predecessor.clone();
    assert_eq!(
        successor.remove_root(changed_root),
        Some(changed_row.scope())
    );
    successor.insert(changed_row);

    assert_eq!(successor.len(), ROWS);
    assert_eq!(
        successor
            .row_for_root(unaffected_root)
            .expect("unrelated row remains addressable")
            .scope(),
        unaffected_scope
    );
    assert!(
        successor.shared_row_nodes_with(&predecessor) > ROWS - 32,
        "a one-row successor must retain almost all real predecessor row storage"
    );
}
