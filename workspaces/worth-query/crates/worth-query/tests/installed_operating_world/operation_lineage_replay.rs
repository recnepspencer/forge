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
    assert_ne!(original.identity(), replay.replay_trace().identity());
    assert_ne!(
        original.lineage_report().unwrap().identity(),
        replay.replay_trace().lineage_report().unwrap().identity()
    );
    assert_eq!(
        original.lineage_report().unwrap().evidence()[0]
            .outcome()
            .engine_artifact(),
        replay.replay_trace().lineage_report().unwrap().evidence()[0]
            .outcome()
            .engine_artifact()
    );
    assert_eq!(
        original.lineage_report().unwrap().evidence()[0]
            .outcome()
            .continuity_evidence(),
        replay.replay_trace().lineage_report().unwrap().evidence()[0]
            .outcome()
            .continuity_evidence()
    );
    assert_eq!(
        original.lineage_report().unwrap().evidence()[0]
            .foundational_lineage()
            .subject_evidence_identity(),
        replay.replay_trace().lineage_report().unwrap().evidence()[0]
            .foundational_lineage()
            .subject_evidence_identity()
    );
}
