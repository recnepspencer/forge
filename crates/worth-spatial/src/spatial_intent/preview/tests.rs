use super::{
    prepare_spatial_intent_preview_with_capabilities_and_profile,
    prepare_spatial_intent_preview_with_profile, SpatialIntentPreviewCommitDisposition,
    SpatialIntentPreviewWarning,
};
use crate::spatial_intent::arbitration::{
    SpatialAuthoredActKind, SpatialIntentCapabilitySet, SpatialObservedRelationFact,
};
use crate::spatial_intent::resolution::{SpatialIntentPolicyProfile, SpatialPreviewRichness};

#[test]
fn conservative_preview_keeps_grazing_contact_in_clarification_posture() {
    let preview = prepare_spatial_intent_preview_with_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    );

    assert_eq!(
        preview.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldRequireClarification
    );
    assert!(preview
        .warnings()
        .contains(&SpatialIntentPreviewWarning::ClarificationRequired));
}

#[test]
fn aggressive_snap_preview_makes_profile_driven_auto_resolve_explicit() {
    let preview = prepare_spatial_intent_preview_with_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentPolicyProfile::aggressive_snap(),
    );

    assert_eq!(
        preview.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            crate::spatial_intent::arbitration::SpatialIntentCandidate::SnapFlush
        )
    );
    assert!(preview
        .warnings()
        .contains(&SpatialIntentPreviewWarning::ProfileDrivenAutoResolve(
            crate::spatial_intent::arbitration::SpatialIntentCandidate::SnapFlush
        )));
}

#[test]
fn host_friendly_preview_can_auto_resolve_attach_when_capability_exists() {
    let preview = prepare_spatial_intent_preview_with_capabilities_and_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::HostFaceContact],
        SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
        SpatialIntentPolicyProfile::bim_host_friendly(),
    );

    assert_eq!(
        preview.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            crate::spatial_intent::arbitration::SpatialIntentCandidate::AttachRelationally
        )
    );
}

#[test]
fn high_fidelity_profile_makes_preview_richness_explicit() {
    let preview = prepare_spatial_intent_preview_with_profile(
        SpatialAuthoredActKind::Align,
        &[SpatialObservedRelationFact::FrameAligned],
        SpatialIntentPolicyProfile::high_fidelity_preview(),
    );

    assert_eq!(
        preview.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert!(preview
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
}
