use worth_store_physical_backend::{BackendDurabilityProfileId, ProductionStorageBoundarySeam};
use worth_store_physical_certification::{
    PhysicalBoundaryYieldpoint, PhysicalDriverKind, PhysicalSimulationDriver,
    ProductionBoundaryDriverTrace, ProductionStorageBoundaryDriver,
};
use worth_store_test_support::admitted_developer_smoke_driver_contracts;

pub fn developer_smoke_production_trace() -> ProductionBoundaryDriverTrace {
    admitted_developer_smoke_driver_contracts()
        .unwrap()
        .iter()
        .find(|driver| driver.kind() == PhysicalDriverKind::ProductionBoundaryYieldpoint)
        .and_then(|driver| driver.production_boundary_trace())
        .unwrap()
}

pub fn alternate_production_trace() -> ProductionBoundaryDriverTrace {
    production_driver_with_profile(BackendDurabilityProfileId::WindowsFlushFileBuffers)
        .production_boundary_trace()
        .unwrap()
}

pub fn production_driver_with_all_seams() -> PhysicalSimulationDriver {
    production_driver_with_profile(BackendDurabilityProfileId::PosixFileFsyncDirFsync)
}

pub fn production_driver_with_profile(
    profile: BackendDurabilityProfileId,
) -> PhysicalSimulationDriver {
    let mut driver = ProductionStorageBoundaryDriver::for_backend_profile(profile);
    for seam in ProductionStorageBoundarySeam::registered_backend_operation_seams() {
        driver = driver.declare_yieldpoint(PhysicalBoundaryYieldpoint::production_storage(*seam));
    }
    driver.admit().unwrap()
}
