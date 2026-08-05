use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::admitted_bundle;
use crate::harness::milestone_nine_certification::fixtures::authorized_projection_field;
use crate::harness::milestone_nine_certification::fixtures::phase_three_test_narrowed_artifact;
use crate::harness::milestone_nine_certification::fixtures::policy_execution_seam_rejection_bundle;
use crate::policy_basis::PolicyExecutionModeRequest;

pub(super) fn rejection_execution_rows() -> Vec<MilestoneNineRejectionRow> {
    let control = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let phase_three_narrowed = phase_three_test_narrowed_artifact();
    let raw_branch_bypass = crate::policy_plan::lower_policy_aware_branch_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareReadBasis::admitted_branch(
            "wrong-branch-digest",
            "phase3-branch-basis",
        ),
    )
    .unwrap_err();
    let raw_diff_scrub = crate::policy_plan::deny_raw_diff_scrub();
    let masked_live_relevance = crate::policy_live::admit_policy_aware_live_plan(
        &phase_three_narrowed,
        &[authorized_projection_field("secret", "salary")],
        crate::policy_live::PolicyDriftDisposition::NoChange,
        crate::policy_live::PolicyLiveDensityPosture::SparseDelta,
    )
    .unwrap_err();
    let delivery_overexposure = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &phase_three_narrowed,
        crate::policy_delivery::DeliveryWidthClass::DeniedWidthInflation,
    )
    .unwrap_err();
    let store_deferred = crate::policy_plan::lower_policy_aware_historical_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareHistoricalBasis::store_backed_deferred("phase3-store-basis"),
    )
    .unwrap_err();
    let durable_cursor_deferred = crate::policy_execution_seam::deny_durable_policy_cursor_claim();
    let durable_artifact_reload_deferred =
        crate::policy_execution_seam::deny_durable_policy_artifact_reload_claim();
    let durable_delivery_metadata_deferred =
        crate::policy_execution_seam::deny_durable_policy_delivery_metadata_reload_claim();
    vec![
        MilestoneNineRejectionRow {
            row_name: "raw-current-plan-bypass-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::RawCurrentPlanBypassForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(raw_branch_bypass.clone()),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "raw-branch-plan-bypass-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::RawBranchPlanBypassForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(raw_branch_bypass),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "raw-historical-plan-bypass-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::RawHistoricalPlanBypassForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(store_deferred.clone()),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "raw-diff-scrub-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::RawDiffScrubForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(raw_diff_scrub),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-live-relevance-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::MaskedLiveRelevanceForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(masked_live_relevance),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "delivery-shape-overexposure-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::DeliveryShapeOverexposureForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(delivery_overexposure),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "store-backed-policy-execution-deferred",
            perturbation_class: MilestoneNinePerturbationClass::StoreBackedPolicyExecutionDeferred,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(store_deferred),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "durable-policy-cursor-deferred",
            perturbation_class: MilestoneNinePerturbationClass::DurablePolicyCursorDeferred,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(durable_cursor_deferred),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "durable-policy-artifact-reload-deferred",
            perturbation_class: MilestoneNinePerturbationClass::DurablePolicyArtifactReloadDeferred,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(durable_artifact_reload_deferred),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "durable-policy-delivery-metadata-deferred",
            perturbation_class:
                MilestoneNinePerturbationClass::DurablePolicyDeliveryMetadataDeferred,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(
                durable_delivery_metadata_deferred,
            ),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "phase-three-no-truth-touch-before-plan-admission",
            perturbation_class: MilestoneNinePerturbationClass::PhaseThreeNoTruthTouch,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(
                crate::policy_plan::deny_raw_diff_scrub(),
            ),
            parity_lane: control.clone(),
        },
    ]
}
