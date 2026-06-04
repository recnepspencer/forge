use worth_kernel::facade::{authoring::policy::*, diagnostics::policy::*};

#[test]
fn kernel_public_facade_exports_policy_profile_report() {
    let report = prepare_primitive_construction_policy_profile_report();
    let row = report
        .row(PrimitiveConstructionPolicyProfileCase::HighFidelityPreview)
        .expect("row");

    assert_eq!(row.preview_richness(), SpatialPreviewRichness::HighFidelity);
}

#[test]
fn kernel_public_facade_exports_policy_profile_direct_reports() {
    let report = prepare_primitive_construction_policy_profile_report();
    let direct = report
        .row(PrimitiveConstructionPolicyProfileCase::AggressiveSnap)
        .expect("direct row");

    assert_eq!(
        direct.arbitration_posture(),
        SpatialArbitrationPosture::PreferSnap
    );
}
