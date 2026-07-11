use forge_store_certification::S6CertifiedQueueExecutionEvidence;
use forge_store_io_scheduler::{
    execute_grouped_ready_queue_plans, group_ready_queue_pair, QueueGroupingOutcome,
};
use forge_store_physical_backend::{
    BackendQueueExecutionAdaptation, BackendQueueExecutionCompletion, BackendQueueExecutionPosture,
    BackendQueueSpeculativeScope,
};

use super::support::{admitted_plan, backend_witness};

#[test]
fn grouped_certification_preserves_secondary_replay_identity() {
    let grouping = group_ready_queue_pair(admitted_plan(), admitted_plan());
    let QueueGroupingOutcome::Grouped(grouped) = grouping else {
        panic!("equivalent ready plans should group");
    };
    let expected_secondary = grouped.replay_identities()[1];
    let scope = BackendQueueSpeculativeScope::admitted(
        grouped.first().grouping_basis().security_scope_identity(),
        grouped.first().grouping_basis().tenant_scope(),
        grouped.first().grouping_basis().key_scope(),
    );
    let witness = backend_witness();
    let posture = BackendQueueExecutionPosture::from_admitted_capability(
        &witness,
        BackendQueueExecutionAdaptation::None,
    )
    .expect("certification backend posture should admit");
    let completion = BackendQueueExecutionCompletion::for_certification(
        grouped
            .backend_completion_binding()
            .backend_execution_binding(),
        posture,
    )
    .observe_queue_depth(1)
    .observe_read_ahead(1, scope);
    let outcome = execute_grouped_ready_queue_plans(grouped, completion);

    let certified = S6CertifiedQueueExecutionEvidence::from_outcome(&outcome)
        .expect("executed grouped outcome should certify");

    assert_eq!(
        certified.secondary_replay_identity(),
        Some(expected_secondary)
    );
    assert_eq!(certified.counters().grouped_writes(), 2);
}
