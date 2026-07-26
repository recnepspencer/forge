use super::*;

#[test]
fn completed_workflow_closure_retains_declared_and_realized_roles_at_exact_d_cost() {
    let mut workspace = mutation_workflow_workspace("dependency-impact-workflow").unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let trace = workspace
        .prepare_mutation_operating_world()
        .unwrap()
        .family(MutationFamily)
        .bind(&installed, WorkflowMutation)
        .unwrap()
        .admit_workflow_resources(
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .start_workflow(&mut workspace)
        .unwrap()
        .advance(
            "mutate",
            domain::WorthQueryWorkflowValue::Text("commit".into()),
            &mut workspace,
        )
        .unwrap()
        .complete()
        .unwrap();
    let closure = trace.semantic_aspect_dependency_closure().unwrap();
    let d = closure.dependencies().len();
    let counters = closure.counters();

    assert_eq!(d, 12);
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::EffectFamily(_)
    )));
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::InstalledInvariant("workflow-invariant:1")
    )));
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowEffect(_)
    )));
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowInvariant(_)
    )));
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowOutput { .. }
    )));
    assert_eq!(counters.effect_contract_edges, 1);
    assert_eq!(counters.invariant_contract_edges, 1);
    assert_eq!(counters.realized_effect_edges, 1);
    assert_eq!(counters.realized_invariant_edges, 1);
    assert_eq!(counters.realized_workflow_output_edges, 1);
    assert_exact_d_work(closure, d);
}

#[test]
fn certification_replay_preserves_the_compiled_workflow_closure() {
    let mut workspace = workflow_workspace("dependency-impact-replay").unwrap();
    let original = bind_workflow(&workspace)
        .admit_workflow_resources(
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .reexecute(intent(), &mut workspace)
        .unwrap();
    let original_closure = original.semantic_aspect_dependency_closure().unwrap();

    let replay = certification::replay_installed_workflow(
        certification::issue_query_certification_replay_capability(),
        &original,
        bind_workflow(&workspace),
        intent(),
        crate::suite::installed_operation_fixture::execution_resource_request(),
        &mut workspace,
    )
    .unwrap();
    assert_eq!(
        replay.comparison(),
        &domain::WorthQueryReplayComparison::Equivalent
    );
    assert_exact_d_work(original_closure, original_closure.dependencies().len());
}
