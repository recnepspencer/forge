use crate::policy_execution_seam::{
    graph_mutation_gate_certification_anchors,
    runtime_backed_policy_execution_seam_support_profile, PolicyExecutionSeamSupportStatus,
    PolicyExecutionSeamSurface,
};
use crate::policy_narrowing::{
    runtime_backed_policy_narrowing_support_profile, PolicyNarrowingSupportStatus,
    PolicyNarrowingSurface,
};

#[test]
fn policy_aware_execution_support_is_verified_for_graph_mutation_gate() {
    let narrowing_profile = runtime_backed_policy_narrowing_support_profile();
    assert_eq!(
        narrowing_profile
            .surfaces()
            .iter()
            .find(|(surface, _)| *surface == PolicyNarrowingSurface::PolicyAwareExecution)
            .map(|(_, status)| *status),
        Some(PolicyNarrowingSupportStatus::Verified)
    );

    let seam_profile = runtime_backed_policy_execution_seam_support_profile();
    assert_eq!(
        seam_profile
            .surfaces()
            .iter()
            .find(|(surface, _)| *surface == PolicyExecutionSeamSurface::GraphMutationGate)
            .map(|(_, status)| *status),
        Some(PolicyExecutionSeamSupportStatus::Verified)
    );
    assert_eq!(
        graph_mutation_gate_certification_anchors(),
        &[
            "scalar_policy_gate_allow",
            "scalar_policy_gate_deny",
            "scalar_policy_gate_advise",
            "wrong_mode_policy_context_denial",
            "no_match_policy_context_has_no_gate_evidence",
            "command_batch_policy_gate",
            "graph_composition_policy_gate",
            "read_write_policy_basis_parity",
        ]
    );
}
