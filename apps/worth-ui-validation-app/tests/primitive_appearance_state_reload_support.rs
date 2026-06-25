#[path = "primitive_appearance_state_basis_support.rs"]
mod primitive_appearance_state_basis_support;

pub use primitive_appearance_state_basis_support::{
    launch_stable_workbench, prepare_reload_for_edits, PRIMITIVE_SURFACE,
};

use primitive_appearance_state_basis_support::{activate_prepared_reload, resolve_projection};
use worth_ui::facade::{
    WorthUiAuthoredDeltaChangePosture, WorthUiPrimitiveChangedFactEvidenceRow,
    WorthUiPrimitiveProjectionReceipt, WorthUiRuntimeFactFamily, WorthUiSemanticSliceId,
    WorthUiValidationChangedFactMappingReceipt, WorthUiValidationPreparedReload,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

pub fn changed_fact_mapping(
    prepared_reload: &WorthUiValidationPreparedReload,
) -> WorthUiValidationChangedFactMappingReceipt {
    prepared_reload
        .changed_fact_mapping_receipt()
        .cloned()
        .expect("appearance-state edit should produce changed fact mapping")
}

pub fn activate_appearance_state_edits_with_workbench(
    edits: &[ValidationAuthoredReloadEdit],
) -> (
    worth_ui_validation_app::ValidationRuntimeWorkbench,
    WorthUiPrimitiveProjectionReceipt,
) {
    let mut workbench = launch_stable_workbench();
    let prepared_reload = prepare_reload_for_edits(&workbench, edits);
    let mapping = changed_fact_mapping(&prepared_reload);
    activate_prepared_reload(&mut workbench, prepared_reload);
    let projection = resolve_projection(&workbench, Some(&mapping))
        .expect("primitive projection should resolve after appearance-state reload");
    (workbench, projection)
}

pub fn assert_exact_appearance_state_projection_rows(
    rows: &[WorthUiPrimitiveChangedFactEvidenceRow],
) {
    assert_eq!(rows.len(), 2);
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
        &[WorthUiRuntimeFactFamily::AuthoredSurfaceProps],
    );
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::PrimitiveAppearanceState,
        &[
            WorthUiRuntimeFactFamily::PrimitiveAppearanceState,
            WorthUiRuntimeFactFamily::PrimitiveActiveAppearance,
            WorthUiRuntimeFactFamily::PrimitiveConstruction,
        ],
    );
}

fn assert_projection_row(
    rows: &[WorthUiPrimitiveChangedFactEvidenceRow],
    expected_slice: WorthUiSemanticSliceId,
    expected_families: &[WorthUiRuntimeFactFamily],
) {
    let row = rows
        .iter()
        .find(|row| row.semantic_slice() == expected_slice)
        .expect("expected appearance-state projection row");
    assert_eq!(row.subject_surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(
        row.change_posture(),
        WorthUiAuthoredDeltaChangePosture::Changed
    );
    assert_eq!(row.changed_facts().len(), expected_families.len());
    for expected_family in expected_families {
        assert!(
            row.changed_facts().iter().any(|fact| {
                fact.family() == *expected_family && fact.identity() == PRIMITIVE_SURFACE
            }),
            "missing changed fact family {}",
            expected_family.token()
        );
    }
}
