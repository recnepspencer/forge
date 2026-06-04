use worth_kernel::facade::{
    authoring::{intents::*, policy::*},
    diagnostics::{arbitration::*, continuity::*},
};

#[test]
fn kernel_public_facade_exports_continuity_surface() {
    let report = prepare_primitive_construction_continuity_surface_report().expect("report");
    let row = report
        .row(PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged)
        .expect("row");

    assert_eq!(
        row.continuity_class(),
        SpatialIdentityContinuityClass::IdentityMerged
    );
}

#[test]
fn kernel_public_facade_exports_continuity_preview_inspection() {
    let preview = PrimitiveIntentPreviewAssessment::analyze_with_capabilities(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::aggressive_snap(),
    )
    .continuity()
    .clone();

    assert_eq!(
        preview.continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
}
