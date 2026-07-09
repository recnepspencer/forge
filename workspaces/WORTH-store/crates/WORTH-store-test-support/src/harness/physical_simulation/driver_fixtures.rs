use worth_store_physical_backend::BackendDurabilityProfileId;
use worth_store_physical_certification::{
    AdmittedDriverContractSet, DriverAdmissionDenial, PhysicalSimulationDriver,
    ProductionStorageBoundaryDriver,
};

pub fn admitted_developer_smoke_driver_contracts(
) -> Result<AdmittedDriverContractSet, DriverAdmissionDenial> {
    AdmittedDriverContractSet::developer_smoke()
}

pub fn admitted_ci_certification_driver_contracts(
) -> Result<AdmittedDriverContractSet, DriverAdmissionDenial> {
    AdmittedDriverContractSet::ci_certification()
}

pub fn unbound_production_driver() -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
    ProductionStorageBoundaryDriver::for_backend_profile(
        BackendDurabilityProfileId::PosixFileFsyncDirFsync,
    )
    .admit()
}
