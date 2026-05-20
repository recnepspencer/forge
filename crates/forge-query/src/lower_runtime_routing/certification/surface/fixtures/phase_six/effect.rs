use forge_relational::facade::history::BranchId;

use crate::basis_lifecycle::{
    admit_basis_capability, evaluate_basis_mutation_preparation_eligibility,
    normalize_raw_basis_intent, scope_basis_for_mutation_preparation, BasisOperationLane,
    MutationPreparationLaneWitness, RawBasisIntent,
};
use crate::effect_lifecycle::{
    admit_effect_intent, evaluate_effect_eligibility, normalize_raw_effect_intent,
    scope_admitted_effect_plan, EffectAuthoringBasis, EffectEligibilityOutcome,
    EffectExecutionAuthority, RawEffectIntent,
};
use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::workflow::{
    synthetic_runtime_workflow_binding, MergeLoweringInput, MutationLoweringInput,
    WorkflowAuthorityTargetFamily, WorkflowBudgetClass, WorkflowCostClass,
    WorkflowDeclarationFamily, WorkflowDeclarationRequest, WorkflowFreshnessPolicy,
};

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};
use super::effect_support::{
    branch_snapshot_token, create_entity, relational_runtime_with_intent_strategy,
    test_bridge_with_writeback_authority,
};

pub(crate) fn representative_effect_relational_mutation_row() -> RepresentativeArtifacts {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("branch-a".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("branch-a should exist");
    let raw = RawEffectIntent::Mutation {
        binding: runtime_workflow_binding_with_snapshot(&branch_snapshot_token(
            &runtime, "branch-a",
        )),
        request: workflow_request(
            WorkflowDeclarationFamily::MutationLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMutation,
        ),
        input: MutationLoweringInput::IntentReconciliation {
            entity_id,
            desired_payload: serde_json::json!({ "name": "phase6-relational-mutation" }),
        },
    };
    let executed = scope_admitted_effect_plan(admit_effect(
        raw,
        EffectAuthoringBasis::from(branch_mutation_basis("branch-a")),
    ))
    .lower()
    .expect("relational mutation should lower")
    .execute_with(EffectExecutionAuthority::relational(&mut runtime))
    .expect("relational mutation should execute");
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMutation,
        ForgeQueryLowerRuntimeAuthorityOwner::Relational,
        "Effect-backed relational mutation",
        &[
            "effect_relational_mutation_subject_v1".to_string(),
            format!("execution:{}", executed.effect_execution_digest()),
            format!("receipt:{}", executed.receipt().receipt_digest()),
        ],
        executed
            .lowered()
            .lowered_effect_execution_plan_digest()
            .to_string(),
        executed.receipt().receipt_digest().to_string(),
    )
}

pub(crate) fn representative_effect_relational_merge_row() -> RepresentativeArtifacts {
    let mut runtime = relational_runtime_with_intent_strategy();
    create_entity(&mut runtime, "main", BranchId("main".to_string()));
    runtime
        .history_authority()
        .create_branch(
            BranchId("candidate".to_string()),
            &BranchId("main".to_string()),
        )
        .expect("candidate branch should exist");
    create_entity(
        &mut runtime,
        "candidate-only",
        BranchId("candidate".to_string()),
    );
    let raw = RawEffectIntent::Merge {
        binding: runtime_workflow_binding(),
        request: workflow_request(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
        ),
        input: MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    };
    let executed = scope_admitted_effect_plan(admit_effect(
        raw,
        EffectAuthoringBasis::from(branch_mutation_basis("main")),
    ))
    .lower()
    .expect("relational merge should lower")
    .execute_with(EffectExecutionAuthority::relational(&mut runtime))
    .expect("relational merge should execute");
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMerge,
        ForgeQueryLowerRuntimeAuthorityOwner::Relational,
        "Effect-backed relational merge",
        &[
            "effect_relational_merge_subject_v1".to_string(),
            format!("execution:{}", executed.effect_execution_digest()),
            format!("receipt:{}", executed.receipt().receipt_digest()),
        ],
        executed
            .lowered()
            .lowered_effect_execution_plan_digest()
            .to_string(),
        executed.receipt().receipt_digest().to_string(),
    )
}

pub(crate) fn representative_effect_bridge_writeback_row() -> RepresentativeArtifacts {
    let bridge = test_bridge_with_writeback_authority();
    let raw = RawEffectIntent::Writeback {
        binding: runtime_workflow_binding(),
        request: workflow_request(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
        ),
        input: crate::workflow::WritebackLoweringInput::projected_state_diff(),
    };
    let executed = scope_admitted_effect_plan(admit_effect(
        raw,
        EffectAuthoringBasis::from(branch_mutation_basis("branch-a")),
    ))
    .lower()
    .expect("bridge writeback should lower")
    .execute_with(EffectExecutionAuthority::bridge(&bridge))
    .expect("bridge writeback should execute");
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        "Effect-backed bridge writeback",
        &[
            "effect_bridge_writeback_subject_v1".to_string(),
            format!("execution:{}", executed.effect_execution_digest()),
            format!("receipt:{}", executed.receipt().receipt_digest()),
        ],
        executed
            .lowered()
            .lowered_effect_execution_plan_digest()
            .to_string(),
        executed.receipt().receipt_digest().to_string(),
    )
}

fn admit_effect(
    raw: RawEffectIntent,
    basis: EffectAuthoringBasis,
) -> crate::effect_lifecycle::AdmittedEffectIntent {
    let normalized =
        normalize_raw_effect_intent(&basis, raw).expect("effect fixture should normalize");
    match evaluate_effect_eligibility(normalized) {
        EffectEligibilityOutcome::Admitted(eligibility) => admit_effect_intent(eligibility),
        other => panic!("expected admitted effect fixture, got {other:?}"),
    }
}

fn branch_mutation_basis(
    branch_identity: &str,
) -> crate::basis_lifecycle::ScopedMutationPreparationBasis {
    let normalized = normalize_raw_basis_intent(
        RawBasisIntent::BranchHead {
            branch_identity: branch_identity.to_string(),
            accessible: true,
        },
        <MutationPreparationLaneWitness as BasisOperationLane>::lane_name(),
    )
    .expect("branch basis should normalize");
    let eligibility = evaluate_basis_mutation_preparation_eligibility(normalized)
        .expect("branch basis should admit");
    scope_basis_for_mutation_preparation(admit_basis_capability(eligibility))
}

fn runtime_workflow_binding() -> crate::workflow::WorkflowContextBinding {
    runtime_workflow_binding_with_snapshot("snapshot-1")
}

fn runtime_workflow_binding_with_snapshot(
    snapshot_token: &str,
) -> crate::workflow::WorkflowContextBinding {
    synthetic_runtime_workflow_binding("lower-runtime-effect-phase-six", snapshot_token.to_string())
}

fn workflow_request(
    family: WorkflowDeclarationFamily,
    authority_target_family: WorkflowAuthorityTargetFamily,
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
        WorkflowFreshnessPolicy::ExactBasis,
    )
}

fn route_planned_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    owner: ForgeQueryLowerRuntimeAuthorityOwner,
    capability_label: &str,
    subject_parts: &[String],
    support_label: String,
    retained_evidence_digest: String,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        owner,
        capability_label,
        hash_parts(subject_parts),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        retained_evidence_digest.clone(),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(eligibility.clone(), support_label);
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        retained_evidence_digest.clone(),
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam_key,
        &route_plan,
        &boundary_receipt,
        &retained_evidence_digest,
    );
    RepresentativeArtifacts {
        seam_key,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}
