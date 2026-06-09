use worth_kernel::facade::diagnostics::motion::{
    prepare_primitive_construction_motion_dx_surface_report,
    PrimitiveConstructionMotionWitnessResolutionReport,
};

fn main() {
    let _ = prepare_primitive_construction_motion_dx_surface_report;
    let _ = std::mem::size_of::<PrimitiveConstructionMotionWitnessResolutionReport>();
}
