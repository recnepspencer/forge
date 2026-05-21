use crate::spatial_intent::arbitration::{
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    resolve_spatial_intent_conflict_by_choice, SpatialAuthoredActKind, SpatialIntentCandidate,
    SpatialIntentCapabilitySet, SpatialObservedRelationFact,
};
use crate::spatial_intent::resolution::SpatialIntentPolicyProfile;

use super::{
    assess_spatial_identity_continuity_from_analysis,
    assess_spatial_identity_continuity_from_resolution, SpatialIdentityContinuityClass,
    SpatialIdentityContinuityExplanationClass,
};

#[test]
fn continuity_marks_move_only_as_identity_preserved() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    );
    let continuity = assess_spatial_identity_continuity_from_analysis(&analysis);

    assert_eq!(
        continuity.continuity_class(),
        SpatialIdentityContinuityClass::IdentityPreserved
    );
    assert_eq!(
        continuity.explanation_class(),
        SpatialIdentityContinuityExplanationClass::BaselineIdentityPreserved
    );
}

#[test]
fn continuity_marks_policy_snap_as_anchor_continuity_preserved() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::aggressive_snap(),
    );
    let continuity = assess_spatial_identity_continuity_from_analysis(&analysis);

    assert_eq!(
        continuity.continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
    assert_eq!(
        continuity.candidate(),
        Some(SpatialIntentCandidate::SnapFlush)
    );
}

#[test]
fn continuity_marks_explicit_merge_as_identity_merged() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialIntentCapabilitySet::blocked_defaults().with_merge_boolean(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    );
    let resolution =
        resolve_spatial_intent_conflict_by_choice(analysis, SpatialIntentCandidate::MergeCandidate)
            .expect("merge choice");
    let continuity = assess_spatial_identity_continuity_from_resolution(&resolution);

    assert_eq!(
        continuity.continuity_class(),
        SpatialIdentityContinuityClass::IdentityMerged
    );
}

#[test]
fn continuity_marks_blocked_overlap_as_pending_choice() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::Overlap],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    );
    let continuity = assess_spatial_identity_continuity_from_analysis(&analysis);

    assert_eq!(
        continuity.continuity_class(),
        SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
    );
    assert!(continuity.blocked_capability().is_some());
}
