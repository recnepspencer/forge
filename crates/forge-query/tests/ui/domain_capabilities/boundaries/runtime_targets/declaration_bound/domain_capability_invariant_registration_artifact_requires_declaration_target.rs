use forge_query::facade::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    materialize_query_invariant_catalog_registration_artifact,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryInvariantCapabilityContributionAuthoring, ForgeQueryMutationReceipt,
    ForgeQueryRuntimeWriteAuthorityAdapter, ForgeQueryWorkspaceError, ForgeQueryWriteCommand,
    WriteAuthorityExecutionReceipt,
};
use forge_relational::facade::runtime::{InvariantRegistration, InvariantRule, RelationalRuntime};
use forge_runtime_bridge::facade::RuntimeBridge;

struct TestWriteAuthorityAdapter;

impl ForgeQueryRuntimeWriteAuthorityAdapter for TestWriteAuthorityAdapter {
    fn write(
        &mut self,
        _bridge: &RuntimeBridge,
        _relational_runtime: Option<&mut RelationalRuntime>,
        _command: ForgeQueryWriteCommand,
    ) -> Result<WriteAuthorityExecutionReceipt, ForgeQueryWorkspaceError> {
        unreachable!()
    }
}

fn main() {
    let command = ForgeQueryWriteCommand::Delete {
        entity_identity: "task-1".to_string(),
    };
    let mutation_receipt = ForgeQueryMutationReceipt {
        commit_identity: "commit-1".to_string(),
        snapshot_token: "snapshot-1".to_string(),
        deltas: Vec::new(),
        bridge_authority: None,
    };
    let envelope = TestWriteAuthorityAdapter
        .build_write_authority_execution_receipt(&command, mutation_receipt)
        .boundary_envelope()
        .clone();
    let requested = ForgeQueryInvariantCapabilityContributionAuthoring::invariant_rule_registration(
        InvariantRegistration::commit_boundary_blocking(InvariantRule::MaxMergedIntents(2)),
        "runtime.invariant.catalog_registration",
        "registration should stay declaration-bound",
    )
    .for_lower_runtime_boundary_envelope(&envelope);
    let eligible = match evaluate_requested_domain_capability_contribution(requested) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };
    let admitted = match admit_eligible_domain_capability_contribution(eligible) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };
    let target = admitted.payload().target().clone();
    let ready = match prepare_admitted_domain_capability_contribution_for_materialization(
        admitted,
        target,
    ) {
        forge_proof::TransitionOutcome::Success(value) => value,
        _ => unreachable!(),
    };

    let _ = materialize_query_invariant_catalog_registration_artifact(ready);
}
