use super::support::*;
use crate::authorized_projection::{PolicyAspectMask, PolicyInfluenceSet, PolicyMaskSnapshot};
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_narrowing::narrow_policy_query;
use crate::relationship_proof::RelationshipProofDescriptorSet;

#[test]
fn read_policy_basis_and_write_gate_evidence_cite_compatible_basis_identity() {
    let canonical = canonical_task_query();
    let read_context = policy_context_for_mode(
        "shared-policy-basis",
        false,
        false,
        PolicyExecutionModeRequest::CurrentRead,
    );
    let write_context = policy_context_for_mode(
        "shared-policy-basis",
        false,
        false,
        PolicyExecutionModeRequest::GraphMutation,
    );
    let narrowed = narrow_policy_query(
        &canonical,
        read_context,
        PolicyMaskSnapshot::synthetic_authority(
            write_context.bundle().policy_digest(),
            PolicyAspectMask::allow_all(),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .expect("read-side policy narrowing should produce comparable basis evidence");
    let mut runtime = supported_policy_gate_runtime("shared-policy-basis-gate");

    let receipt = runtime
        .write_with_policy_context(task_insert_command("shared-policy-basis"), write_context)
        .expect("write-side policy gate should execute");
    let gate = receipt
        .obligation_dispatch()
        .and_then(|dispatch| dispatch.policy_gate())
        .expect("write dispatch should carry policy gate evidence");

    assert_eq!(narrowed.policy_digest(), gate.policy_digest());
    assert_eq!(
        narrowed.tenant_truth_basis_digest(),
        gate.tenant_truth_basis_digest()
    );
    assert_eq!(
        narrowed.tenant_schema_basis_digest(),
        gate.tenant_schema_basis_digest()
    );
    assert_eq!(narrowed.branch_access_digest(), gate.branch_access_digest());
    assert_ne!(
        narrowed.policy_tenant_admission_digest(),
        gate.policy_tenant_admission_digest()
    );
}
