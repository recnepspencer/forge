use runtime_bypass::facade::admission as runtime_admission;

type RuntimeAdmissionTarget = runtime_admission::UiAdmissionTarget;
type RuntimeAdmissionWorld = runtime_admission::UiAdmissionWorld;

pub fn bypass_runtime_owned_admission_via_direct_alias(
    target: RuntimeAdmissionTarget,
    world: RuntimeAdmissionWorld,
) {
    let _ = (target, world);
}
