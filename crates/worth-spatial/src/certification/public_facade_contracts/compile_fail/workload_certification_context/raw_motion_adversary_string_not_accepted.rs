use worth_spatial::facade::workload_certification_context::{
    WorkloadCertificationContext, WorkloadMotionBinding,
};

fn raw_strings_are_not_motion_adversaries(context: &WorkloadCertificationContext) {
    let _ = WorkloadMotionBinding::adversarial_for_context(
        context,
        "movement:tiny-rotation-exits-coplanar-class",
    );
}

fn main() {}
