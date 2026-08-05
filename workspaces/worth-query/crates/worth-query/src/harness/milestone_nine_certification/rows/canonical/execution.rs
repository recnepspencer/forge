use crate::harness::certification::HostileExpectation;
use crate::harness::certification::ParityAnchor;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::native_authorized_projection_fields;
use crate::harness::milestone_nine_certification::fixtures::phase_three_bundle;
use crate::harness::milestone_nine_certification::fixtures::phase_three_test_narrowed_artifact;
use crate::harness::milestone_nine_certification::fixtures::policy_execution_handoff_bundle;

pub(super) fn canonical_execution_rows() -> Vec<MilestoneNineCertificationRow> {
    let phase_three_narrowed = phase_three_test_narrowed_artifact();
    let current_plan = crate::policy_plan::lower_policy_aware_current_plan(&phase_three_narrowed);
    let branch_plan = crate::policy_plan::lower_policy_aware_branch_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareReadBasis::admitted_branch(
            phase_three_narrowed.branch_access_digest(),
            "phase3-branch-basis",
        ),
    )
    .unwrap();
    let historical_plan = crate::policy_plan::lower_policy_aware_historical_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareHistoricalBasis::runtime_backed("phase3-historical-basis"),
    )
    .unwrap();
    let diff_plan = crate::policy_plan::lower_policy_aware_diff_plan(
        &phase_three_narrowed,
        crate::policy_plan::PolicyAwareDiffBasisPair::runtime_backed(
            "phase3-left-basis",
            "phase3-right-basis",
        ),
    )
    .unwrap();
    let live_plan = crate::policy_live::admit_policy_aware_live_plan(
        &phase_three_narrowed,
        &native_authorized_projection_fields(&phase_three_narrowed),
        crate::policy_live::PolicyDriftDisposition::NoChange,
        crate::policy_live::PolicyLiveDensityPosture::SparseDelta,
    )
    .unwrap();
    let delivery_shape = crate::policy_delivery::lower_policy_aware_delivery_shape(
        &phase_three_narrowed,
        crate::policy_delivery::DeliveryWidthClass::ScalarDetail,
    )
    .unwrap();
    let optimizer_input =
        crate::policy_plan::lower_policy_aware_optimizer_input(&phase_three_narrowed);
    let phase_three_current = phase_three_bundle(
        "policy-aware-current",
        current_plan.core().digest().as_str(),
        current_plan.core().seam().identity().as_str(),
        "phase3-current-delivery-not-lowered",
    );
    let phase_three_branch = phase_three_bundle(
        "policy-aware-branch",
        branch_plan.core().digest().as_str(),
        branch_plan.core().seam().identity().as_str(),
        "phase3-branch-delivery-not-lowered",
    );
    let phase_three_historical = phase_three_bundle(
        "policy-aware-historical",
        historical_plan.core().digest().as_str(),
        historical_plan.core().seam().identity().as_str(),
        "phase3-historical-delivery-not-lowered",
    );
    let phase_three_diff = phase_three_bundle(
        "policy-aware-diff",
        diff_plan.core().digest().as_str(),
        diff_plan.core().seam().identity().as_str(),
        "phase3-diff-delivery-not-lowered",
    );
    let phase_three_live = phase_three_bundle(
        "policy-aware-live",
        live_plan.core().digest().as_str(),
        live_plan.core().seam().identity().as_str(),
        "phase3-live-delivery-not-lowered",
    );
    let phase_three_delivery = phase_three_bundle(
        "policy-aware-delivery",
        delivery_shape.seam().identity().as_str(),
        delivery_shape.seam().identity().as_str(),
        delivery_shape.digest().as_str(),
    );
    let phase_three_optimizer = phase_three_bundle(
        "policy-aware-optimizer",
        optimizer_input.optimizer_input_digest(),
        "phase3-optimizer-seam-bound-to-narrowed",
        "phase3-optimizer-delivery-not-lowered",
    );
    let phase_three_handoff = policy_execution_handoff_bundle();
    vec![
        MilestoneNineCertificationRow {
            row_name: "policy-aware-current-plan-lowering",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareCurrentPlanLowering,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_current.clone(),
            parity_lane: phase_three_current.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-branch-plan-lowering",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareBranchPlanLowering,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_branch.clone(),
            parity_lane: phase_three_branch,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-historical-plan-runtime-backed-lowering",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareHistoricalPlanLowering,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_historical.clone(),
            parity_lane: phase_three_historical,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-diff-plan-runtime-backed-lowering",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareDiffPlanLowering,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_diff.clone(),
            parity_lane: phase_three_diff,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-live-admission",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareLiveAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_live.clone(),
            parity_lane: phase_three_live,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-delivery-shape-derived-after-mask",
            perturbation_class:
                MilestoneNinePerturbationClass::PolicyAwareDeliveryShapeDerivedAfterMask,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_delivery.clone(),
            parity_lane: phase_three_delivery,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-aware-optimizer-input-only",
            perturbation_class: MilestoneNinePerturbationClass::PolicyAwareOptimizerInputOnly,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_optimizer.clone(),
            parity_lane: phase_three_optimizer,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-execution-seam-parity",
            perturbation_class: MilestoneNinePerturbationClass::PolicyExecutionSeamParity,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_three_current.clone(),
            hostile_lane: phase_three_current.clone(),
            parity_lane: phase_three_current.clone(),
        },
        MilestoneNineCertificationRow {
            row_name: "policy-execution-handoff-honesty",
            perturbation_class: MilestoneNinePerturbationClass::PolicyExecutionHandoffHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: phase_three_handoff.clone(),
            hostile_lane: phase_three_handoff.clone(),
            parity_lane: phase_three_handoff,
        },
    ]
}
