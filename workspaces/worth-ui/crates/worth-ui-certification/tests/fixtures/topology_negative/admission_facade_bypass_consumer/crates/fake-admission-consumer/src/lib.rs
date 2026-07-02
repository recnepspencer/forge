use runtime_admission::{UiAdmissionTarget, UiAdmissionWorld};
use runtime_bypass::facade::admission as runtime_admission;

type AliasedAdmissionTarget = runtime_admission::UiAdmissionTarget;
type AliasedAdmissionWorld = runtime_admission::UiAdmissionWorld;

pub fn bypass_runtime_owned_admission(target: UiAdmissionTarget, world: UiAdmissionWorld) {
    let _ = (target, world);
}

pub fn bypass_runtime_owned_admission_via_nested_alias(
    target: AliasedAdmissionTarget,
    world: AliasedAdmissionWorld,
) {
    let _ = (target, world);
}
