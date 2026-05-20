#[test]
fn query_assembly_mod_does_not_inline_snapshot_row_fallback_logic() {
    let source = include_str!("mod.rs");

    assert!(
        !source.contains("workspace.read(&self.entities)"),
        "query assembly entry surface should not inline entity-row snapshot reads",
    );
    assert!(
        !source.contains("workspace.materialize(&self.materialized)"),
        "query assembly entry surface should not inline materialized-row snapshot reads",
    );
    assert!(
        !source.contains("naming_attachment_report_from_query"),
        "query assembly entry surface should not own naming attachment row decoding",
    );
}
