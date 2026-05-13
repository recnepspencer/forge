use forge_relational::facade::commit_strategies::{
    CommitStrategyId, CommitStrategyRegistration, IntentReconciliationStrategy,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::payloads::RecordPayload;
use forge_relational::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use forge_relational::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationalSchemaRegistry, SchemaId,
    SchemaVersionId,
};
use forge_relational::facade::transactions::{
    CreateIntent, EntitySpec, MutationIntent, RecordRef, TransactionOptions, WorkerIntentBatch,
};
use forge_relational::facade::{identity::KindId, identity::PartitionId, symbols::InternedString};
use serde_json::json;

use crate::effect_lifecycle::{
    admit_effect_intent, evaluate_effect_eligibility, normalize_raw_effect_intent,
    scope_admitted_effect_plan, EffectAuthoringBasis, EffectEligibilityOutcome,
    EffectExecutionAuthority, EffectExecutionDenialKind, ExecutedEffectAuthorityArtifact,
    RawEffectIntent,
};
use crate::workflow::{
    MutationLoweringInput, WorkflowAuthorityTargetFamily, WorkflowDeclarationFamily,
    WorkflowFreshnessPolicy,
};

use super::support::{
    admitted_branch_merge_effect, admitted_tenant_writeback_effect, branch_mutation_basis,
    runtime_workflow_binding, workflow_request,
};

#[test]
fn lowered_mutation_execution_runs_through_relational_strategy_authority() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    let lowered = scope_admitted_effect_plan(admitted_mutation_effect_for(entity_id))
        .lower()
        .expect("mutation should lower");

    let executed = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("lowered mutation should execute");

    assert!(matches!(
        executed.artifact(),
        ExecutedEffectAuthorityArtifact::Mutation(_)
    ));
    assert_eq!(executed.counters().executed_effect_count(), 1);
    assert_eq!(executed.counters().effect_execution_width(), 1);
    assert_eq!(
        executed.authority_owner(),
        executed.lowered().authority_owner()
    );
    let snapshot = runtime.snapshots().snapshot();
    let read_view = runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .expect("snapshot should read");
    let updated = read_view
        .entities()
        .iter()
        .find(|record| record.entity_id == entity_id)
        .expect("entity should still exist after execution");
    assert_eq!(
        updated.payload,
        RecordPayload::StructuredJson(json!({ "name": "authority-plan" }))
    );
}

#[test]
fn lowered_merge_execution_runs_through_relational_merge_authority() {
    let mut runtime = relational_runtime_with_intent_strategy();
    create_entity(&mut runtime, "main", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("candidate".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("candidate branch should be created");
    create_entity(
        &mut runtime,
        "feature-only",
        BranchId("candidate".to_string()),
    );
    let lowered = scope_admitted_effect_plan(admitted_branch_merge_effect())
        .lower()
        .expect("merge should lower");

    let executed = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("lowered merge should execute");

    assert!(matches!(
        executed.artifact(),
        ExecutedEffectAuthorityArtifact::Merge(_)
    ));
    assert_eq!(executed.counters().executed_effect_count(), 1);
    assert_eq!(executed.counters().effect_execution_width(), 1);
    let merge = executed
        .as_merge()
        .expect("merge artifact should be present");
    assert_eq!(
        merge.commit.outcome.commit.version_id.0,
        runtime.history().latest_commit().unwrap().version_id.0
    );
}

#[test]
fn lowered_writeback_execution_denies_until_bridge_contract_chain_exists() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let lowered = scope_admitted_effect_plan(admitted_tenant_writeback_effect())
        .lower()
        .expect("writeback should lower");

    let denial = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect_err("writeback execution should fail closed for now");

    assert_eq!(
        denial.denial_kind(),
        EffectExecutionDenialKind::WritebackContractAssemblyRequired
    );
    assert_eq!(denial.counters().execution_denied_count(), 1);
}

fn relational_runtime_with_intent_strategy() -> RelationalRuntime {
    let descriptor = IntentReconciliationStrategy::descriptor(CommitStrategyId(211));
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .commit_strategy(
            CommitStrategyRegistration::new(descriptor.clone()).expect("strategy registration"),
        )
        .commit_strategy_executor(IntentReconciliationStrategy::execution_registration(
            &descriptor,
        ))
        .build()
}

fn create_entity(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch: BranchId,
) -> forge_relational::facade::identity::EntityId {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new(format!("create-{name}")).push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw(name.to_string()),
                payload: RecordPayload::StructuredJson(json!({ "name": name })),
            }),
        )),
    );
    let outcome = txn.commit().expect("seed commit should succeed");
    outcome
        .changed_records
        .iter()
        .find_map(|record| match record {
            RecordRef::Entity(entity_id) => Some(*entity_id),
            RecordRef::Relation(_) => None,
        })
        .expect("seed commit should touch one entity")
}

fn test_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::new(vec![
                DeclaredAspect {
                    key: AspectKey(InternedString::Raw("name".to_string())),
                    binding: AspectBinding::EntityPayloadField {
                        field: InternedString::Raw("name".to_string()),
                    },
                    comparator: AspectComparator::JsonScalarEquality,
                    precision: AspectPrecision::Structured,
                },
                DeclaredAspect {
                    key: AspectKey(InternedString::Raw("lifecycle".to_string())),
                    binding: AspectBinding::LifecycleTransition,
                    comparator: AspectComparator::LifecycleTransitionEquality,
                    precision: AspectPrecision::Structured,
                },
            ]),
        })
        .expect("test entity kind should register")
}

fn admitted_mutation_effect_for(
    entity_id: forge_relational::facade::identity::EntityId,
) -> crate::effect_lifecycle::AdmittedEffectIntent {
    let normalized = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(branch_mutation_basis()),
        RawEffectIntent::Mutation {
            binding: runtime_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::MutationLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            input: MutationLoweringInput::IntentReconciliation {
                entity_id,
                desired_payload: serde_json::json!({ "name": "authority-plan" }),
            },
        },
    )
    .expect("mutation effect should normalize");

    match evaluate_effect_eligibility(normalized) {
        EffectEligibilityOutcome::Admitted(eligibility) => admit_effect_intent(eligibility),
        other => panic!("expected admitted mutation effect, got {other:?}"),
    }
}
