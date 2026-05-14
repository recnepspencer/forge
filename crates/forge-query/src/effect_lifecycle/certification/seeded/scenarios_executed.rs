use crate::effect_lifecycle::{
    admit_effect_intent, discover_effect_lifecycle_support, effect_batch,
    evaluate_effect_eligibility, normalize_raw_effect_intent, scope_admitted_effect_plan,
    EffectAuthoringBasis, EffectEligibilityOutcome, EffectExecutionAuthority, EffectFamily,
    RawEffectIntent,
};
use crate::workflow::{
    MergeLoweringInput, WorkflowAuthorityTargetFamily, WorkflowDeclarationFamily,
    WritebackLoweringInput,
};
use forge_relational::facade::history::BranchId;

use super::super::support::{
    branch_snapshot_token, create_entity, relational_runtime_with_intent_strategy,
    test_bridge_with_writeback_authority,
};
use super::{
    branch_mutation_basis, raw_mutation_effect_with_binding, runtime_workflow_binding,
    runtime_workflow_binding_with_snapshot, scalar_or_terminal_row, seeded_label, workflow_request,
    EffectLifecycleSeededCertificationRow, EffectLifecycleSeededOutcomeClass, SeedStepper,
};

pub(super) fn scalar_mutation_row(
    index: usize,
    stepper: &mut SeedStepper,
) -> EffectLifecycleSeededCertificationRow {
    let branch = seeded_label("branch", stepper, index);
    let entity_name = seeded_label("entity", stepper, index);
    let updated_name = seeded_label("updated", stepper, index);
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, &entity_name, BranchId(branch.clone()));
    let binding = runtime_workflow_binding_with_snapshot(&branch_snapshot_token(&runtime, &branch));
    let basis = EffectAuthoringBasis::from(branch_mutation_basis(&branch));
    let raw = raw_mutation_effect_with_binding(binding, entity_id, updated_name);
    let support = discover_effect_lifecycle_support(basis.family(), EffectFamily::Mutation);
    let normalized =
        normalize_raw_effect_intent(&basis, raw).expect("mutation scenario should normalize");
    let admitted = match evaluate_effect_eligibility(normalized.clone()) {
        EffectEligibilityOutcome::Admitted(eligibility) => admit_effect_intent(eligibility),
        other => panic!("expected admitted mutation scenario, got {other:?}"),
    };
    let scoped = scope_admitted_effect_plan(admitted);
    let lowered = scoped.lower().expect("mutation scenario should lower");
    let executed = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("mutation scenario should execute");
    scalar_or_terminal_row(
        format!("seeded-mutation-executed-{index}"),
        EffectLifecycleSeededOutcomeClass::ScalarExecuted,
        basis.family(),
        EffectFamily::Mutation,
        1,
        support.discovery_digest().to_string(),
        Some(normalized.normalized_digest().to_string()),
        executed
            .lowered()
            .authority_scoped_plan()
            .admitted()
            .admitted_digest()
            .to_string(),
        Some(
            executed
                .lowered()
                .authority_scoped_plan()
                .plan_digest()
                .to_string(),
        ),
        Some(
            executed
                .lowered()
                .lowered_effect_execution_plan_digest()
                .to_string(),
        ),
        Some(executed.effect_execution_digest().to_string()),
        None,
        executed.counters().clone(),
    )
}

pub(super) fn scalar_writeback_row(
    index: usize,
    stepper: &mut SeedStepper,
) -> EffectLifecycleSeededCertificationRow {
    let branch = seeded_label("branch", stepper, index);
    let basis = EffectAuthoringBasis::from(branch_mutation_basis(&branch));
    let raw = RawEffectIntent::Writeback {
        binding: runtime_workflow_binding(),
        request: workflow_request(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
        ),
        input: WritebackLoweringInput::projected_state_diff(),
    };
    let support = discover_effect_lifecycle_support(basis.family(), EffectFamily::Writeback);
    let normalized =
        normalize_raw_effect_intent(&basis, raw).expect("writeback scenario should normalize");
    let admitted = match evaluate_effect_eligibility(normalized.clone()) {
        EffectEligibilityOutcome::Admitted(eligibility) => admit_effect_intent(eligibility),
        other => panic!("expected admitted writeback scenario, got {other:?}"),
    };
    let scoped = scope_admitted_effect_plan(admitted);
    let lowered = scoped.lower().expect("writeback scenario should lower");
    let bridge = test_bridge_with_writeback_authority();
    let executed = lowered
        .execute_with(EffectExecutionAuthority::bridge(&bridge))
        .expect("writeback scenario should execute");
    scalar_or_terminal_row(
        format!("seeded-writeback-executed-{index}"),
        EffectLifecycleSeededOutcomeClass::ScalarExecuted,
        basis.family(),
        EffectFamily::Writeback,
        1,
        support.discovery_digest().to_string(),
        Some(normalized.normalized_digest().to_string()),
        executed
            .lowered()
            .authority_scoped_plan()
            .admitted()
            .admitted_digest()
            .to_string(),
        Some(
            executed
                .lowered()
                .authority_scoped_plan()
                .plan_digest()
                .to_string(),
        ),
        Some(
            executed
                .lowered()
                .lowered_effect_execution_plan_digest()
                .to_string(),
        ),
        Some(executed.effect_execution_digest().to_string()),
        None,
        executed.counters().clone(),
    )
}

pub(super) fn merge_lowered_row(
    index: usize,
    stepper: &mut SeedStepper,
) -> EffectLifecycleSeededCertificationRow {
    let branch = seeded_label("branch", stepper, index);
    let source = seeded_label("candidate", stepper, index);
    let basis = EffectAuthoringBasis::from(branch_mutation_basis(&branch));
    let raw = RawEffectIntent::Merge {
        binding: runtime_workflow_binding(),
        request: workflow_request(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
        ),
        input: MergeLoweringInput::reconcile_into_target(BranchId(branch), BranchId(source)),
    };
    let support = discover_effect_lifecycle_support(basis.family(), EffectFamily::Merge);
    let normalized =
        normalize_raw_effect_intent(&basis, raw).expect("merge scenario should normalize");
    let admitted = match evaluate_effect_eligibility(normalized.clone()) {
        EffectEligibilityOutcome::Admitted(eligibility) => admit_effect_intent(eligibility),
        other => panic!("expected admitted merge scenario, got {other:?}"),
    };
    let scoped = scope_admitted_effect_plan(admitted);
    let lowered = scoped.lower().expect("merge scenario should lower");
    scalar_or_terminal_row(
        format!("seeded-merge-lowered-{index}"),
        EffectLifecycleSeededOutcomeClass::Lowered,
        basis.family(),
        EffectFamily::Merge,
        1,
        support.discovery_digest().to_string(),
        Some(normalized.normalized_digest().to_string()),
        lowered
            .authority_scoped_plan()
            .admitted()
            .admitted_digest()
            .to_string(),
        Some(lowered.authority_scoped_plan().plan_digest().to_string()),
        Some(lowered.lowered_effect_execution_plan_digest().to_string()),
        None,
        None,
        lowered.counters().clone(),
    )
}

pub(super) fn batch_mutation_row(
    index: usize,
    stepper: &mut SeedStepper,
) -> EffectLifecycleSeededCertificationRow {
    let branch = seeded_label("branch", stepper, index);
    let mut runtime = relational_runtime_with_intent_strategy();
    let batch_width = 2 + stepper.next_index(2);
    let basis = EffectAuthoringBasis::from(branch_mutation_basis(&branch));
    let mut seeded_entities = Vec::with_capacity(batch_width);
    for component_index in 0..batch_width {
        let entity_name = format!(
            "{}-{component_index}",
            seeded_label("entity", stepper, index)
        );
        let entity_id = create_entity(&mut runtime, &entity_name, BranchId(branch.clone()));
        let desired = format!(
            "{}-{component_index}",
            seeded_label("after", stepper, index)
        );
        seeded_entities.push((entity_id, desired));
    }
    let binding = runtime_workflow_binding_with_snapshot(&branch_snapshot_token(&runtime, &branch));
    let mut draft = effect_batch();
    for (entity_id, desired) in seeded_entities {
        draft = draft.push(raw_mutation_effect_with_binding(
            binding.clone(),
            entity_id,
            desired,
        ));
    }
    let support = discover_effect_lifecycle_support(basis.family(), EffectFamily::Mutation);
    let admitted = draft
        .using_basis(basis.clone())
        .admit()
        .expect("batch scenario should admit");
    let admitted_digest = admitted.batch_digest().to_string();
    let lowered = admitted.lower().expect("batch scenario should lower");
    let lowered_digest = lowered.batch_digest().to_string();
    let executed = lowered
        .execute_with(EffectExecutionAuthority::relational(&mut runtime))
        .expect("batch scenario should execute");
    scalar_or_terminal_row(
        format!("seeded-batch-executed-{index}"),
        EffectLifecycleSeededOutcomeClass::BatchExecuted,
        basis.family(),
        EffectFamily::Mutation,
        batch_width,
        support.discovery_digest().to_string(),
        None,
        admitted_digest,
        None,
        Some(lowered_digest),
        Some(executed.batch_digest().to_string()),
        None,
        executed.counters().clone(),
    )
}
