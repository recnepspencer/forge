extern crate worth_ui_runtime as runtime_bypass;

use runtime_bypass::facade::admission::{UiAdmissionTarget, UiAdmissionWorld};

pub fn bypass_runtime_owned_admission_via_extern_alias(
    target: UiAdmissionTarget,
    world: UiAdmissionWorld,
) {
    let _ = (target, world);
}
