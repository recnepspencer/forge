use super::support::*;

#[test]
fn policy_context_batch_selects_configured_domain_handle_gate() {
    let policy_context = policy_context("batch-collaborative", false);
    let expected_policy_digest = policy_context.bundle().policy_digest().to_string();
    let mut runtime = supported_batch_policy_gate_runtime("batch-policy-gate");

    let receipt = runtime
        .write_batch_with_policy_context(
            vec![task_insert_command("batch-policy-gate")],
            policy_context,
        )
        .expect("policy-aware command batch should execute");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("policy-aware batch should attach obligation dispatch");
    let gate = dispatch
        .policy_gate()
        .expect("policy-aware batch should carry gate evidence");

    assert_eq!(dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        dispatch.envelope().unwrap().context().kind(),
        WorthQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch
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
fn restricted_policy_batch_denies_before_batch_execution() {
    let restricted_policy = policy_context_with_policy_posture("batch-restricted", false, true);
    let mut runtime = supported_batch_policy_gate_runtime("batch-restricted-gate");

    let error = runtime
        .write_batch_with_policy_context(
            vec![task_insert_command("batch-restricted")],
            restricted_policy,
        )
        .expect_err("restricted command batch should deny before execution");

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
