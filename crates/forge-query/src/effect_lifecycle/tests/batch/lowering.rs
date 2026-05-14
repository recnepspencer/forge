use forge_relational::facade::history::BranchId;
use serde_json::json;

use crate::effect_lifecycle::{
    effect_batch, EffectAuthoringBasis, EffectAuthorityLane, EffectAuthorityOwner,
};
use crate::workflow::WorkflowBasisFamily;

use super::super::execution_support::{
    create_entity, relational_runtime_with_intent_strategy, runtime_snapshot_token,
};
use super::super::support::{
    branch_mutation_basis, raw_mutation_effect_with_binding, runtime_workflow_binding_with_snapshot,
};

#[test]
fn mutation_batch_lowers_once_into_a_batch_native_relational_artifact() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let left = create_entity(&mut runtime, "left", BranchId("main".to_string()));
    let right = create_entity(&mut runtime, "right", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let binding = runtime_workflow_binding_with_snapshot(&runtime_snapshot_token(&runtime));

    let lowered = effect_batch()
        .using_basis(EffectAuthoringBasis::from(branch_mutation_basis()))
        .push(raw_mutation_effect_with_binding(
            binding.clone(),
            left,
            json!({ "name": "left-batched" }),
        ))
        .push(raw_mutation_effect_with_binding(
            binding,
            right,
            json!({ "name": "right-batched" }),
        ))
        .admit()
        .expect("batch should admit")
        .lower()
        .expect("batch should lower");

    let batch = lowered
        .as_relational_mutation_batch()
        .expect("mutation-only batch should lower into a batch-native mutation artifact");
    assert_eq!(lowered.authority_lane(), EffectAuthorityLane::Relational);
    assert_eq!(
        lowered.authority_owner(),
        EffectAuthorityOwner::ForgeRelational
    );
    assert_eq!(
        lowered.workflow_basis_lane(),
        &WorkflowBasisFamily::RuntimePreflight
    );
    assert_eq!(batch.declarations().len(), 2);
    assert_eq!(lowered.counters().batch_lowering_count(), 1);
    assert_eq!(lowered.counters().effect_lowering_width(), 2);
    assert_eq!(lowered.counters().effect_executor_rediscovery_count(), 0);
    assert!(batch
        .declarations()
        .iter()
        .all(|declaration| declaration.counters().workflow_mutation_lowering_count() == 1));
}
