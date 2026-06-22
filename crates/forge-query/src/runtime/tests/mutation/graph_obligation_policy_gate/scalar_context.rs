use super::support::*;
use crate::policy_basis::{PolicyAdmissionDisposition, PolicyExecutionModeRequest};

#[test]
fn policy_context_write_selects_configured_domain_handle_gate() {
    let policy_context = policy_context("collaborative", false);
    let expected_admission_digest = policy_context.bundle().digest().as_str().to_string();
    let expected_policy_digest = policy_context.bundle().policy_digest().to_string();
    let mut runtime = supported_policy_gate_runtime("collaborative-allow");

    let receipt = runtime
        .write_with_policy_context(task_insert_command("policy-gate"), policy_context)
        .expect("policy-aware write should execute");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("policy-aware write should attach obligation dispatch");
    let gate = dispatch
        .policy_gate()
        .expect("policy-aware dispatch should carry gate evidence");

    assert_eq!(dispatch.selection().matched_obligation_count(), 1);
    assert_eq!(
        dispatch
            .envelope()
            .unwrap()
            .context()
            .operating_world_digest(),
        ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle()
            .descriptor_digest()
    );
    assert_eq!(
        gate.policy_tenant_admission_digest(),
        expected_admission_digest
    );
    assert_eq!(gate.policy_digest(), expected_policy_digest);
    assert_eq!(
        gate.verdict(),
        ForgeQueryGraphMutationPolicyGateVerdict::Allow
    );
    assert_eq!(gate.registration_full_scan_count(), 0);
}

#[test]
fn read_admitted_policy_context_cannot_drive_graph_mutation_gate() {
    let read_policy_context = policy_context_for_mode(
        "read-context",
        false,
        false,
        PolicyExecutionModeRequest::CurrentRead,
    );
    let expected_admission_digest = read_policy_context.bundle().digest().as_str().to_string();
    let mut runtime = supported_policy_gate_runtime("graph-mutation-mode-required");

    let error = runtime
        .write_with_policy_context(task_insert_command("wrong-mode"), read_policy_context)
        .expect_err("read-admitted policy context must not authorize graph mutation");

    match error {
        ForgeQueryRuntimeError::GraphMutationPolicyContextDenied {
            expected,
            actual,
            policy_tenant_admission_digest,
        } => {
            assert_eq!(expected, PolicyExecutionModeRequest::GraphMutation);
            assert_eq!(actual, PolicyExecutionModeRequest::CurrentRead);
            assert_eq!(policy_tenant_admission_digest, expected_admission_digest);
        }
        other => panic!("expected graph mutation policy context denial, got {other:?}"),
    }
}

#[test]
fn collaborative_and_restricted_policy_rules_produce_different_gate_verdicts() {
    let collaborative_policy = policy_context("collaborative-rules", false);
    let mut collaborative_runtime = supported_policy_gate_runtime("collaborative-rule");
    let collaborative_receipt = collaborative_runtime
        .write_with_policy_context(
            task_insert_command("collaborative-rules"),
            collaborative_policy,
        )
        .expect("collaborative gate should allow");
    assert_eq!(
        collaborative_receipt
            .obligation_dispatch()
            .and_then(|dispatch| dispatch.policy_gate())
            .map(|gate| gate.verdict()),
        Some(ForgeQueryGraphMutationPolicyGateVerdict::Allow)
    );

    let restricted_policy = policy_context_with_policy_posture("restricted-rules", false, true);
    let expected_admission_digest = restricted_policy.bundle().digest().as_str().to_string();
    let mut restricted_runtime = supported_policy_gate_runtime("restricted-rule");
    let restricted_error = restricted_runtime
        .write_with_policy_context(task_insert_command("restricted-rules"), restricted_policy)
        .expect_err("restricted gate should deny before backend write");

    match restricted_error {
        ForgeQueryRuntimeError::GraphMutationPolicyGateDenied(evidence) => {
            assert_eq!(
                evidence.verdict(),
                ForgeQueryGraphMutationPolicyGateVerdict::Deny
            );
            assert_eq!(
                evidence.policy_tenant_admission_digest(),
                expected_admission_digest
            );
            assert_eq!(evidence.matched_obligation_count(), 1);
        }
        other => panic!("expected graph mutation policy gate denial, got {other:?}"),
    }
    assert_task_live_row_count(restricted_runtime, "restricted-denial-state", 0);
}

#[test]
fn narrowed_policy_basis_advises_without_losing_basis_identity() {
    let narrowed_context = policy_context("narrowed", true);
    let expected_admission_digest = narrowed_context.bundle().digest().as_str().to_string();
    let mut runtime = supported_policy_gate_runtime("narrowed-advisory");

    let receipt = runtime
        .write_with_policy_context(task_insert_command("narrowed-policy"), narrowed_context)
        .expect("narrowed policy gate should advise without blocking");
    let projection = receipt
        .obligation_dispatch()
        .expect("policy-aware write should attach dispatch")
        .evidence_projection();

    assert_eq!(
        projection.policy_tenant_admission_digest(),
        Some(expected_admission_digest.as_str())
    );
    assert_eq!(
        projection.policy_gate_verdict(),
        Some(ForgeQueryGraphMutationPolicyGateVerdict::Advise)
    );
    assert_eq!(
        receipt
            .obligation_dispatch()
            .and_then(|dispatch| dispatch.policy_gate())
            .map(|gate| gate.admission_disposition()),
        Some(PolicyAdmissionDisposition::AdmittedNarrowed)
    );
}

fn assert_task_live_row_count(runtime: ForgeQueryRuntime, workspace: &str, expected: usize) {
    let mut workspace = runtime
        .workspace(workspace)
        .expect("runtime should open workspace for state verification");
    let live: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.policy-denial-state", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .schema_basis("tasks-policy-denial-state")
        })
        .expect("state verification live view should declare");
    assert_eq!(workspace.read(&live).len(), expected);
}
