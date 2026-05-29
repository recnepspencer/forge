use crate::facade::{
    certify_milestone_three_closeout, milestone_one_runtime_builder,
    milestone_three_closeout_requirements, CertificationRequiredOutput,
    MilestoneThreeEditFalloutClass, MilestoneThreeHostileScenario,
    TopologyEditDerivedFallbackPolicy, TopologyEditRejectionClass,
};

#[test]
fn milestone_three_closeout_requires_derived_fallback_policy_denial_rows() {
    let requirements = milestone_three_closeout_requirements();
    let report = certify_milestone_three_closeout(
        || {
            milestone_one_runtime_builder()
                .expect(" milestone one runtime builder")
                .build()
        },
        "milestone-three-derived-fallback-policy-denial",
    )
    .expect("milestone three closeout should certify");

    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::MilestoneThreeDerivedFallbackPolicyDenialRows));
    assert!(!report.derived_fallback_policy_denial_rows.is_empty());
    assert!(report
        .derived_fallback_policy_denial_rows
        .iter()
        .all(|row| {
            row.strict_fallback_policy() == TopologyEditDerivedFallbackPolicy::RejectAnyFallback
                && row.policy_exceeded()
                && row.observed_fallback_count() > 0
                && row.denied_rejection_class()
                    == TopologyEditRejectionClass::DerivedFallbackExceeded
                && row
                    .row_digest()
                    .starts_with(&format!("scenario={};", row.scenario().as_str()))
        }));
    assert!(report
        .derived_fallback_policy_denial_rows
        .iter()
        .any(|row| {
            row.scenario() == MilestoneThreeHostileScenario::SplitCollapseChurn
                && row.observed_fallout_class() == MilestoneThreeEditFalloutClass::WholeViewFallback
        }));
}
