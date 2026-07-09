use crate::tests::async_node_public_hierarchy_branch_support::{
    explanation_signature, public_hierarchy_branch_workload,
};

#[test]
fn public_rediscovery_keeps_gate_visibility_and_hierarchy_explanations_branch_honest() {
    let workload = public_hierarchy_branch_workload();

    assert_eq!(
        workload.sibling_gate_baseline.gate_digest(),
        workload.feature_gate_baseline.gate_digest(),
        "equivalent sibling branch should share baseline gate truth"
    );
    assert_eq!(
        workload.sibling_gate_history.explanation_summary(),
        workload.feature_gate_history_baseline.explanation_summary()
    );
    assert_eq!(
        workload
            .sibling_hierarchy
            .historical_parity_report()
            .explanation_summary(),
        workload
            .feature_hierarchy_baseline
            .historical_parity_report()
            .explanation_summary()
    );

    assert_eq!(
        workload.sibling_still_gate_baseline.gate_digest(),
        workload.sibling_gate_baseline.gate_digest(),
        "restoring feature must not perturb sibling gate truth"
    );
    assert_eq!(
        workload
            .sibling_still_hierarchy
            .hierarchy_replay_summary()
            .replay_digest(),
        workload
            .sibling_hierarchy
            .hierarchy_replay_summary()
            .replay_digest(),
        "restoring feature must not perturb sibling hierarchy truth"
    );

    assert_eq!(
        workload.feature_gate_restored_state.gate_digest(),
        workload.feature_gate_baseline.gate_digest()
    );
    assert_ne!(
        workload.feature_gate_restored_state.gate_digest(),
        workload.feature_gate_drifted.gate_digest()
    );
    assert_eq!(
        workload
            .feature_gate_restored_history
            .explanation_availability(),
        workload
            .feature_gate_history_baseline
            .explanation_availability()
    );
    assert_eq!(
        explanation_signature(workload.feature_gate_restored_history.explanation_summary()),
        explanation_signature(workload.feature_gate_history_baseline.explanation_summary())
    );
    assert_eq!(
        workload
            .feature_gate_restored_history
            .replay_reconstruction()
            .replay_digest(),
        workload
            .feature_gate_history_baseline
            .replay_reconstruction()
            .replay_digest()
    );

    assert_eq!(
        workload
            .feature_hierarchy_restored
            .historical_parity_report()
            .explanation_availability(),
        workload
            .feature_hierarchy_baseline
            .historical_parity_report()
            .explanation_availability()
    );
    assert_eq!(
        explanation_signature(
            workload
                .feature_hierarchy_restored
                .historical_parity_report()
                .explanation_summary(),
        ),
        explanation_signature(
            workload
                .feature_hierarchy_baseline
                .historical_parity_report()
                .explanation_summary(),
        )
    );
    assert_eq!(
        workload
            .feature_hierarchy_restored
            .hierarchy_replay_summary()
            .replay_digest(),
        workload
            .feature_hierarchy_baseline
            .hierarchy_replay_summary()
            .replay_digest()
    );
    assert_ne!(
        workload
            .feature_hierarchy_restored
            .hierarchy_replay_summary()
            .replay_digest(),
        workload
            .feature_hierarchy_after_cancellation
            .hierarchy_replay_summary()
            .replay_digest()
    );
}
