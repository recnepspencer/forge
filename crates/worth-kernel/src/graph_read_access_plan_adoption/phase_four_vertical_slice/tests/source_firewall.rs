use super::super::source_firewall::reject_post_admission_local_graph_read_residue;

#[test]
fn local_loop_after_plan_admission_fails_source_firewall() {
    let violation = reject_post_admission_local_graph_read_residue(&[(
        "crates/worth-kernel/src/query_adoption/graph_read_access",
        "fn local_graph_traversal() { let _ = manual_read_plan(); }",
    )])
    .expect_err("local graph traversal after plan admission must fail");

    assert_eq!(
        "crates/worth-kernel/src/query_adoption/graph_read_access",
        violation.source_path()
    );
    assert_eq!("local_graph_traversal", violation.forbidden_pattern());
}

#[test]
fn receipt_backed_query_path_passes_post_admission_source_firewall() {
    let report = reject_post_admission_local_graph_read_residue(&[(
        "crates/worth-kernel/src/graph_read_access_plan_adoption/phase_four_vertical_slice",
        "execute_read_family_with_access_plan; graph_read_access_plan_consumption",
    )])
    .expect("canonical Query execution and receipt surfaces should pass");

    assert_eq!(1, report.checked_source_count());
}
