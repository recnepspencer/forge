use super::validation_engine_fixtures::*;

#[test]
fn unique_entity_aspect_field_invariant_rejects_duplicate_struct_field_projection() {
    let mut runtime = runtime_with_summary_title_uniqueness();
    commit_entity_with_summary(&mut runtime, "alpha", "shared-title", "open")
        .expect("first summary entity");

    let duplicate = commit_entity_with_summary(&mut runtime, "beta", "shared-title", "closed");

    assert!(duplicate.is_err());
}

#[test]
fn unique_entity_aspect_field_invariant_ignores_sibling_struct_field_values() {
    let mut runtime = runtime_with_summary_title_uniqueness();
    commit_entity_with_summary(&mut runtime, "alpha", "alpha-title", "shared-status")
        .expect("first summary entity");
    let distinct_title =
        commit_entity_with_summary(&mut runtime, "beta", "beta-title", "shared-status");

    assert!(distinct_title.is_ok());
}
