use worth_query::facade::{certification, domain};

use super::installed_operation_fixture::{lineage_workflow_workspace, LineageEvidenceScenario};

#[test]
fn certification_replay_compares_full_lineage_output_and_stage_evidence_semantics() {
    let mut workspace = lineage_workflow_workspace(
        "dependency-impact-lineage-replay",
        domain::WorthQueryOperationLineageContract::Evolve,
        false,
        vec![LineageEvidenceScenario::SingularSuccessor],
    )
    .unwrap();
    let basis = super::operation_lineage::mutation_basis();
    let original = super::operation_lineage::bind(&workspace, basis.clone())
        .reexecute(super::operation_lineage::intent(), &mut workspace)
        .unwrap();
    let replay = certification::replay_installed_workflow(
        certification::issue_query_certification_replay_capability(),
        &original,
        super::operation_lineage::bind(&workspace, basis),
        super::operation_lineage::intent(),
        &mut workspace,
    )
    .unwrap();
    let original_closure = original.semantic_aspect_dependency_closure().unwrap();
    assert_eq!(
        replay.comparison(),
        &domain::WorthQueryReplayComparison::Equivalent
    );
    for source_kind in [
        SourceKind::WorkflowRead,
        SourceKind::WorkflowEffect,
        SourceKind::WorkflowLineage,
        SourceKind::WorkflowOutput,
    ] {
        assert!(has_source_kind(original_closure, source_kind));
    }
    assert!(replay
        .replay_semantics()
        .stages()
        .iter()
        .any(|stage| !stage.effects().is_empty() && !stage.lineage().is_empty()));
}

#[derive(Clone, Copy)]
enum SourceKind {
    WorkflowRead,
    WorkflowEffect,
    WorkflowLineage,
    WorkflowOutput,
}

fn has_source_kind(
    closure: &domain::WorthQueryCompiledSemanticAspectDependencyClosure,
    expected: SourceKind,
) -> bool {
    closure.dependencies().iter().any(|dependency| {
        matches!(
            (expected, dependency.source()),
            (
                SourceKind::WorkflowRead,
                domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowRead(_)
            ) | (
                SourceKind::WorkflowEffect,
                domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowEffect(_)
            ) | (
                SourceKind::WorkflowLineage,
                domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowLineage(_)
            ) | (
                SourceKind::WorkflowOutput,
                domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowOutput { .. }
            )
        )
    })
}
