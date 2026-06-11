use worth_spatial::facade::planar_motion_posture::{
    PlanarMotionCancellation, PlanarReorientation, PlanarRotationPosture,
};
use worth_spatial::facade::transform_workload::TransformPostureReceipt;

fn main() {
    let _ = TransformPostureReceipt::new(
        unconstructible(),
        "projected",
        PlanarRotationPosture::ExactRotation,
        PlanarReorientation::PreservesHandedness,
        PlanarMotionCancellation::None,
    );
}

fn unconstructible<T>() -> T {
    panic!("compile-fail input is never executed")
}
