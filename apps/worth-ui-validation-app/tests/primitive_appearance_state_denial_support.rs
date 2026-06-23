#[path = "primitive_appearance_state_basis_support.rs"]
mod primitive_appearance_state_basis_support;

pub use primitive_appearance_state_basis_support::PRIMITIVE_SURFACE;

use primitive_appearance_state_basis_support::{
    activate_prepared_reload, launch_stable_workbench, prepare_reload_for_edits, resolve_projection,
};
use worth_ui::facade::WorthUiPrimitiveProofDenial;
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

pub fn appearance_state_denial_for_edits(
    edits: &[ValidationAuthoredReloadEdit],
) -> WorthUiPrimitiveProofDenial {
    let mut workbench = launch_stable_workbench();
    let prepared_reload = prepare_reload_for_edits(&workbench, edits);
    activate_prepared_reload(&mut workbench, prepared_reload);
    resolve_projection(&workbench, None)
        .expect_err("primitive projection should reject malformed appearance-state value")
}
