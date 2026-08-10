use super::*;

#[test]
fn journal_identity_inventory_is_seeded_and_scans_clean() {
    let workspace_root = workspace_root();
    let inventory = worth_query_journal_identity_inventory();
    let operations = inventory
        .iter()
        .map(|row| row.kind())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(inventory.iter().all(|row| !row.path().is_empty()));
    assert!(inventory
        .iter()
        .all(|row| !row.required_patterns().is_empty()));
    assert_eq!(missing_operation_count(&operations), 0);
    assert_eq!(
        scan_journal_identity_forbidden_patterns(&workspace_root),
        Vec::new()
    );
    assert_eq!(
        scan_journal_identity_required_pattern_failures(&workspace_root),
        Vec::new()
    );
}
