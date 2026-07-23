use worth_query::facade::domain;

use super::installed_operation_fixture::{
    grouped_lineage_workflow_workspace, LineageEvidenceScenario,
};

#[test]
fn binding_work_tracks_each_real_dependency_width_without_unrelated_scans() {
    let (_workspace, settled, _delivery) =
        super::dependency_impact::changed_projection("dependency-impact-scale-direct");
    let direct = settled.semantic_aspect_dependency_closure();

    let mut lineage = grouped_lineage_workflow_workspace(
        "dependency-impact-scale-workflow",
        vec![LineageEvidenceScenario::SingularSuccessor],
    )
    .unwrap();
    let trace = super::operation_lineage::execute(&mut lineage);
    let workflow = trace.semantic_aspect_dependency_closure().unwrap();
    let direct_d = direct.dependencies().len();
    let workflow_d = workflow.dependencies().len();

    assert_ne!(direct_d, workflow_d);
    assert_exact_binding_work(direct, direct_d);
    assert_exact_binding_work(workflow, workflow_d);
}

#[test]
fn change_work_tracks_affected_edges_not_total_compiled_width() {
    let (changed, unchanged) =
        super::dependency_impact_live::authority_correct_overlap_impact_counters();

    assert_eq!(changed.affected_edges, 5);
    assert_eq!(unchanged.affected_edges, 3);
    for counters in [changed, unchanged] {
        assert_eq!(counters.owner_changes_inspected, 1);
        assert_eq!(counters.unrelated_dependency_scans, 0);
        assert_eq!(counters.consumer_registry_scans, 0);
    }
}

fn assert_exact_binding_work(
    closure: &domain::WorthQueryCompiledSemanticAspectDependencyClosure,
    d: usize,
) {
    let counters = closure.counters();
    assert_eq!(counters.compiled_dependency_count, d);
    assert_eq!(counters.canonical_traversal_edges, d);
    assert_eq!(counters.uniqueness_hash_checks, d);
    assert_eq!(counters.impact_index_dependency_visits, d);
    assert_eq!(counters.unrelated_definition_scans, 0);
    assert_eq!(counters.unrelated_runtime_scans, 0);
    assert_eq!(counters.consumer_registry_scans, 0);
}
