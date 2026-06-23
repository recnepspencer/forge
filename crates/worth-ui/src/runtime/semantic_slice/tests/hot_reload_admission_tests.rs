use crate::runtime::{
    WorthUiAdmittedHotReloadableSemanticSliceSet, WorthUiSemanticHotReloadAdmissionDenial,
    WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
};

#[test]
fn hot_reload_admission_accepts_product_meaning_slices() {
    let admitted = WorthUiAdmittedHotReloadableSemanticSliceSet::admit(
        &WorthUiSemanticSliceInventory::current(),
        [
            WorthUiSemanticSliceId::ThemeTokenValue,
            WorthUiSemanticSliceId::SurfaceMountTarget,
            WorthUiSemanticSliceId::ThemeTokenValue,
        ],
    )
    .expect("product-meaning slices should admit into hot-reloadable set");

    let admitted_ids = admitted
        .slices()
        .iter()
        .map(|slice| slice.descriptor().id())
        .collect::<Vec<_>>();
    assert_eq!(
        admitted_ids,
        vec![
            WorthUiSemanticSliceId::ThemeTokenValue,
            WorthUiSemanticSliceId::SurfaceMountTarget,
        ]
    );
}

#[test]
fn hot_reload_admission_rejects_compile_required_platform_slices() {
    let denial = WorthUiAdmittedHotReloadableSemanticSliceSet::admit(
        &WorthUiSemanticSliceInventory::current(),
        [WorthUiSemanticSliceId::NewRustComponentImplementation],
    )
    .expect_err("platform-meaning slices must stay outside hot reload");

    assert_eq!(
        denial,
        WorthUiSemanticHotReloadAdmissionDenial::CompileRequiredPlatformMeaning(
            WorthUiSemanticSliceId::NewRustComponentImplementation
        )
    );
}
