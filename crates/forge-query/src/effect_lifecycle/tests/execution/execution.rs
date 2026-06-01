use forge_relational::facade::history::BranchId;
use forge_runtime_bridge::facade::BridgeWritebackOutcomeClass;
use serde_json::json;

use super::execution_support::{
    create_entity, relational_runtime_with_intent_strategy, test_bridge,
    test_bridge_with_writeback_authority,
};
use super::support::{
    admitted_branch_merge_effect, admitted_mutation_effect_for_entity_with_binding,
    admitted_tenant_writeback_effect, runtime_workflow_binding_with_snapshot,
};
use crate::aspect_field_authoring::aspect_key;
use crate::effect_lifecycle::{
    scope_admitted_effect_plan, EffectExecutionAuthority, EffectExecutionDenialKind,
    ExecutedEffectAuthorityArtifact,
};

pub(super) use super::execution_support::{branch_snapshot_token, runtime_snapshot_token};

#[test]
fn lowered_mutation_execution_runs_through_relational_strategy_authority() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let lowered = scope_admitted_effect_plan(admitted_mutation_effect_for_entity_with_binding(
        runtime_workflow_binding_with_snapshot(&runtime_snapshot_token(&runtime)),
        entity_id,
        json!({ "name": "authority-plan" }),
    ))
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
    assert_eq!(entity_name(updated), Some("authority-plan".to_string()));
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
fn lowered_writeback_execution_requires_bridge_authority() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let lowered = scope_admitted_effect_plan(admitted_tenant_writeback_effect())
        .lower()
        .expect("writeback should lower");

    let denial = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect_err("writeback execution should reject non-bridge authority");

    assert_eq!(
        denial.denial_kind(),
        EffectExecutionDenialKind::AuthorityOverrideRejected
    );
    assert_eq!(denial.counters().execution_denied_count(), 1);
}

#[test]
fn lowered_mutation_execution_rejects_bridge_host_override() {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should be created");
    let lowered = scope_admitted_effect_plan(admitted_mutation_effect_for_entity_with_binding(
        runtime_workflow_binding_with_snapshot(&branch_snapshot_token(&runtime, "branch-a")),
        entity_id,
        json!({ "name": "authority-plan" }),
    ))
    .lower()
    .expect("mutation should lower");

    let denial = lowered
        .execute_with(EffectExecutionAuthority::bridge(&test_bridge()))
        .expect_err("mutation execution should reject bridge host override");

    assert_eq!(
        denial.denial_kind(),
        EffectExecutionDenialKind::AuthorityOverrideRejected
    );
    assert_eq!(denial.counters().execution_denied_count(), 1);
}

#[test]
fn lowered_merge_execution_rejects_bridge_host_override() {
    let lowered = scope_admitted_effect_plan(admitted_branch_merge_effect())
        .lower()
        .expect("merge should lower");

    let denial = lowered
        .execute_with(EffectExecutionAuthority::bridge(&test_bridge()))
        .expect_err("merge execution should reject bridge host override");

    assert_eq!(
        denial.denial_kind(),
        EffectExecutionDenialKind::AuthorityOverrideRejected
    );
    assert_eq!(denial.counters().execution_denied_count(), 1);
}

#[test]
fn lowered_writeback_execution_denies_without_bound_writeback_authority() {
    let lowered = scope_admitted_effect_plan(admitted_tenant_writeback_effect())
        .lower()
        .expect("writeback should lower");

    let denial = lowered
        .execute_with(EffectExecutionAuthority::bridge(&test_bridge()))
        .expect_err("writeback execution should fail when no writeback authority is bound");

    assert_eq!(
        denial.denial_kind(),
        EffectExecutionDenialKind::BridgeWritebackExecutionFailed
    );
    assert_eq!(denial.counters().execution_denied_count(), 1);
}

#[test]
fn lowered_writeback_execution_runs_through_bridge_authority() {
    let bridge = test_bridge_with_writeback_authority();
    let lowered = scope_admitted_effect_plan(admitted_tenant_writeback_effect())
        .lower()
        .expect("writeback should lower");

    let executed = lowered
        .execute_with(EffectExecutionAuthority::bridge(&bridge))
        .expect("lowered writeback should execute");

    let (outcome, receipt) = executed
        .as_writeback()
        .expect("writeback artifact should be present");
    let execution = executed
        .writeback_execution()
        .expect("writeback execution proof should be retained");
    assert_eq!(
        executed.authority_owner(),
        executed.lowered().authority_owner()
    );
    assert_eq!(executed.counters().executed_effect_count(), 1);
    assert_eq!(executed.counters().effect_execution_width(), 1);
    assert_eq!(
        receipt.outcome_class(),
        BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(receipt.failure_class(), None);
    assert_eq!(
        outcome.authoritative_artifact_digest(),
        receipt.authoritative_artifact_digest()
    );
    assert_eq!(execution.outcome().digest(), outcome.digest());
    assert_eq!(
        execution.execution_receipt().authority_receipt_digest(),
        receipt.digest()
    );
    let record = bridge
        .diagnostics()
        .last_writeback_execution_record()
        .expect("bridge should retain one writeback execution record");
    assert_eq!(record.outcome_digest(), Some(outcome.digest()));
    assert_eq!(record.receipt_digest(), Some(receipt.digest()));
    assert_eq!(record.request_digest(), Some(receipt.request_digest()));
}

fn entity_name(record: &forge_relational::facade::runtime::EntityReadRecord) -> Option<String> {
    let state = record.authoritative_aspect_state.as_ref()?;
    let value = state.get(&aspect_key("name").ok()?)?;
    match value.view() {
        forge_foundational::facade::ContractValidatedAspectValueView::Scalar(
            forge_foundational::facade::AspectValue::String(
                forge_foundational::facade::InternedString::Raw(actual),
            ),
        ) => Some(actual.clone()),
        _ => None,
    }
}
