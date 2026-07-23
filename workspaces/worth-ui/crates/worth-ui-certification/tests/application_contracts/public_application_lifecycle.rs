use worth_ui_certification::scenario::application_authority_closure::certify_application_authority_closure;

#[test]
fn production_facades_preserve_one_application_authority_end_to_end() {
    let report = certify_application_authority_closure();

    assert!(report.file_rust_converged());
    assert!(report.generation_changed_once());
    assert!(report.host_session_preserved());
    assert!(report.graph_node_count() > 0);
    assert_eq!(report.query_binding_count(), 1);
    assert_eq!(report.planning_policy_family_count(), 1);
    assert_eq!(report.planning_classification_count(), 1);
    assert!(report.foreign_graph_denied());
}
