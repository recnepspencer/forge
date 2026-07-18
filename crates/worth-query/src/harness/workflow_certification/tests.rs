use super::{
    MilestoneFivePointFiveWorkflowCertificationAdapter, WorkflowFailureClass,
    WORKFLOW_REQUIRED_CANONICAL_ROW_NAMES, WORKFLOW_REQUIRED_REJECTION_ROW_NAMES,
};
use crate::aspect_field_authoring::single_native_string_aspect_field_patch;
use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::harness::certification::{milestone_five_point_five_requirements, unmet_required_rows};
use crate::harness::fixtures::execution_preflights;
use crate::harness::fixtures::relational_merge_inspection::deleted_vs_modified_inspection_artifact;
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, lower_merge_workflow_declaration,
    lower_mutation_intent_declaration, lower_query_writeback_declaration, MergeLoweringInput,
    MutationLoweringInput, WorkflowAuthorityTargetFamily, WorkflowBindingSource,
    WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
    WorkflowFreshnessPolicy, WritebackLoweringInput,
};
use worth_relational::facade::commit_strategies::{
    IntentReconciliationInput, StrategyCallerProvenance, StrategyRequestOrigin,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::identity::{EntityId, PartitionId};
use worth_relational::facade::merge::{MergeExecutionRequest, MergeIntent};
use worth_runtime_bridge::facade::{
    BridgeRequestKind, BridgeWritebackDeclaration, BridgeWritebackDeclarationIdentity,
    BridgeWritebackEffectClass, BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass,
    BridgeWritebackRequestMode, BridgeWritebackStrategyClass,
};

#[test]
fn workflow_certification_matrix_covers_required_rows() {
    let matrix =
        MilestoneFivePointFiveWorkflowCertificationAdapter::workflow_declaration_taxonomy_and_context_binding_test();
    let requirements = milestone_five_point_five_requirements();
    assert_eq!(
        requirements.suite_name,
        "Query Workflow Lowering And Writeback Boundary Test"
    );
    assert_eq!(
        unmet_required_rows(
            &matrix,
            WORKFLOW_REQUIRED_CANONICAL_ROW_NAMES,
            WORKFLOW_REQUIRED_REJECTION_ROW_NAMES,
        ),
        Vec::<&'static str>::new()
    );
}

#[test]
fn workflow_certification_lanes_emit_required_verification_outputs() {
    let matrix =
        MilestoneFivePointFiveWorkflowCertificationAdapter::workflow_declaration_taxonomy_and_context_binding_test();

    for row in &matrix.rows {
        for lane in [&row.control_lane, &row.hostile_lane, &row.parity_lane] {
            assert!(
                !lane.query_digest.is_empty(),
                "query digest must be present"
            );
            assert!(!lane.plan_digest.is_empty(), "plan digest must be present");
            assert!(
                !lane.result_digest.is_empty(),
                "result digest must be present"
            );
            assert!(
                !lane.delivery_digest.is_empty(),
                "delivery digest must be present"
            );
            assert!(
                !lane.failure_digest.is_empty(),
                "failure digest must be present"
            );
            assert!(
                !lane.counter_snapshot_digest.is_empty(),
                "counter snapshot digest must be present"
            );
        }
    }

    for row in &matrix.rejection_rows {
        assert!(
            !row.hostile_lane.failure_digest.is_empty(),
            "rejection failure digest must be present"
        );
        assert!(
            !row.hostile_lane.counter_snapshot_digest.is_empty(),
            "rejection counter snapshot digest must be present"
        );
    }
}

#[test]
fn workflow_certification_mutation_lowering_matches_direct_relational_control() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MutationLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMutation,
            WorkflowCostClass::MutationLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("mutation declaration should admit");
    let authority_binding_identity = binding.basis_identity();
    let lowered = lower_mutation_intent_declaration(
        &declaration,
        &authority_binding_identity,
        MutationLoweringInput::IntentReconciliation {
            entity_id: EntityId::new(PartitionId(1), 41, 0),
            desired_aspect_fields: single_native_string_aspect_field_patch("name", "name", "after")
                .expect("name patch should lower"),
        },
    )
    .expect("mutation lowering should succeed");

    let control = IntentReconciliationInput {
        entity_id: EntityId::new(PartitionId(1), 41, 0),
        desired_aspect_fields: single_native_string_aspect_field_patch("name", "name", "after")
            .expect("control field patch"),
    }
    .into_native_canonical_request(StrategyCallerProvenance {
        request_origin: StrategyRequestOrigin::Api,
        actor_identity: Some("worth-query".to_string()),
        correlation_id: Some(declaration.report().declaration_digest().to_string()),
    })
    .expect("control request should encode");

    assert_eq!(
        lowered.strategy_request().strategy_name(),
        control.strategy_name()
    );
    assert_eq!(
        lowered.strategy_request().input_bytes(),
        control.input_bytes()
    );
    assert_eq!(
        lowered.strategy_request().caller_provenance(),
        control.caller_provenance()
    );
}

#[test]
fn workflow_certification_merge_lowering_matches_direct_relational_control() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("merge declaration should admit");
    let lowered = lower_merge_workflow_declaration(
        &declaration,
        MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    )
    .expect("merge lowering should succeed");

    let control = MergeExecutionRequest {
        target_branch: BranchId("main".to_string()),
        source_branch: BranchId("candidate".to_string()),
        merge_intent: MergeIntent::ReconcileIntoTarget,
    };
    assert_eq!(lowered.merge_request(), &control);
}

#[test]
fn workflow_certification_writeback_lowering_matches_direct_bridge_control() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
            WorkflowCostClass::WritebackLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("writeback declaration should admit");
    let lowered = lower_query_writeback_declaration(
        &declaration,
        WritebackLoweringInput::projected_state_diff(),
    )
    .expect("writeback lowering should succeed");

    assert_eq!(
        lowered.bridge_declaration().request_kind(),
        BridgeRequestKind::Authoritative,
    );
    assert_eq!(
        lowered.bridge_declaration().request_mode(),
        BridgeWritebackRequestMode::WritebackCapable,
    );
    assert_eq!(
        lowered.bridge_declaration().family_kind(),
        Some(BridgeWritebackFamilyKind::ProjectedStateDiff),
    );
    assert_eq!(
        lowered.bridge_declaration().effect_class(),
        BridgeWritebackEffectClass::ProjectedStateDiff,
    );
    assert_eq!(
        lowered.bridge_declaration().strategy_class(),
        Some(BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation),
    );
    assert_eq!(
        lowered.bridge_declaration().idempotence_class(),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    assert_eq!(
        lowered.bridge_declaration().digest(),
        BridgeWritebackDeclaration::writeback_capable(
            BridgeWritebackDeclarationIdentity::from_bridge_evidence(
                &WorthQueryEvidenceIdentity::compose(
                    WorthQueryEvidenceScope::WorkflowMutationLowering
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("identity_family"),
                    "workflow_writeback_bridge_declaration_v1",
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("declaration"),
                    declaration.report().declaration_identity(),
                )
                .seal()
                .bridge_external_identity_evidence(),
            ),
            BridgeRequestKind::Authoritative,
            BridgeWritebackFamilyKind::ProjectedStateDiff,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        )
        .digest(),
    );
}

#[test]
fn workflow_certification_conflict_inspection_preserves_lower_authority_merge_class() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("merge declaration should admit");
    let lowered = lower_merge_workflow_declaration(
        &declaration,
        MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    )
    .expect("merge lowering should succeed");
    let inspection_declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection,
            WorkflowCostClass::InspectionNarrow,
            WorkflowBudgetClass::InspectionBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("conflict inspection declaration should admit");

    let inspection = crate::workflow::inspect_merge_conflicts(
        &inspection_declaration,
        &lowered,
        &deleted_vs_modified_inspection_artifact(),
    )
    .expect("inspection should succeed");
    let deletion_row = inspection
        .rows()
        .iter()
        .find(|row| row.merge_class() == "deletion:deleted_vs_modified")
        .expect("deleted-vs-modified row should be present");

    assert_eq!(deletion_row.merge_class(), "deletion:deleted_vs_modified");
    assert_eq!(
        deletion_row.merge_class_admission(),
        &crate::workflow::MergeClassAdmission::ExecutionDenied
    );
}

#[test]
fn workflow_certification_denial_counters_are_exact_and_non_trivial() {
    let matrix =
        MilestoneFivePointFiveWorkflowCertificationAdapter::workflow_declaration_taxonomy_and_context_binding_test();

    let merge_family_denial = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "unsupported-merge-family")
        .expect("merge denial row should exist");
    let writeback_family_denial = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "unsupported-writeback-family")
        .expect("writeback denial row should exist");
    let explicit_rebind = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "explicit-rebind-required")
        .expect("explicit rebind row should exist");
    let stale_denial = matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == "stale-workflow-denied")
        .expect("stale denial row should exist");

    assert_eq!(
        merge_family_denial.hostile_lane.failure_class,
        WorkflowFailureClass::UnsupportedWorkflowFamily
    );
    assert_eq!(
        writeback_family_denial.hostile_lane.failure_class,
        WorkflowFailureClass::UnsupportedWorkflowFamily
    );
    assert_eq!(
        explicit_rebind.hostile_lane.failure_class,
        WorkflowFailureClass::ExplicitRebindRequired
    );
    assert_eq!(
        stale_denial.hostile_lane.failure_class,
        WorkflowFailureClass::StaleWorkflowDenied
    );
    assert_ne!(
        merge_family_denial.hostile_lane.counter_snapshot_digest,
        explicit_rebind.hostile_lane.counter_snapshot_digest
    );
    assert_ne!(
        writeback_family_denial.hostile_lane.counter_snapshot_digest,
        explicit_rebind.hostile_lane.counter_snapshot_digest
    );
    assert_ne!(
        stale_denial.hostile_lane.counter_snapshot_digest,
        explicit_rebind.hostile_lane.counter_snapshot_digest
    );
}

#[test]
fn workflow_certification_lane_specific_counters_are_exercised() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");

    let mutation_declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MutationLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMutation,
            WorkflowCostClass::MutationLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("mutation declaration should admit");
    let authority_binding_identity = binding.basis_identity();
    let mutation_lowered = lower_mutation_intent_declaration(
        &mutation_declaration,
        &authority_binding_identity,
        MutationLoweringInput::IntentReconciliation {
            entity_id: EntityId::new(PartitionId(1), 41, 0),
            desired_aspect_fields: single_native_string_aspect_field_patch("name", "name", "after")
                .expect("name patch should lower"),
        },
    )
    .expect("mutation lowering should succeed");
    assert_eq!(
        mutation_lowered
            .counters()
            .workflow_mutation_lowering_count(),
        1
    );
    assert_eq!(
        mutation_lowered.counters().workflow_merge_lowering_count(),
        0
    );

    let merge_declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("merge declaration should admit");
    let merge_lowered = lower_merge_workflow_declaration(
        &merge_declaration,
        MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    )
    .expect("merge lowering should succeed");
    assert_eq!(merge_lowered.counters().workflow_merge_lowering_count(), 1);
    assert_eq!(
        merge_lowered.counters().workflow_mutation_lowering_count(),
        0
    );

    let inspection_declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection,
            WorkflowCostClass::InspectionNarrow,
            WorkflowBudgetClass::InspectionBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("inspection declaration should admit");
    let inspection = crate::workflow::inspect_merge_conflicts(
        &inspection_declaration,
        &merge_lowered,
        &deleted_vs_modified_inspection_artifact(),
    )
    .expect("inspection should succeed");
    assert_eq!(
        inspection.counters().workflow_conflict_inspection_count(),
        1
    );
    assert_eq!(
        inspection.counters().workflow_post_merge_inspection_count(),
        0
    );

    let writeback_declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::WritebackLoweringNarrow,
            WorkflowAuthorityTargetFamily::BridgeWriteback,
            WorkflowCostClass::WritebackLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("writeback declaration should admit");
    let writeback_lowered = lower_query_writeback_declaration(
        &writeback_declaration,
        WritebackLoweringInput::projected_state_diff(),
    )
    .expect("writeback lowering should succeed");
    assert_eq!(
        writeback_lowered
            .counters()
            .workflow_writeback_declaration_count(),
        1
    );
    assert_eq!(
        writeback_lowered
            .counters()
            .workflow_writeback_causality_binding_count(),
        1
    );
}

#[test]
fn workflow_certification_hostile_rows_are_distinct_when_spec_says_they_must_be() {
    let matrix =
        MilestoneFivePointFiveWorkflowCertificationAdapter::workflow_declaration_taxonomy_and_context_binding_test();

    for row in &matrix.rows {
        match row.row_name {
            "query-authored-mutation-lowering-parity"
            | "query-authored-merge-lowering-parity"
            | "query-triggered-writeback-lowering-parity"
            | "workflow-preview-foundation-no-rediscovery"
            | "workflow-rediscovery-zero-parity" => {
                assert_eq!(
                    row.hostile_lane.result_digest, row.control_lane.result_digest,
                    "row {} should preserve control result digest",
                    row.row_name
                );
            }
            _ => {
                assert_ne!(
                    (
                        row.hostile_lane.result_digest.clone(),
                        row.hostile_lane.delivery_digest.clone(),
                        row.hostile_lane.inspection_family.clone(),
                        row.hostile_lane.authority_outcome_family.clone(),
                    ),
                    (
                        row.control_lane.result_digest.clone(),
                        row.control_lane.delivery_digest.clone(),
                        row.control_lane.inspection_family.clone(),
                        row.control_lane.authority_outcome_family.clone(),
                    ),
                    "row {} should stay distinct from control in at least one verification surface",
                    row.row_name
                );
            }
        }
    }
}
