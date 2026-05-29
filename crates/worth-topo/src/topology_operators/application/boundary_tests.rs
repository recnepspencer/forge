#[test]
fn application_operator_files_do_not_decode_raw_query_payloads() {
    let application_mod = include_str!("mod.rs");
    let admission = include_str!("admission.rs");
    let bindings = include_str!("bindings.rs");
    let existing_truth = include_str!("existing_truth.rs");

    for source in [application_mod, admission, bindings, existing_truth] {
        assert!(
            !source.contains(".payload"),
            "operator application surface should not decode raw query payloads",
        );
    }
}

#[test]
fn application_operator_files_do_not_own_post_write_materialized_decode() {
    let application_mod = include_str!("mod.rs");
    let bindings = include_str!("bindings.rs");

    for source in [application_mod, bindings] {
        assert!(
            !source.contains("workspace.materialize("),
            "operator application surface should not own post-write materialized reads",
        );
        assert!(
            !source.contains("serde_json::from_value"),
            "operator application surface should not deserialize materialized rows directly",
        );
    }
}




