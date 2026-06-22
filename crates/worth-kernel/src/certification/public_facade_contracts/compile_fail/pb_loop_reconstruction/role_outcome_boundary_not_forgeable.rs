use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopContainmentEvidencePostureSet, PlanarBooleanLoopRoleOutcomeBoundary,
    PlanarBooleanLoopRoleOutcomeBoundaryCounters, PlanarBooleanLoopRoleOutcomeSet,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanLoopRoleOutcomeBoundary {
        role_outcomes: bogus::<PlanarBooleanLoopRoleOutcomeSet>(),
        containment_evidence_postures: bogus::<PlanarBooleanLoopContainmentEvidencePostureSet>(),
        counters: PlanarBooleanLoopRoleOutcomeBoundaryCounters::default(),
    };
}
