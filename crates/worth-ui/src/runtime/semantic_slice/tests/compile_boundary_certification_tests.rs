use crate::runtime::{
    WorthUiCompileBoundaryCertification, WorthUiCompileBoundaryPosture,
    WorthUiSemanticChangedSliceRow, WorthUiSemanticChangedSliceSet, WorthUiSemanticCompileBoundary,
    WorthUiSemanticSliceId, WorthUiSemanticSliceInventory, WorthUiSemanticSliceLoweringCause,
};

#[test]
fn certification_marks_product_meaning_slices_as_hot_reloadable() {
    let inventory = WorthUiSemanticSliceInventory::current();
    let lowered = WorthUiSemanticChangedSliceSet::from_rows(vec![
        changed_slice(
            inventory,
            WorthUiSemanticSliceId::SurfaceMountTarget,
            WorthUiSemanticSliceLoweringCause::ExactRuntimeFactFamily(
                crate::runtime::WorthUiRuntimeFactFamily::SurfaceMount,
            ),
        ),
        changed_slice(
            inventory,
            WorthUiSemanticSliceId::AppearanceField,
            WorthUiSemanticSliceLoweringCause::ExactRuntimeFactFamily(
                crate::runtime::WorthUiRuntimeFactFamily::Appearance,
            ),
        ),
    ]);

    let certification = WorthUiCompileBoundaryCertification::certify(
        &WorthUiSemanticCompileBoundary::current(),
        &lowered,
    );

    assert_eq!(
        certification.posture(),
        WorthUiCompileBoundaryPosture::HotReloadWithinProductMeaning
    );
    assert!(certification.hot_reload_stays_within_product_meaning());
    assert_eq!(certification.compile_required_slice_ids(), &[]);
    assert_eq!(
        certification.hot_reloadable_slice_ids(),
        &[
            WorthUiSemanticSliceId::AppearanceField,
            WorthUiSemanticSliceId::SurfaceMountTarget,
        ]
    );
}

#[test]
fn certification_marks_platform_meaning_slices_as_compile_required() {
    let inventory = WorthUiSemanticSliceInventory::current();
    let lowered = WorthUiSemanticChangedSliceSet::from_rows(vec![
        changed_slice(
            inventory,
            WorthUiSemanticSliceId::SurfaceMountTarget,
            WorthUiSemanticSliceLoweringCause::ExactRuntimeFactFamily(
                crate::runtime::WorthUiRuntimeFactFamily::SurfaceMount,
            ),
        ),
        changed_slice(
            inventory,
            WorthUiSemanticSliceId::NewRustComponentImplementation,
            WorthUiSemanticSliceLoweringCause::CompositeRuntimeFactFamily(
                crate::runtime::WorthUiRuntimeFactFamily::Component,
            ),
        ),
    ]);

    let certification = WorthUiCompileBoundaryCertification::certify(
        &WorthUiSemanticCompileBoundary::current(),
        &lowered,
    );

    assert_eq!(
        certification.posture(),
        WorthUiCompileBoundaryPosture::CompileRequiredPlatformMeaning
    );
    assert!(!certification.hot_reload_stays_within_product_meaning());
    assert_eq!(
        certification.compile_required_slice_ids(),
        &[WorthUiSemanticSliceId::NewRustComponentImplementation]
    );
    assert_eq!(
        certification.hot_reloadable_slice_ids(),
        &[WorthUiSemanticSliceId::SurfaceMountTarget]
    );
}

fn changed_slice(
    inventory: WorthUiSemanticSliceInventory,
    id: WorthUiSemanticSliceId,
    cause: WorthUiSemanticSliceLoweringCause,
) -> WorthUiSemanticChangedSliceRow {
    let descriptor = inventory.slice(id).expect("semantic slice is registered");
    WorthUiSemanticChangedSliceRow::new(descriptor, cause)
}
