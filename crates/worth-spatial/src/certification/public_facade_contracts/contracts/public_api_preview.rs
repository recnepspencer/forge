use worth_spatial::facade::arbitration::{
    analyze_spatial_intent_conflict_with_capabilities_and_profile, SpatialAuthoredActKind,
    SpatialIntentCandidate, SpatialIntentCapabilitySet, SpatialIntentPolicyProfile,
    SpatialIntentPreviewCommitDisposition, SpatialObservedRelationFact, SpatialPreviewRichness,
};

#[test]
fn spatial_public_facade_exports_preview_and_policy_profile_surface() {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::HostFaceContact],
        SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
        SpatialIntentPolicyProfile::bim_host_friendly(),
    );

    assert_eq!(analysis.policy_profile_name(), "bim_host_friendly");
    assert_eq!(
        analysis.preview_commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            SpatialIntentCandidate::AttachRelationally
        )
    );
    assert_eq!(
        analysis.preview_richness(),
        SpatialPreviewRichness::Standard
    );
    assert_eq!(
        analysis
            .to_query_eligibility()
            .expect("preview-facing analysis should admit query handoff")
            .request()
            .runtime_declaration()
            .expect("runtime declaration")
            .name(),
        "worth.spatial.arbitration"
    );
}
