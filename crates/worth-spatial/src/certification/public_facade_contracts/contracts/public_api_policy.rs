use worth_spatial::certification::policy_support::{
    SpatialArbitrationConflict, SpatialArbitrationPolicyProfile, SpatialArbitrationPosture,
    SpatialPreviewRichness,
};

#[test]
fn spatial_certification_namespace_exports_policy_profile_artifacts_for_proof_lanes() {
    let profile = SpatialArbitrationPolicyProfile::bim_host_friendly().derive(
        worth_spatial::certification::policy_support::SpatialArbitrationPolicyProfileOverride::new(
        )
        .with_name("bim_host_friendly_high_fidelity_ask_first")
        .with_arbitration_posture(SpatialArbitrationPosture::AskFirst)
        .with_preview_richness(SpatialPreviewRichness::HighFidelity),
    );

    assert_eq!(profile.name(), "bim_host_friendly_high_fidelity_ask_first");
    assert_eq!(
        profile.arbitration_posture(),
        SpatialArbitrationPosture::AskFirst
    );
    assert_eq!(
        profile.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
}

#[test]
fn spatial_certification_namespace_exports_policy_conflict_artifacts_without_reopening_runtime_facade(
) {
    let conflict = SpatialArbitrationConflict::analyze_with_capabilities_and_profile(
        worth_spatial::certification::policy_support::SpatialAuthoredActKind::Move,
        &[worth_spatial::certification::policy_support::SpatialObservedRelationFact::HostFaceContact],
        worth_spatial::certification::policy_support::SpatialArbitrationCapabilitySet::blocked_defaults()
            .with_host_attach(),
        SpatialArbitrationPolicyProfile::bim_host_friendly(),
    );

    assert_eq!(
        conflict.analysis().policy_profile_name(),
        "bim_host_friendly"
    );
}
