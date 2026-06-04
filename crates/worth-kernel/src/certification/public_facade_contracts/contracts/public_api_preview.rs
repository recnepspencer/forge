use worth_kernel::facade::{
    authoring::{intents::*, policy::*},
    diagnostics::{arbitration::*, preview::*},
};

#[test]
fn kernel_public_facade_exports_preview_surface() {
    let report = prepare_primitive_construction_preview_surface_report().expect("report");
    let row = report
        .row(PrimitiveConstructionPreviewCase::GrazingAggressiveSnap)
        .expect("row");

    assert_eq!(row.profile_name(), "aggressive_snap");
}

#[test]
fn kernel_public_facade_exports_preview_assessment_envelope_and_profile_override() {
    let profile = SpatialIntentPolicyProfile::aggressive_snap().derive(
        SpatialIntentPolicyProfileOverride::new()
            .with_name("aggressive_snap_high_fidelity")
            .with_preview_richness(SpatialPreviewRichness::HighFidelity)
            .with_arbitration_posture(SpatialArbitrationPosture::PreferSnap),
    );
    let assessment = PrimitiveIntentPreviewAssessment::analyze(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        profile,
    );

    assert_eq!(assessment.profile().name(), "aggressive_snap_high_fidelity");
    assert_eq!(
        assessment.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(SpatialIntentCandidate::SnapFlush)
    );
    assert_eq!(
        assessment.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert_eq!(
        assessment.continuity().candidate(),
        Some(SpatialIntentCandidate::SnapFlush)
    );
    assert!(assessment.clarification_request().is_err());
    assert!(assessment
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
    assert_eq!(
        assessment.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(SpatialIntentCandidate::SnapFlush)
    );
}
