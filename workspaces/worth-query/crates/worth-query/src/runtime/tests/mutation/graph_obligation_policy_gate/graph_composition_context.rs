use super::support::*;

#[test]
fn policy_context_graph_composition_selects_configured_domain_handle_gate() {
    let policy_context = policy_context("graph-collaborative", false);
    let expected_policy_digest = policy_context.bundle().policy_digest().to_string();
    let mut runtime = supported_graph_policy_gate_runtime("graph-policy-gate");
    let (commands, breadth, program) = task_graph_program("graph-policy-gate");

    let receipt = runtime
        .write_graph_batch_with_policy_context(commands, breadth, program, policy_context)
        .expect("policy-aware graph composition should execute");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("policy-aware graph composition should attach dispatch");
    let gate = dispatch
        .policy_gate()
        .expect("policy-aware graph composition should carry gate evidence");

    assert_eq!(dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        dispatch.envelope().unwrap().context().kind(),
        WorthQueryGraphObligationDispatchContextKind::GraphComposition
    );
    assert_eq!(
        dispatch
            .envelope()
            .unwrap()
            .context()
            .operating_world_digest(),
        WorthQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle()
            .descriptor_digest()
    );
    assert_eq!(gate.policy_digest(), expected_policy_digest);
    assert_eq!(
        gate.verdict(),
        WorthQueryGraphMutationPolicyGateVerdict::Allow
    );
}

#[test]
fn restricted_policy_graph_composition_denies_before_execution() {
    let restricted_policy = policy_context_with_policy_posture("graph-restricted", false, true);
    let mut runtime = supported_graph_policy_gate_runtime("graph-restricted-gate");
    let (commands, breadth, program) = task_graph_program("graph-restricted");

    let error = runtime
        .write_graph_batch_with_policy_context(commands, breadth, program, restricted_policy)
        .expect_err("restricted graph composition should deny before execution");

    match error {
        WorthQueryRuntimeError::GraphMutationPolicyGateDenied(evidence) => {
            assert_eq!(
                evidence.verdict(),
                WorthQueryGraphMutationPolicyGateVerdict::Deny
            );
            assert_eq!(evidence.matched_obligation_count(), 1);
        }
        other => panic!("expected graph mutation policy gate denial, got {other:?}"),
    }
}
