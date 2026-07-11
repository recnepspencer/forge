use forge_store_physical_backend::{BackendDurabilityProfileId, ProductionStorageBoundarySeam};
use forge_store_physical_certification::{
    DriverBoundaryKind, PhysicalBoundarySeam, PhysicalBoundaryYieldpoint, PhysicalDriverKind,
    ProductionStorageBoundaryDriver,
};

#[test]
fn production_driver_trace_matches_backend_registered_boundary_seams() {
    let driver = registered_production_boundary_driver();
    let trace = driver
        .production_boundary_trace()
        .expect("production storage driver must emit boundary trace");

    assert_eq!(
        trace.driver(),
        PhysicalDriverKind::ProductionBoundaryYieldpoint
    );
    assert_eq!(trace.boundary(), DriverBoundaryKind::ProductionStorage);
    assert_eq!(
        trace.backend_profile(),
        BackendDurabilityProfileId::PosixFileFsyncDirFsync
    );
    assert_eq!(
        trace.yieldpoints().len(),
        ProductionStorageBoundarySeam::phase4_registered_seams().len()
    );

    for seam in ProductionStorageBoundarySeam::phase4_registered_seams() {
        let yieldpoint = trace
            .yieldpoints()
            .iter()
            .find(|candidate| candidate.seam() == PhysicalBoundarySeam::ProductionStorage(*seam))
            .expect("registered production seam must appear in admitted trace");
        assert_eq!(yieldpoint.name(), seam.token());
    }
}

#[test]
fn non_production_drivers_do_not_emit_production_boundary_trace() {
    let memory_driver =
        forge_store_physical_certification::MemoryPressureDriver::deterministic_pressure_boundary()
            .declare_yieldpoint(PhysicalBoundaryYieldpoint::memory_pressure_boundary())
            .admit()
            .unwrap();

    assert!(memory_driver.production_boundary_trace().is_none());
}

fn registered_production_boundary_driver(
) -> forge_store_physical_certification::PhysicalSimulationDriver {
    let mut driver = ProductionStorageBoundaryDriver::for_backend_profile(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    );
    for seam in ProductionStorageBoundarySeam::phase4_registered_seams() {
        driver = driver.declare_yieldpoint(PhysicalBoundaryYieldpoint::production_storage(*seam));
    }
    driver.admit().unwrap()
}
