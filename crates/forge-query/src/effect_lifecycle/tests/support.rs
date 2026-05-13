use crate::basis_lifecycle::{
    admit_basis_capability, evaluate_basis_mutation_preparation_eligibility,
    evaluate_basis_preview_closeout_eligibility, normalize_raw_basis_intent,
    scope_basis_for_mutation_preparation, scope_basis_for_preview_closeout, BasisOperationLane,
    MutationPreparationLaneWitness, PreviewCloseoutLaneWitness, RawBasisIntent,
};
use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::preview::{
    admit_preview_workflow_foundation, bind_preflight_to_preview_session, PreviewEvaluationClass,
    PreviewSessionQueryContext,
};
use crate::workflow::{
    bind_workflow_context, MergeLoweringInput, MutationLoweringInput,
    WorkflowAuthorityTargetFamily, WorkflowBindingSource, WorkflowBudgetClass,
    WorkflowContextBinding, WorkflowCostClass, WorkflowDeclarationFamily,
    WorkflowDeclarationRequest, WorkflowFreshnessPolicy, WritebackLoweringInput,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, PartitionId};

use crate::effect_lifecycle::{
    admit_effect_intent, evaluate_effect_eligibility, normalize_raw_effect_intent,
    AdmittedEffectIntent, EffectAuthoringBasis, EffectEligibilityOutcome, RawEffectIntent,
};

pub(super) fn branch_mutation_basis() -> crate::basis_lifecycle::ScopedMutationPreparationBasis {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: "branch-a".to_string(),
            accessible: true,
        },
        <MutationPreparationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch basis should normalize");
    let eligibility = evaluate_basis_mutation_preparation_eligibility(normalized)
        .expect("branch basis should admit");
    scope_basis_for_mutation_preparation(admit_basis_capability(eligibility))
}

pub(super) fn tenant_mutation_basis() -> crate::basis_lifecycle::ScopedMutationPreparationBasis {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::TenantScoped {
            tenant_identity: "tenant-a".to_string(),
            branch_identity: "branch-a".to_string(),
            schema_identity: "schema-a".to_string(),
            tenant_schema_matches: true,
        },
        <MutationPreparationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("tenant basis should normalize");
    let eligibility = evaluate_basis_mutation_preparation_eligibility(normalized)
        .expect("tenant basis should admit");
    scope_basis_for_mutation_preparation(admit_basis_capability(eligibility))
}

pub(super) fn preview_closeout_basis() -> crate::basis_lifecycle::ScopedPreviewCloseoutBasis {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::Preview {
            preview_identity: "preview-a".to_string(),
            stale: false,
        },
        <PreviewCloseoutLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("preview basis should normalize");
    let eligibility = evaluate_basis_preview_closeout_eligibility(normalized)
        .expect("preview closeout basis should admit");
    scope_basis_for_preview_closeout(admit_basis_capability(eligibility))
}

pub(super) fn runtime_workflow_binding() -> WorkflowContextBinding {
    let preflight = execution_preflights::direct_runtime_preflight();
    bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime preflight should bind")
}

pub(super) fn preview_workflow_binding() -> WorkflowContextBinding {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("effect-lifecycle-preview");
    let binding = bind_preflight_to_preview_session(
        preflight,
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("preview binding should succeed");
    let foundation =
        admit_preview_workflow_foundation(&binding).expect("preview foundation should admit");
    bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
        .expect("preview workflow binding should admit")
}

pub(super) fn workflow_request(
    family: WorkflowDeclarationFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
    freshness_policy: WorkflowFreshnessPolicy,
) -> WorkflowDeclarationRequest {
    let cost_class = match authority_target_family {
        WorkflowAuthorityTargetFamily::QueryInspection => WorkflowCostClass::InspectionNarrow,
        WorkflowAuthorityTargetFamily::RelationalMutation => {
            WorkflowCostClass::MutationLoweringNarrow
        }
        WorkflowAuthorityTargetFamily::RelationalMerge => WorkflowCostClass::MergeLoweringNarrow,
        WorkflowAuthorityTargetFamily::BridgeWriteback => {
            WorkflowCostClass::WritebackLoweringNarrow
        }
    };
    WorkflowDeclarationRequest::new(
        family,
        authority_target_family,
        cost_class,
        WorkflowBudgetClass::AuthorityTargetBounded,
        freshness_policy,
    )
}

pub(super) fn admitted_mutation_effect() -> AdmittedEffectIntent {
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
                entity_id: EntityId::new(PartitionId(1), 8, 0),
                desired_payload: serde_json::json!({ "name": "authority-plan" }),
            },
        },
    )
    .expect("mutation effect should normalize");

    admit_from_normalized(normalized)
}

pub(super) fn admitted_branch_merge_effect() -> AdmittedEffectIntent {
    let normalized = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(branch_mutation_basis()),
        RawEffectIntent::Merge {
            binding: runtime_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::MergeLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMerge,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            input: MergeLoweringInput::reconcile_into_target(
                BranchId("main".to_string()),
                BranchId("candidate".to_string()),
            ),
        },
    )
    .expect("merge effect should normalize");

    admit_from_normalized(normalized)
}

pub(super) fn admitted_invalid_merge_effect() -> AdmittedEffectIntent {
    let normalized = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(branch_mutation_basis()),
        RawEffectIntent::Merge {
            binding: runtime_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::MergeLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMerge,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            input: MergeLoweringInput::reconcile_into_target(
                BranchId("same".to_string()),
                BranchId("same".to_string()),
            ),
        },
    )
    .expect("invalid merge effect should still normalize");

    admit_from_normalized(normalized)
}

pub(super) fn admitted_tenant_writeback_effect() -> AdmittedEffectIntent {
    let normalized = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(tenant_mutation_basis()),
        RawEffectIntent::Writeback {
            binding: runtime_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            input: WritebackLoweringInput::projected_state_diff(),
        },
    )
    .expect("tenant writeback should normalize");

    admit_from_normalized(normalized)
}

fn admit_from_normalized(
    normalized: crate::effect_lifecycle::NormalizedEffectIntent,
) -> AdmittedEffectIntent {
    match evaluate_effect_eligibility(normalized) {
        EffectEligibilityOutcome::Admitted(eligibility) => admit_effect_intent(eligibility),
        other => panic!("expected admitted effect, got {other:?}"),
    }
}
