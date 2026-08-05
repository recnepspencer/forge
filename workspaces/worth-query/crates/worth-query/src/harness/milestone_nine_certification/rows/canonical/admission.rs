use crate::harness::certification::HostileExpectation;
use crate::harness::certification::ParityAnchor;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::admitted_bundle;
use crate::harness::milestone_nine_certification::fixtures::saved_query_reuse_bundle;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_basis::SavedQueryPolicyReuseDisposition;

pub(super) fn canonical_admission_rows() -> Vec<MilestoneNineCertificationRow> {
    let current = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let branch = admitted_bundle(PolicyExecutionModeRequest::BranchRead, false);
    let historical = admitted_bundle(PolicyExecutionModeRequest::HistoricalRead, false);
    let narrowed = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, true);
    let bounded = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let exact_reuse =
        saved_query_reuse_bundle(SavedQueryPolicyReuseDisposition::LegalNoSemanticChange);
    let support = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    vec![
        MilestoneNineCertificationRow {
            row_name: "current-read-policy-tenant-admission",
            perturbation_class: MilestoneNinePerturbationClass::CurrentReadAdmission,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: current.clone(),
            hostile_lane: current.clone(),
            parity_lane: current,
        },
        MilestoneNineCertificationRow {
            row_name: "branch-read-policy-tenant-admission",
            perturbation_class: MilestoneNinePerturbationClass::BranchReadAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false),
            hostile_lane: branch.clone(),
            parity_lane: branch,
        },
        MilestoneNineCertificationRow {
            row_name: "historical-read-policy-tenant-admission",
            perturbation_class: MilestoneNinePerturbationClass::HistoricalReadAdmission,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false),
            hostile_lane: historical.clone(),
            parity_lane: historical,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-narrowing-disposition",
            perturbation_class: MilestoneNinePerturbationClass::PolicyNarrowingDisposition,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false),
            hostile_lane: narrowed.clone(),
            parity_lane: narrowed,
        },
        MilestoneNineCertificationRow {
            row_name: "policy-work-budget-explicitness",
            perturbation_class: MilestoneNinePerturbationClass::PolicyWorkBudgetExplicitness,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: bounded.clone(),
            hostile_lane: bounded.clone(),
            parity_lane: bounded,
        },
        MilestoneNineCertificationRow {
            row_name: "saved-query-exact-policy-tenant-reuse",
            perturbation_class: MilestoneNinePerturbationClass::SavedQueryExactReuse,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: exact_reuse.clone(),
            hostile_lane: exact_reuse.clone(),
            parity_lane: exact_reuse,
        },
        MilestoneNineCertificationRow {
            row_name: "support-profile-honesty",
            perturbation_class: MilestoneNinePerturbationClass::SupportProfileHonesty,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: support.clone(),
            hostile_lane: support.clone(),
            parity_lane: support,
        },
    ]
}
