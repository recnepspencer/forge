use worth_spatial::facade::arbitration::{
    analyze_spatial_intent_conflict_with_capabilities_and_profile, SpatialAuthoredActKind,
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
    let continuity = analysis.identity_continuity_assessment();

    assert_eq!(
        continuity.continuity_class(),
        SpatialIdentityContinuityClass::IdentityReinterpreted
    );
    assert!(continuity.preserves_subject_identity());
    assert!(!continuity.preserves_anchor_identity());
}
