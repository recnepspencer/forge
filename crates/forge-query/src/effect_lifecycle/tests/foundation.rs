use crate::basis_lifecycle::BasisFamily;
use crate::workflow::{
    MergeLoweringInput, MutationLoweringInput, WorkflowAuthorityTargetFamily,
    WorkflowDeclarationFamily, WorkflowFreshnessPolicy, WritebackLoweringInput,
};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, PartitionId};

use crate::effect_lifecycle::{
    admit_effect_intent, discover_effect_lifecycle_support, effect_lifecycle_support_matrix,
    evaluate_effect_eligibility, normalize_raw_effect_intent, DeniedEffectEligibilityKind,
    EffectAuthoringBasis, EffectDeferredNeighborFamily, EffectEligibilityOutcome, EffectFamily,
    EffectIntentDenialKind, EffectLoweredArtifactKind, EffectReceiptArtifactKind,
    EffectSupportCause, EffectSupportPosture, RawEffectIntent,
};

use super::support::{
    branch_mutation_basis, preview_closeout_basis, preview_derived_inspection_advisory,
    preview_workflow_binding, runtime_workflow_binding, tenant_mutation_basis, workflow_request,
};

#[test]
fn mutation_effect_normalizes_and_admits_from_raw_workflow_request() {
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
                entity_id: EntityId::new(PartitionId(1), 7, 0),
                desired_payload: serde_json::json!({ "name": "esther" }),
            },
        },
    )
    .expect("mutation effect should normalize");

    let admitted = match evaluate_effect_eligibility(normalized) {
        EffectEligibilityOutcome::Admitted(eligibility) => admit_effect_intent(eligibility),
        other => panic!("expected admitted effect, got {other:?}"),
    };

    assert_eq!(admitted.normalized().family(), EffectFamily::Mutation);
    assert_eq!(
        admitted.normalized().basis_family(),
        BasisFamily::BranchHead
    );
    assert_eq!(
        admitted
            .normalized()
            .workflow_request()
            .declaration_family(),
        &WorkflowDeclarationFamily::MutationLoweringNarrow
    );
    assert_eq!(
        admitted
            .workflow_declaration()
            .report()
            .authority_target_family(),
        &WorkflowAuthorityTargetFamily::RelationalMutation
    );
    assert!(!admitted.admitted_digest().is_empty());
}

#[test]
fn preview_writeback_authoring_is_real_and_returns_typed_rebind() {
    let normalized = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(preview_closeout_basis()),
        RawEffectIntent::Writeback {
            binding: preview_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowFreshnessPolicy::AllowExplicitRebind,
            ),
            input: WritebackLoweringInput::projected_state_diff(),
        },
    )
    .expect("preview writeback should normalize");

    let discovery =
        discover_effect_lifecycle_support(BasisFamily::Preview, EffectFamily::Writeback);
    let rebind = match evaluate_effect_eligibility(normalized) {
        EffectEligibilityOutcome::RebindRequired(rebind) => rebind,
        other => panic!("expected rebind-required effect, got {other:?}"),
    };

    assert_eq!(
        rebind.denial_kind(),
        DeniedEffectEligibilityKind::PreviewRebindRequired
    );
    assert_eq!(
        rebind.counters().support_lookup_width(),
        discovery.counters().support_lookup_width()
    );
    assert_eq!(
        rebind.counters().effect_support_row_count(),
        discovery.counters().support_lookup_width()
    );
}

#[test]
fn preview_mutation_authoring_is_real_and_returns_typed_rebind() {
    let normalized = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(preview_closeout_basis()),
        RawEffectIntent::Mutation {
            binding: preview_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::MutationLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowFreshnessPolicy::AllowExplicitRebind,
            ),
            input: MutationLoweringInput::IntentReconciliation {
                entity_id: EntityId::new(PartitionId(1), 17, 0),
                desired_payload: serde_json::json!({ "name": "preview-mutation" }),
            },
        },
    )
    .expect("preview mutation should normalize");

    let discovery = discover_effect_lifecycle_support(BasisFamily::Preview, EffectFamily::Mutation);
    let rebind = match evaluate_effect_eligibility(normalized) {
        EffectEligibilityOutcome::RebindRequired(rebind) => rebind,
        other => panic!("expected rebind-required effect, got {other:?}"),
    };

    assert_eq!(
        rebind.denial_kind(),
        DeniedEffectEligibilityKind::PreviewRebindRequired
    );
    assert_eq!(
        rebind.decision_trace().message(),
        "preview-backed mutation must rebind to an authoritative basis before lowering"
    );
    assert_eq!(rebind.decision_trace().cause(), "preview_rebind_required");
    assert_eq!(
        rebind.counters().support_lookup_width(),
        discovery.counters().support_lookup_width()
    );
    assert_eq!(
        rebind.counters().effect_support_row_count(),
        discovery.counters().support_lookup_width()
    );
}

#[test]
fn tenant_scoped_merge_denies_with_same_support_width_as_discovery() {
    let normalized = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(tenant_mutation_basis()),
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

    let discovery =
        discover_effect_lifecycle_support(BasisFamily::TenantScoped, EffectFamily::Merge);
    let denial = match evaluate_effect_eligibility(normalized) {
        EffectEligibilityOutcome::Denied(denial) => denial,
        other => panic!("expected denied effect, got {other:?}"),
    };

    assert_eq!(
        denial.denial_kind(),
        DeniedEffectEligibilityKind::BranchAuthorityRequired
    );
    assert_eq!(denial.counters().denied_count(), 1);
    assert_eq!(denial.decision_trace().cause(), "branch_authority_required");
    assert_eq!(
        denial.counters().support_lookup_width(),
        discovery.counters().support_lookup_width()
    );
    assert_eq!(
        discovery.cause(),
        EffectSupportCause::BranchAuthorityRequired
    );
}

#[test]
fn normalization_rejects_workflow_target_mismatch() {
    let denial = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(branch_mutation_basis()),
        RawEffectIntent::Writeback {
            binding: runtime_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            input: WritebackLoweringInput::projected_state_diff(),
        },
    )
    .expect_err("mismatched workflow target should deny normalization");

    assert_eq!(
        denial.denial_kind(),
        EffectIntentDenialKind::WorkflowAuthorityTargetMismatch
    );
    assert_eq!(denial.counters().workflow_authority_target_check_count(), 1);
}

#[test]
fn normalization_rejects_preview_basis_with_runtime_workflow_binding() {
    let denial = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(preview_closeout_basis()),
        RawEffectIntent::Writeback {
            binding: runtime_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::WritebackLoweringNarrow,
                WorkflowAuthorityTargetFamily::BridgeWriteback,
                WorkflowFreshnessPolicy::AllowExplicitRebind,
            ),
            input: WritebackLoweringInput::projected_state_diff(),
        },
    )
    .expect_err("preview basis must reject runtime-only workflow binding");

    assert_eq!(
        denial.denial_kind(),
        EffectIntentDenialKind::BasisWorkflowBindingMismatch
    );
}

#[test]
fn support_matrix_includes_admitted_rebind_and_deferred_rows() {
    let matrix = effect_lifecycle_support_matrix();

    assert!(matrix.rows().iter().any(|row| {
        row.basis_family() == BasisFamily::CurrentHead
            && row.effect_family() == EffectFamily::Mutation
            && row.posture() == EffectSupportPosture::Admitted
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.basis_family() == BasisFamily::Preview
            && row.effect_family() == EffectFamily::Mutation
            && row.posture() == EffectSupportPosture::RebindRequired
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.basis_family() == BasisFamily::Preview
            && row.effect_family() == EffectFamily::Writeback
            && row.posture() == EffectSupportPosture::RebindRequired
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.basis_family() == BasisFamily::StoreBacked
            && row.effect_family() == EffectFamily::Writeback
            && row.posture() == EffectSupportPosture::Deferred
            && row.cause() == EffectSupportCause::StoreBackedExecutionDeferred
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.basis_family() == BasisFamily::DurableReload
            && row.effect_family() == EffectFamily::Writeback
            && row.posture() == EffectSupportPosture::Deferred
            && row.cause() == EffectSupportCause::DurableReplayDeferred
    }));
}

#[test]
fn support_discovery_and_eligibility_agree_for_deferred_neighbors() {
    let store_backed =
        discover_effect_lifecycle_support(BasisFamily::StoreBacked, EffectFamily::Writeback);
    assert_eq!(store_backed.posture(), EffectSupportPosture::Deferred);
    assert_eq!(
        store_backed.cause(),
        EffectSupportCause::StoreBackedExecutionDeferred
    );

    let durable =
        discover_effect_lifecycle_support(BasisFamily::DurableReload, EffectFamily::Writeback);
    assert_eq!(durable.posture(), EffectSupportPosture::Deferred);
    assert_eq!(durable.cause(), EffectSupportCause::DurableReplayDeferred);
}

#[test]
fn support_discovery_exposes_caller_contract_for_runtime_writeback() {
    let support =
        discover_effect_lifecycle_support(BasisFamily::CurrentHead, EffectFamily::Writeback);

    assert_eq!(support.posture(), EffectSupportPosture::Admitted);
    assert_eq!(
        support.authority_owner(),
        Some(crate::effect_lifecycle::EffectAuthorityOwner::ForgeRuntimeBridge)
    );
    assert_eq!(
        support.supported_lowering(),
        Some(EffectLoweredArtifactKind::QueryWritebackDeclaration)
    );
    assert_eq!(
        support.receipt_family(),
        Some(EffectReceiptArtifactKind::ForgeQueryWriteReceipt)
    );
    assert!(!support.requires_rebind());
    assert_eq!(support.denial_kinds(), &[]);
    assert_eq!(
        support.deferred_neighbors(),
        &[
            EffectDeferredNeighborFamily::StoreBackedExecutionParity,
            EffectDeferredNeighborFamily::DurableReplayAndRestartStableEnvelope,
        ]
    );
}

#[test]
fn support_discovery_exposes_denial_and_rebind_expectations() {
    let denied = discover_effect_lifecycle_support(BasisFamily::TenantScoped, EffectFamily::Merge);
    assert_eq!(denied.posture(), EffectSupportPosture::Denied);
    assert_eq!(
        denied.supported_lowering(),
        Some(EffectLoweredArtifactKind::LoweredMergeWorkflowDeclaration)
    );
    assert_eq!(
        denied.receipt_family(),
        Some(EffectReceiptArtifactKind::ForgeQueryIntentExecution)
    );
    assert_eq!(
        denied.denial_kinds(),
        &[DeniedEffectEligibilityKind::BranchAuthorityRequired]
    );

    let rebind = discover_effect_lifecycle_support(BasisFamily::Preview, EffectFamily::Mutation);
    assert_eq!(rebind.posture(), EffectSupportPosture::RebindRequired);
    assert!(rebind.requires_rebind());
    assert_eq!(
        rebind.denial_kinds(),
        &[DeniedEffectEligibilityKind::PreviewRebindRequired]
    );
    assert_eq!(
        rebind.supported_lowering(),
        Some(EffectLoweredArtifactKind::LoweredMutationIntentDeclaration)
    );
}

#[test]
fn unsupported_support_discovery_fails_closed_without_artifacts() {
    let support = discover_effect_lifecycle_support(BasisFamily::Preview, EffectFamily::Merge);

    assert_eq!(support.posture(), EffectSupportPosture::Unsupported);
    assert_eq!(support.authority_owner(), None);
    assert_eq!(support.supported_lowering(), None);
    assert_eq!(support.receipt_family(), None);
    assert!(!support.requires_rebind());
    assert_eq!(
        support.denial_kinds(),
        &[DeniedEffectEligibilityKind::UnsupportedForBasisFamily]
    );
    assert_eq!(support.deferred_neighbors(), &[]);
}

#[test]
fn preview_derived_mutation_returns_real_advisory_effect_posture() {
    let normalized = normalize_raw_effect_intent(
        &EffectAuthoringBasis::from(preview_derived_inspection_advisory()),
        RawEffectIntent::Mutation {
            binding: runtime_workflow_binding(),
            request: workflow_request(
                WorkflowDeclarationFamily::MutationLoweringNarrow,
                WorkflowAuthorityTargetFamily::RelationalMutation,
                WorkflowFreshnessPolicy::ExactBasis,
            ),
            input: MutationLoweringInput::IntentReconciliation {
                entity_id: EntityId::new(PartitionId(1), 41, 0),
                desired_payload: serde_json::json!({ "name": "advisory-preview-derived" }),
            },
        },
    )
    .expect("preview-derived advisory mutation should normalize");

    let discovery =
        discover_effect_lifecycle_support(BasisFamily::PreviewDerived, EffectFamily::Mutation);
    let advisory = match evaluate_effect_eligibility(normalized) {
        EffectEligibilityOutcome::Advisory(advisory) => advisory,
        other => panic!("expected advisory effect, got {other:?}"),
    };

    assert_eq!(discovery.posture(), EffectSupportPosture::Advisory);
    assert_eq!(discovery.cause(), EffectSupportCause::AdvisoryOnlyExecution);
    assert_eq!(
        advisory.advisory_cause(),
        EffectSupportCause::AdvisoryOnlyExecution
    );
    assert_eq!(
        advisory.normalized().basis_family(),
        BasisFamily::PreviewDerived
    );
    assert_eq!(advisory.normalized().family(), EffectFamily::Mutation);
    assert_eq!(advisory.counters().advisory_count(), 1);
    assert_eq!(advisory.decision_trace().cause(), "advisory_only_execution");
    assert_ne!(
        advisory.normalized().capability_digest(),
        advisory.decision_trace().trace_digest()
    );
}
