use worth_spatial::facade::{
    prepare_spatial_intent_preview_with_capabilities_and_profile, SpatialAuthoredActKind,
    SpatialIntentCapabilitySet, SpatialIntentPolicyProfile, SpatialIntentPreviewCommitDisposition,
    SpatialObservedRelationFact, SpatialPreviewRichness,
};

#[test]
fn spatial_public_facade_exports_preview_and_policy_profile_surface() {
    let preview = prepare_spatial_intent_preview_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::HostFaceContact],
        SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
        SpatialIntentPolicyProfile::bim_host_friendly(),
    );

    assert_eq!(preview.policy_profile().name(), "bim_host_friendly");
    assert_eq!(
        preview.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            worth_spatial::facade::SpatialIntentCandidate::AttachRelationally
        )
    );
    assert_eq!(preview.preview_richness(), SpatialPreviewRichness::Standard);
}
