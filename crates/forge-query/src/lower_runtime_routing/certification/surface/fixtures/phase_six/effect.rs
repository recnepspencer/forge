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
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeRoutePlan, ForgeQueryLowerRuntimeSeamKey,
};
use crate::workflow::{
    synthetic_runtime_workflow_binding_for_snapshot_identity, MergeLoweringInput,
    MutationLoweringInput, WorkflowAuthorityTargetFamily, WorkflowBudgetClass, WorkflowCostClass,
    WorkflowDeclarationFamily, WorkflowDeclarationRequest, WorkflowFreshnessPolicy,
};

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};
use super::effect_support::{
    create_entity, exact_branch_snapshot_identity, relational_runtime_with_intent_strategy,
    test_bridge_with_writeback_authority,
};

pub(crate) fn representative_effect_relational_mutation_row() -> RepresentativeArtifacts {
    let mut runtime = relational_runtime_with_intent_strategy();
    let entity_id = create_entity(&mut runtime, "before", BranchId("main".to_string()));
    let raw = RawEffectIntent::Mutation {
        binding: runtime_workflow_binding_for_snapshot_identity(exact_branch_snapshot_identity(
            &runtime, "main",
        )),
        request: workflow_request(
            WorkflowDeclarationFamily::MutationLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMutation,
        ),
        input: MutationLoweringInput::IntentReconciliation {
            entity_id,
            desired_aspect_fields_external_json: serde_json::json!({ "name": "phase6-relational-mutation" }),
        },
    };
    let executed = scope_admitted_effect_plan(admit_effect(
        raw,
        EffectAuthoringBasis::from(branch_mutation_basis("main")),
    ))
    .lower()
    .expect("relational mutation should lower")
    .execute_with(EffectExecutionAuthority::relational(&mut runtime))
    .expect("relational mutation should execute");
    route_planned_row(
        ForgeQueryLowerRuntimeSeamKey::EffectBackedRelationalMutation,
        ForgeQueryLowerRuntimeAuthorityOwner::Relational,
        "Effect-backed relational mutation",
        effect_execution_evidence_identity(
            executed.effect_execution_digest(),
            executed.lowered().lowered_effect_execution_plan_digest(),
            executed.receipt().receipt_digest(),
        ),
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
        effect_execution_evidence_identity(
            executed.effect_execution_digest(),
            executed.lowered().lowered_effect_execution_plan_digest(),
            executed.receipt().receipt_digest(),
        ),
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
        effect_execution_evidence_identity(
            executed.effect_execution_digest(),
            executed.lowered().lowered_effect_execution_plan_digest(),
            executed.receipt().receipt_digest(),
        ),
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
    synthetic_runtime_workflow_binding_for_snapshot_identity(
        "lower-runtime-effect-phase-six",
        crate::memory_workspace::ForgeQuerySnapshotIdentity::from_external_authority_label(
            "snapshot-1",
        ),
    )
}

fn runtime_workflow_binding_for_snapshot_identity(
    snapshot_identity: crate::memory_workspace::ForgeQuerySnapshotIdentity,
) -> crate::workflow::WorkflowContextBinding {
    synthetic_runtime_workflow_binding_for_snapshot_identity(
        "lower-runtime-effect-phase-six",
        snapshot_identity,
    )
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

fn effect_execution_evidence_identity(
    execution_digest: &str,
    lowered_plan_digest: &str,
    receipt_digest: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_identity(ForgeQueryEvidenceTag::new("execution"), execution_digest)
        .field_identity(
            ForgeQueryEvidenceTag::new("lowered_plan"),
            lowered_plan_digest,
        )
        .field_identity(ForgeQueryEvidenceTag::new("receipt"), receipt_digest)
        .seal()
}

fn route_planned_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    owner: ForgeQueryLowerRuntimeAuthorityOwner,
    capability_label: &str,
    evidence: ForgeQueryEvidenceIdentity,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        owner,
        capability_label,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "phase-six-effect-route-subject",
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("effect"), &evidence)
        .seal(),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request.clone(),
        &evidence,
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "phase-six-effect-route",
            &evidence,
        ),
    );
    let retained_evidence_identity =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "phase-six-effect-route",
            &evidence,
        );
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence_identity,
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        seam_key,
        &route_plan,
        &boundary_receipt,
        &retained_evidence_identity,
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
