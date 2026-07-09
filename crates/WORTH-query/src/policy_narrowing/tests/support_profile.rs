use crate::policy_narrowing::{
    runtime_backed_policy_narrowing_support_profile, PolicyNarrowingSupportStatus,
    PolicyNarrowingSurface,
};

#[test]
fn support_profile_marks_execution_verified_while_live_diff_and_delivery_stay_deferred() {
    let profile = runtime_backed_policy_narrowing_support_profile();

    assert!(profile.surfaces().contains(&(
        PolicyNarrowingSurface::NarrowedPolicyQueryArtifact,
        PolicyNarrowingSupportStatus::Verified
    )));
    assert!(profile.surfaces().contains(&(
        PolicyNarrowingSurface::PolicyAwareExecution,
        PolicyNarrowingSupportStatus::Verified
    )));
    assert!(profile.surfaces().contains(&(
        PolicyNarrowingSurface::PolicyAwareLive,
        PolicyNarrowingSupportStatus::Deferred
    )));
    assert!(profile.surfaces().contains(&(
        PolicyNarrowingSurface::PolicyAwareHistoricalDiff,
        PolicyNarrowingSupportStatus::Deferred
    )));
    assert!(profile.surfaces().contains(&(
        PolicyNarrowingSurface::StoreBackedDurability,
        PolicyNarrowingSupportStatus::BlockedOnWORTHStore
    )));
    assert!(!profile.profile_digest().is_empty());
}
