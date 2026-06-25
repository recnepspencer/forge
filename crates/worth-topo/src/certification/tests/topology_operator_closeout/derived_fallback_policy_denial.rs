use crate::facade::{
    milestone_three_closeout_requirements, CertificationRequiredOutput,
    MilestoneThreeHostileScenario, MilestoneThreeMutationFalloutClass,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationRejectionClass,
};

#[test]
fn milestone_three_closeout_requires_derived_fallback_policy_denial_rows() {
    let requirements = milestone_three_closeout_requirements();
    let report = crate::certification::test_support::cached_milestone_three_closeout_report();

    assert!(requirements
        .required_outputs
        .contains(&CertificationRequiredOutput::MilestoneThreeDerivedFallbackPolicyDenialRows));
    assert!(!report.derived_fallback_policy_denial_rows.is_empty());
    assert!(report
        .derived_fallback_policy_denial_rows
        .iter()
        .all(|row| {
            row.strict_fallback_policy() == TopologyMutationDerivedFallbackPolicy::RejectAnyFallback
                && row.policy_exceeded()
                && row.observed_fallback_count() > 0
                && row.denied_rejection_class()
                    == TopologyMutationRejectionClass::DerivedFallbackExceeded
                && row
                    .row_digest()
                    .starts_with(&format!("scenario={};", row.scenario().as_str()))
        }));
    assert!(report
        .derived_fallback_policy_denial_rows
        .iter()
        .any(|row| {
            row.scenario() == MilestoneThreeHostileScenario::SplitCollapseChurn
                && row.observed_fallout_class()
                    == MilestoneThreeMutationFalloutClass::WholeViewFallback
        }));
}
