use worth_spatial::facade::{
    analyze_spatial_intent_conflict_with_capabilities_and_profile,
    assess_spatial_identity_continuity_from_analysis, SpatialAuthoredActKind,
    SpatialIdentityContinuityClass, SpatialIntentCapabilitySet, SpatialIntentPolicyProfile,
    SpatialObservedRelationFact,
};

#[test]
fn spatial_public_facade_exports_identity_continuity_surface() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::HostFaceContact],
        SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
        SpatialIntentPolicyProfile::bim_host_friendly(),
    );
    let continuity = assess_spatial_identity_continuity_from_analysis(&analysis);

    assert_eq!(
        continuity.continuity_class(),
        SpatialIdentityContinuityClass::IdentityReinterpreted
    );
    assert!(continuity.preserves_subject_identity());
    assert!(!continuity.preserves_anchor_identity());
}
