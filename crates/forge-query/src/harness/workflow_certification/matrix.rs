use super::lane::{
    WorkflowCertificationLane, WorkflowCertificationMatrix, WorkflowCertificationRejection,
    WorkflowPerturbationClass,
};
use super::row_catalog::{
    WorkflowCanonicalRowSpec, WorkflowRejectionRowSpec, WORKFLOW_CANONICAL_ROW_SPECS,
    WORKFLOW_REJECTION_ROW_SPECS,
};
use crate::harness::certification::{
    CanonicalCertificationRow, ParityAnchor, RejectionCertificationRow,
};
use crate::harness::fixtures::{
    execution_preflights,
    preview_bridge::active_preview_artifacts,
    relational_merge_inspection::{
        deleted_vs_modified_inspection_artifact, source_addition_inspection_artifact,
        topology_region_conflict_inspection_artifact,
    },
};
use crate::preview::{
    admit_preview_workflow_foundation, bind_preflight_to_preview_session,
    execute_read_only_preview_session_plan, PreviewEvaluationClass, PreviewSessionQueryContext,
};
use crate::workflow::{
    admit_query_workflow_declaration, bind_workflow_context, build_workflow_replay_bundle,
    inspect_merge_conflicts, inspect_post_merge_outcome, lower_merge_workflow_declaration,
    lower_mutation_intent_declaration, lower_query_writeback_declaration,
    shape_merge_authority_outcome, WorkflowAuthorityTargetFamily, WorkflowBindingSource,
    WorkflowBudgetClass, WorkflowCostClass, WorkflowDeclarationFamily, WorkflowDeclarationRequest,
    WorkflowFreshnessPolicy,
};
use crate::workflow::{MergeLoweringInput, MutationLoweringInput, WritebackLoweringInput};
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, PartitionId};
use serde_json::json;

pub struct MilestoneFivePointFiveWorkflowCertificationAdapter;

impl MilestoneFivePointFiveWorkflowCertificationAdapter {
    pub fn workflow_declaration_taxonomy_and_context_binding_test() -> WorkflowCertificationMatrix {
        let runtime_conflict = runtime_conflict_lane();
        let runtime_merge = runtime_merge_lane(WorkflowBudgetClass::AuthorityTargetBounded);
        let runtime_merge_alt_budget = runtime_merge_lane(WorkflowBudgetClass::InspectionBounded);
        let runtime_mutation = runtime_mutation_lane();
        let preview_foundation = preview_foundation_lane();
        let merge_lowering = merge_lowering_lane();
        let writeback_lowering = writeback_lowering_lane();
        let conflict_inspection = conflict_inspection_lane();
        let denied_conflict_inspection = denied_conflict_inspection_lane();
        let post_merge_inspection = post_merge_inspection_lane();
        let preview_merge_lowering = preview_merge_lowering_lane();

        WorkflowCertificationMatrix {
            suite_name: "Query Workflow Lowering And Writeback Boundary Test",
            rows: WORKFLOW_CANONICAL_ROW_SPECS
                .iter()
                .map(|spec| {
                    canonical_row(
                        spec,
                        &runtime_conflict,
                        &runtime_merge,
                        &runtime_merge_alt_budget,
                        &runtime_mutation,
                        &preview_foundation,
                        &merge_lowering,
                        &writeback_lowering,
                        &conflict_inspection,
                        &denied_conflict_inspection,
                        &post_merge_inspection,
                        &preview_merge_lowering,
                    )
                })
                .collect(),
            rejection_rows: WORKFLOW_REJECTION_ROW_SPECS
                .iter()
                .map(rejection_row)
                .collect(),
        }
    }
}

fn runtime_conflict_lane() -> WorkflowCertificationLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection,
            WorkflowCostClass::InspectionNarrow,
            WorkflowBudgetClass::InspectionBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("runtime conflict declaration should admit");
    WorkflowCertificationLane::from_declaration(&declaration)
}

fn runtime_merge_lane(budget_class: WorkflowBudgetClass) -> WorkflowCertificationLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let binding = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime binding should admit");
    let declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            budget_class,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("runtime merge declaration should admit");
    WorkflowCertificationLane::from_declaration(&declaration)
}

fn runtime_mutation_lane() -> WorkflowCertificationLane {
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
    .expect("runtime mutation declaration should admit");
    WorkflowCertificationLane::from_declaration(&declaration)
}

fn preview_foundation_lane() -> WorkflowCertificationLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("workflow-certification-preview");
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
    let workflow_binding =
        bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
            .expect("preview workflow binding should admit");
    let declaration = admit_query_workflow_declaration(
        &workflow_binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::ConflictInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection,
            WorkflowCostClass::InspectionNarrow,
            WorkflowBudgetClass::InspectionBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("preview inspection declaration should admit");
    WorkflowCertificationLane::from_declaration(&declaration)
}

fn merge_lowering_lane() -> WorkflowCertificationLane {
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
            WorkflowFreshnessPolicy::AllowExplicitRebind,
        ),
    )
    .expect("runtime merge declaration should admit");
    let lowered = lower_merge_workflow_declaration(
        &declaration,
        MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    )
    .expect("merge lowering should succeed");
    let outcome = shape_merge_authority_outcome(&lowered);
    let replay = build_workflow_replay_bundle(&outcome);

    let mut lane = WorkflowCertificationLane::from_declaration(&declaration);
    lane.lowered_request_digest = Some(lowered.lowering_digest().to_string());
    lane.lowered_freshness_binding = Some(lowered.freshness_binding().as_str().to_string());
    lane.authority_outcome_family = Some(outcome.family().as_str().to_string());
    lane.replay_bundle_digest = Some(replay.bundle_digest().to_string());
    lane.result_digest = outcome.authoritative_outcome_digest().to_string();
    lane.delivery_digest = replay.delivery_or_failure_digest().to_string();
    lane.prediction_drift_outcome = Some(outcome.prediction_drift_outcome().as_str().to_string());
    lane.budget_outcome = Some(outcome.budget_outcome().as_str().to_string());
    lane.predicted_declaration_width =
        Some(outcome.prediction_report().predicted_declaration_width());
    lane.predicted_lowering_width = Some(outcome.prediction_report().predicted_lowering_width());
    lane.realized_width = Some(outcome.realized_width());
    lane.lowering_width = Some(lowered.counters().workflow_lowering_width());
    lane.lowering_executor_rediscovery_count =
        Some(lowered.counters().workflow_executor_rediscovery_count());
    lane.replay_executor_rediscovery_count =
        Some(replay.counters().workflow_executor_rediscovery_count());
    lane
}

fn writeback_lowering_lane() -> WorkflowCertificationLane {
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
    .expect("runtime writeback declaration should admit");
    let lowered = lower_query_writeback_declaration(
        &declaration,
        WritebackLoweringInput::projected_state_diff(),
    )
    .expect("writeback lowering should succeed");
    let outcome = crate::workflow::shape_writeback_authority_outcome(&lowered);
    let replay = build_workflow_replay_bundle(&outcome);

    let mut lane = WorkflowCertificationLane::from_declaration(&declaration);
    lane.lowered_request_digest = Some(lowered.lowering_digest().to_string());
    lane.lowered_freshness_binding = Some(lowered.freshness_binding().as_str().to_string());
    lane.authority_outcome_family = Some(outcome.family().as_str().to_string());
    lane.replay_bundle_digest = Some(replay.bundle_digest().to_string());
    lane.result_digest = outcome.authoritative_outcome_digest().to_string();
    lane.delivery_digest = replay.delivery_or_failure_digest().to_string();
    lane.prediction_drift_outcome = Some(outcome.prediction_drift_outcome().as_str().to_string());
    lane.budget_outcome = Some(outcome.budget_outcome().as_str().to_string());
    lane.predicted_declaration_width =
        Some(outcome.prediction_report().predicted_declaration_width());
    lane.predicted_lowering_width = Some(outcome.prediction_report().predicted_lowering_width());
    lane.realized_width = Some(outcome.realized_width());
    lane.lowering_width = Some(lowered.counters().workflow_lowering_width());
    lane.lowering_executor_rediscovery_count =
        Some(lowered.counters().workflow_executor_rediscovery_count());
    lane.replay_executor_rediscovery_count =
        Some(replay.counters().workflow_executor_rediscovery_count());
    lane
}

fn preview_merge_lowering_lane() -> WorkflowCertificationLane {
    let preflight = execution_preflights::direct_runtime_preflight();
    let (_runtime, active, execution_record) =
        active_preview_artifacts("workflow-certification-preview-merge");
    let binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("preview merge binding should succeed");
    let execution = crate::preview::execute_promotion_eligible_preview_session_plan(
        &crate::preview::admit_promotion_eligible_preview_session_plan_binding(binding)
            .expect("promotion binding should admit"),
    )
    .expect("promotion execution should succeed");
    let candidate_execution = crate::execution::execute_preflight_bundle(&preflight)
        .expect("candidate execution should succeed");
    let candidate = crate::preview::admit_authoritative_preview_comparison_candidate(
        &preflight,
        &candidate_execution,
    )
    .expect("candidate should admit");
    let comparison =
        crate::preview::admit_preview_promotion_parity_comparison(&execution, &candidate)
            .expect("promotion parity should admit");
    let workflow_binding = bind_workflow_context(
        WorkflowBindingSource::PreviewPromotionComparison(&comparison),
    )
    .expect("promotion comparison should bind");
    let declaration = admit_query_workflow_declaration(
        &workflow_binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::MergeLoweringNarrow,
            WorkflowAuthorityTargetFamily::RelationalMerge,
            WorkflowCostClass::MergeLoweringNarrow,
            WorkflowBudgetClass::AuthorityTargetBounded,
            WorkflowFreshnessPolicy::AllowExplicitRebind,
        ),
    )
    .expect("preview merge declaration should admit");
    let lowered = lower_merge_workflow_declaration(
        &declaration,
        MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    )
    .expect("preview merge lowering should succeed");

    let mut lane = WorkflowCertificationLane::from_declaration(&declaration);
    lane.lowered_request_digest = Some(lowered.lowering_digest().to_string());
    lane.lowered_freshness_binding = Some(lowered.freshness_binding().as_str().to_string());
    lane.result_digest = lowered.lowering_digest().to_string();
    lane.delivery_digest = lowered.lowering_digest().to_string();
    lane.lowering_width = Some(lowered.counters().workflow_lowering_width());
    lane.lowering_executor_rediscovery_count =
        Some(lowered.counters().workflow_executor_rediscovery_count());
    lane
}

fn conflict_inspection_lane() -> WorkflowCertificationLane {
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
    .expect("runtime merge declaration should admit");
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
    let inspection_artifact = source_addition_inspection_artifact();
    let inspection =
        inspect_merge_conflicts(&inspection_declaration, &lowered, &inspection_artifact)
            .expect("inspection should succeed");
    let source_addition_row = inspection
        .rows()
        .iter()
        .find(|row| row.merge_class() == "source_only_addition")
        .expect("source-only addition row should be present");

    let mut lane = WorkflowCertificationLane::from_declaration(&inspection_declaration);
    lane.lowered_request_digest = Some(lowered.lowering_digest().to_string());
    lane.inspection_family = Some(inspection.family().as_str().to_string());
    lane.result_digest = source_addition_row.conflict_scope_digest().to_string();
    lane.delivery_digest = inspection.declaration_digest().to_string();
    lane.prediction_drift_outcome = Some(inspection.drift_outcome().as_str().to_string());
    lane.predicted_declaration_width =
        Some(inspection.prediction_report().predicted_declaration_width());
    lane.predicted_inspection_width =
        Some(inspection.prediction_report().predicted_inspection_width());
    lane.inspection_row_width = Some(inspection.counters().workflow_inspection_row_width());
    lane.inspection_executor_rediscovery_count =
        Some(inspection.counters().workflow_executor_rediscovery_count());
    lane
}

fn denied_conflict_inspection_lane() -> WorkflowCertificationLane {
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
    .expect("runtime merge declaration should admit");
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
    let inspection_artifact = deleted_vs_modified_inspection_artifact();
    let topology_artifact = topology_region_conflict_inspection_artifact();
    let inspection =
        inspect_merge_conflicts(&inspection_declaration, &lowered, &inspection_artifact)
            .expect("inspection should succeed");
    let topology_inspection =
        inspect_merge_conflicts(&inspection_declaration, &lowered, &topology_artifact)
            .expect("topology inspection should succeed");
    let topology_row = topology_inspection
        .rows()
        .iter()
        .find(|row| row.merge_class() == "topology_region_conflict")
        .expect("topology-region row should be present");

    let mut lane = WorkflowCertificationLane::from_declaration(&inspection_declaration);
    lane.lowered_request_digest = Some(lowered.lowering_digest().to_string());
    lane.inspection_family = Some(inspection.family().as_str().to_string());
    lane.result_digest = topology_row.conflict_scope_digest().to_string();
    lane.delivery_digest = inspection.rows()[0].conflict_scope_digest().to_string();
    lane.prediction_drift_outcome = Some(inspection.drift_outcome().as_str().to_string());
    lane.predicted_declaration_width =
        Some(inspection.prediction_report().predicted_declaration_width());
    lane.predicted_inspection_width =
        Some(inspection.prediction_report().predicted_inspection_width());
    lane.inspection_row_width = Some(inspection.counters().workflow_inspection_row_width());
    lane.inspection_executor_rediscovery_count =
        Some(inspection.counters().workflow_executor_rediscovery_count());
    lane
}

fn post_merge_inspection_lane() -> WorkflowCertificationLane {
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
    .expect("runtime merge declaration should admit");
    let lowered = lower_merge_workflow_declaration(
        &declaration,
        MergeLoweringInput::reconcile_into_target(
            BranchId("main".to_string()),
            BranchId("candidate".to_string()),
        ),
    )
    .expect("merge lowering should succeed");
    let outcome = shape_merge_authority_outcome(&lowered);
    let inspection_declaration = admit_query_workflow_declaration(
        &binding,
        WorkflowDeclarationRequest::new(
            WorkflowDeclarationFamily::PostMergeInspectionNarrow,
            WorkflowAuthorityTargetFamily::QueryInspection,
            WorkflowCostClass::InspectionNarrow,
            WorkflowBudgetClass::InspectionBounded,
            WorkflowFreshnessPolicy::ExactBasis,
        ),
    )
    .expect("post-merge inspection declaration should admit");
    let inspection = inspect_post_merge_outcome(&inspection_declaration, &outcome)
        .expect("inspection should succeed");

    let mut lane = WorkflowCertificationLane::from_declaration(&declaration);
    lane.lowered_request_digest = Some(lowered.lowering_digest().to_string());
    lane.authority_outcome_family = Some(outcome.family().as_str().to_string());
    lane.inspection_family = Some(inspection.family().as_str().to_string());
    lane.result_digest = inspection.rows()[0]
        .authoritative_commit_or_outcome_digest()
        .to_string();
    lane.delivery_digest = inspection.origin_digest().to_string();
    lane.prediction_drift_outcome = Some(inspection.drift_outcome().as_str().to_string());
    lane.predicted_declaration_width =
        Some(inspection.prediction_report().predicted_declaration_width());
    lane.predicted_inspection_width =
        Some(inspection.prediction_report().predicted_inspection_width());
    lane.inspection_row_width = Some(inspection.counters().workflow_inspection_row_width());
    lane.inspection_executor_rediscovery_count =
        Some(inspection.counters().workflow_executor_rediscovery_count());
    lane
}

fn canonical_row(
    spec: &WorkflowCanonicalRowSpec,
    runtime_conflict: &WorkflowCertificationLane,
    runtime_merge: &WorkflowCertificationLane,
    runtime_merge_alt_budget: &WorkflowCertificationLane,
    runtime_mutation: &WorkflowCertificationLane,
    preview_foundation: &WorkflowCertificationLane,
    merge_lowering: &WorkflowCertificationLane,
    writeback_lowering: &WorkflowCertificationLane,
    conflict_inspection: &WorkflowCertificationLane,
    denied_conflict_inspection: &WorkflowCertificationLane,
    post_merge_inspection: &WorkflowCertificationLane,
    preview_merge_lowering: &WorkflowCertificationLane,
) -> CanonicalCertificationRow<WorkflowPerturbationClass, WorkflowCertificationLane> {
    let (control_lane, hostile_lane) = match spec.row_name {
        "workflow-declaration-family-explicitness" => {
            (runtime_conflict.clone(), runtime_merge.clone())
        }
        "workflow-basis-family-explicitness" => {
            (runtime_conflict.clone(), preview_foundation.clone())
        }
        "workflow-authority-target-explicitness" => {
            (runtime_merge.clone(), runtime_mutation.clone())
        }
        "workflow-preview-foundation-no-rediscovery" => {
            (preview_foundation.clone(), preview_foundation.clone())
        }
        "workflow-budget-class-explicitness" => {
            (runtime_merge.clone(), runtime_merge_alt_budget.clone())
        }
        "query-authored-mutation-lowering-parity" => {
            (runtime_mutation.clone(), runtime_mutation.clone())
        }
        "query-authored-merge-lowering-parity" => (merge_lowering.clone(), merge_lowering.clone()),
        "query-triggered-writeback-lowering-parity" => {
            (writeback_lowering.clone(), writeback_lowering.clone())
        }
        "conflict-inspection-explicitness" => (merge_lowering.clone(), conflict_inspection.clone()),
        "unsupported-deletion-topology-merge-class" => (
            conflict_inspection.clone(),
            denied_conflict_inspection.clone(),
        ),
        "post-merge-inspection-explicitness" => {
            (merge_lowering.clone(), post_merge_inspection.clone())
        }
        "workflow-freshness-explicitness" => {
            (merge_lowering.clone(), preview_merge_lowering.clone())
        }
        "workflow-prediction-width-explicitness" => {
            (runtime_merge.clone(), conflict_inspection.clone())
        }
        "workflow-realized-width-explicitness" => (runtime_merge.clone(), merge_lowering.clone()),
        "workflow-rediscovery-zero-parity" => (merge_lowering.clone(), merge_lowering.clone()),
        other => panic!("unexpected workflow canonical row {other}"),
    };

    CanonicalCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        hostile_expectation: spec.hostile_expectation,
        parity_anchor: ParityAnchor::Control,
        control_lane: control_lane.clone(),
        hostile_lane,
        parity_lane: control_lane,
    }
}

fn rejection_row(
    spec: &WorkflowRejectionRowSpec,
) -> RejectionCertificationRow<
    WorkflowPerturbationClass,
    WorkflowCertificationLane,
    WorkflowCertificationRejection,
> {
    let control_lane = runtime_conflict_lane();
    let parity_lane = preview_foundation_lane();
    let hostile_lane = match spec.row_name {
        "unsupported-workflow-family" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let (_runtime, active, execution_record) =
                active_preview_artifacts("workflow-certification-unsupported-post-merge");
            let preview_binding = bind_preflight_to_preview_session(
                preflight,
                PreviewSessionQueryContext::active(
                    &active,
                    &execution_record,
                    PreviewEvaluationClass::promotion_eligible(),
                ),
            )
            .expect("preview binding should succeed");
            let foundation =
                admit_preview_workflow_foundation(&preview_binding).expect("preview should admit");
            let workflow_binding =
                bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
                    .expect("preview workflow binding should admit");
            let error = admit_query_workflow_declaration(
                &workflow_binding,
                WorkflowDeclarationRequest::new(
                    WorkflowDeclarationFamily::PostMergeInspectionNarrow,
                    WorkflowAuthorityTargetFamily::QueryInspection,
                    WorkflowCostClass::InspectionNarrow,
                    WorkflowBudgetClass::InspectionBounded,
                    WorkflowFreshnessPolicy::ExactBasis,
                ),
            )
            .expect_err("preview post-merge inspection should deny");
            WorkflowCertificationRejection::from_error(&error)
        }
        "invalid-basis-pairing" => {
            let preflight = execution_preflights::store_detail_preflight();
            let error = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
                .expect_err("store-backed preflight should deny");
            WorkflowCertificationRejection::from_error(&error)
        }
        "preview-read-only-authority-request-forbidden" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let (_runtime, active, execution_record) =
                active_preview_artifacts("workflow-certification-read-only");
            let preview_binding = bind_preflight_to_preview_session(
                preflight,
                PreviewSessionQueryContext::active(
                    &active,
                    &execution_record,
                    PreviewEvaluationClass::read_only(),
                ),
            )
            .expect("read-only preview binding should succeed");
            let admitted_binding = crate::preview::admit_read_only_preview_session_plan_binding(
                preview_binding.clone(),
            )
            .expect("read-only binding should admit");
            let _execution = execute_read_only_preview_session_plan(&admitted_binding)
                .expect("read-only execution should succeed");
            let foundation = admit_preview_workflow_foundation(&preview_binding)
                .expect("read-only foundation should admit");
            let workflow_binding =
                bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
                    .expect("read-only workflow binding should admit");
            let error = admit_query_workflow_declaration(
                &workflow_binding,
                WorkflowDeclarationRequest::new(
                    WorkflowDeclarationFamily::MergeLoweringNarrow,
                    WorkflowAuthorityTargetFamily::RelationalMerge,
                    WorkflowCostClass::MergeLoweringNarrow,
                    WorkflowBudgetClass::AuthorityTargetBounded,
                    WorkflowFreshnessPolicy::ExactBasis,
                ),
            )
            .expect_err("read-only preview merge intent should deny");
            WorkflowCertificationRejection::from_error(&error)
        }
        "unsupported-authority-target" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let binding =
                bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
                    .expect("runtime binding should admit");
            let error = admit_query_workflow_declaration(
                &binding,
                WorkflowDeclarationRequest::new(
                    WorkflowDeclarationFamily::ConflictInspectionNarrow,
                    WorkflowAuthorityTargetFamily::BridgeWriteback,
                    WorkflowCostClass::InspectionNarrow,
                    WorkflowBudgetClass::InspectionBounded,
                    WorkflowFreshnessPolicy::ExactBasis,
                ),
            )
            .expect_err("unsupported authority target should deny");
            WorkflowCertificationRejection::from_error(&error)
        }
        "forbidden-workflow-broadening" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let binding =
                bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
                    .expect("runtime binding should admit");
            let error = admit_query_workflow_declaration(
                &binding,
                WorkflowDeclarationRequest::new(
                    WorkflowDeclarationFamily::MergeLoweringNarrow,
                    WorkflowAuthorityTargetFamily::RelationalMerge,
                    WorkflowCostClass::MergeLoweringNarrow,
                    WorkflowBudgetClass::CrossBoundaryExpansion,
                    WorkflowFreshnessPolicy::AllowExplicitRebind,
                ),
            )
            .expect_err("cross-boundary expansion should deny");
            WorkflowCertificationRejection::from_error(&error)
        }
        "unsupported-merge-family" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let binding =
                bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
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
            let error = lower_merge_workflow_declaration(
                &declaration,
                MergeLoweringInput::reconcile_into_target(
                    BranchId("main".to_string()),
                    BranchId("candidate".to_string()),
                ),
            )
            .expect_err("writeback declaration should not lower as merge");
            WorkflowCertificationRejection::from_lowering_error(&error)
        }
        "unsupported-writeback-family" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let binding =
                bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
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
            let error = lower_query_writeback_declaration(
                &declaration,
                WritebackLoweringInput::projected_state_diff(),
            )
            .expect_err("merge declaration should not lower as writeback");
            WorkflowCertificationRejection::from_lowering_error(&error)
        }
        "explicit-rebind-required" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let (_runtime, active, execution_record) =
                active_preview_artifacts("workflow-certification-writeback-rebind");
            let preview_binding = bind_preflight_to_preview_session(
                preflight,
                PreviewSessionQueryContext::active(
                    &active,
                    &execution_record,
                    PreviewEvaluationClass::promotion_eligible(),
                ),
            )
            .expect("preview binding should succeed");
            let foundation = admit_preview_workflow_foundation(&preview_binding)
                .expect("preview foundation should admit");
            let workflow_binding =
                bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
                    .expect("preview workflow binding should admit");
            let declaration = admit_query_workflow_declaration(
                &workflow_binding,
                WorkflowDeclarationRequest::new(
                    WorkflowDeclarationFamily::WritebackLoweringNarrow,
                    WorkflowAuthorityTargetFamily::BridgeWriteback,
                    WorkflowCostClass::WritebackLoweringNarrow,
                    WorkflowBudgetClass::AuthorityTargetBounded,
                    WorkflowFreshnessPolicy::AllowExplicitRebind,
                ),
            )
            .expect("writeback declaration should admit");
            let error = lower_query_writeback_declaration(
                &declaration,
                WritebackLoweringInput::projected_state_diff(),
            )
            .expect_err("preview writeback should require rebind");
            WorkflowCertificationRejection::from_lowering_error(&error)
        }
        "stale-workflow-denied" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let (_runtime, active, execution_record) =
                active_preview_artifacts("workflow-certification-writeback-stale");
            let preview_binding = bind_preflight_to_preview_session(
                preflight,
                PreviewSessionQueryContext::active(
                    &active,
                    &execution_record,
                    PreviewEvaluationClass::promotion_eligible(),
                ),
            )
            .expect("preview binding should succeed");
            let foundation = admit_preview_workflow_foundation(&preview_binding)
                .expect("preview foundation should admit");
            let workflow_binding =
                bind_workflow_context(WorkflowBindingSource::PreviewFoundation(&foundation))
                    .expect("preview workflow binding should admit");
            let declaration = admit_query_workflow_declaration(
                &workflow_binding,
                WorkflowDeclarationRequest::new(
                    WorkflowDeclarationFamily::WritebackLoweringNarrow,
                    WorkflowAuthorityTargetFamily::BridgeWriteback,
                    WorkflowCostClass::WritebackLoweringNarrow,
                    WorkflowBudgetClass::AuthorityTargetBounded,
                    WorkflowFreshnessPolicy::ExactBasis,
                ),
            )
            .expect("writeback declaration should admit");
            let error = lower_query_writeback_declaration(
                &declaration,
                WritebackLoweringInput::projected_state_diff(),
            )
            .expect_err("preview writeback with exact-basis freshness should stale-deny");
            WorkflowCertificationRejection::from_lowering_error(&error)
        }
        "post-merge-non-authoritative-outcome-forbidden" => {
            let preflight = execution_preflights::direct_runtime_preflight();
            let binding =
                bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
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
            let lowered = lower_mutation_intent_declaration(
                &declaration,
                binding.basis_digest(),
                MutationLoweringInput::IntentReconciliation {
                    entity_id: EntityId::new(PartitionId(1), 41, 0),
                    desired_aspect_fields_json: json!({"name":"after"}),
                },
            )
            .expect("mutation lowering should succeed");
            let outcome = crate::workflow::shape_mutation_authority_outcome(&lowered);
            let inspection_declaration = admit_query_workflow_declaration(
                &binding,
                WorkflowDeclarationRequest::new(
                    WorkflowDeclarationFamily::PostMergeInspectionNarrow,
                    WorkflowAuthorityTargetFamily::QueryInspection,
                    WorkflowCostClass::InspectionNarrow,
                    WorkflowBudgetClass::InspectionBounded,
                    WorkflowFreshnessPolicy::ExactBasis,
                ),
            )
            .expect("post-merge inspection declaration should admit");
            let error = inspect_post_merge_outcome(&inspection_declaration, &outcome)
                .expect_err("mutation outcome should deny post-merge inspection");
            WorkflowCertificationRejection::from_inspection_error(&error)
        }
        _ => WorkflowCertificationRejection::compile_fail(
            spec.compile_fail_case
                .expect("compile-fail workflow rejection rows must declare a case"),
        ),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}
