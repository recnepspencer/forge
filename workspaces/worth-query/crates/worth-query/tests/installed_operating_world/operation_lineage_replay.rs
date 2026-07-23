use worth_query::facade::{certification, domain};

use super::installed_operation_fixture::{lineage_workflow_workspace, LineageEvidenceScenario};
use super::operation_lineage::{bind, execute, intent, mutation_basis};

#[test]
fn certification_replay_reexecutes_the_same_lineage_semantics() {
    let mut workspace = lineage_workflow_workspace(
        "lineage-replay",
        domain::WorthQueryOperationLineageContract::Evolve,
        false,
        vec![LineageEvidenceScenario::SingularSuccessor],
    )
    .unwrap();
    let basis = mutation_basis();
    let original = execute(&mut workspace, basis.clone());
    let replay = certification::replay_installed_workflow(
        certification::issue_query_certification_replay_capability(),
        &original,
        bind(&workspace, basis),
        intent(),
        &mut workspace,
    )
    .unwrap();

    assert_eq!(
        replay.comparison(),
        &domain::WorthQueryReplayComparison::Equivalent
    );
    assert_ne!(original.identity(), replay.replay_trace_identity());
    let original_lineage_width = replay
        .original_semantics()
        .stages()
        .iter()
        .flat_map(|stage| stage.lineage())
        .count();
    let replay_lineage_width = replay
        .replay_semantics()
        .stages()
        .iter()
        .flat_map(|stage| stage.lineage())
        .count();
    assert!(original_lineage_width > 0);
    assert_eq!(original_lineage_width, replay_lineage_width);
    assert!(original.lineage_report().is_some());
}
