use worth_store_physical_backend::{BackendDurabilityProfileId, ProductionStorageBoundarySeam};

use super::{
    AdmittedDriverContractSet, AdversarialStorageBoundaryDriver, CrashRuntimeIsolationDriver,
    DriverAdmissionDenial, IoPressureDriver, MemoryPressureDriver, OfflineVerifierDriver,
    PhysicalBoundaryYieldpoint, PhysicalSimulationDriver, ProductionStorageBoundaryDriver,
    YieldpointScheduleBinding,
};
use crate::PhysicalDriverKind;

impl AdmittedDriverContractSet {
    pub fn from_drivers(
        drivers: impl IntoIterator<Item = PhysicalSimulationDriver>,
    ) -> Result<Self, DriverAdmissionDenial> {
        Ok(Self {
            drivers: sorted_unique_driver_contracts(drivers)?,
        })
    }

    pub fn empty() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }

    pub fn developer_smoke() -> Result<Self, DriverAdmissionDenial> {
        Self::developer_smoke_with_production_storage_yieldpoints(
            BackendDurabilityProfileId::PosixFileFsyncDirFsync,
            [],
        )
    }

    pub fn developer_smoke_with_production_storage_yieldpoints(
        backend_profile: BackendDurabilityProfileId,
        additional: impl IntoIterator<Item = ProductionStorageBoundarySeam>,
    ) -> Result<Self, DriverAdmissionDenial> {
        let additional = additional
            .into_iter()
            .filter(|seam| !baseline_storage_seam(*seam))
            .collect::<std::collections::BTreeSet<_>>();
        let production = additional.into_iter().fold(
            ProductionStorageBoundaryDriver::for_backend_profile(backend_profile)
                .declare_yieldpoint(PhysicalBoundaryYieldpoint::wal_append_before_flush())
                .declare_yieldpoint(PhysicalBoundaryYieldpoint::root_publication_before_observe()),
            |driver, seam| {
                driver.declare_yieldpoint(PhysicalBoundaryYieldpoint::production_storage(seam))
            },
        );
        Self::from_drivers([
            production.admit()?,
            CrashRuntimeIsolationDriver::fresh_runtime_recovery()
                .declare_yieldpoint(PhysicalBoundaryYieldpoint::fresh_runtime_replay_open())
                .admit()?,
            AdversarialStorageBoundaryDriver::shortcut_rejection()
                .declare_yieldpoint(PhysicalBoundaryYieldpoint::shortcut_rejection_boundary())
                .admit()?,
            MemoryPressureDriver::deterministic_pressure_boundary()
                .declare_yieldpoint(PhysicalBoundaryYieldpoint::memory_pressure_boundary())
                .admit()?,
            IoPressureDriver::deterministic_queue_boundary()
                .declare_yieldpoint(PhysicalBoundaryYieldpoint::io_pressure_boundary())
                .admit()?,
            OfflineVerifierDriver::layout_walk_boundary()
                .declare_yieldpoint(
                    PhysicalBoundaryYieldpoint::offline_verifier_layout_walk_before_runtime_recovery(),
                )
                .admit()?,
        ])
    }

    pub fn ci_certification() -> Result<Self, DriverAdmissionDenial> {
        Self::developer_smoke()
    }

    pub fn without(mut self, driver: PhysicalDriverKind) -> Self {
        self.drivers.retain(|candidate| candidate.kind() != driver);
        self
    }

    pub fn contains_driver(&self, driver: PhysicalDriverKind) -> bool {
        self.drivers
            .iter()
            .any(|candidate| candidate.kind() == driver)
    }

    pub(crate) fn select_required_drivers(
        &self,
        required_drivers: impl IntoIterator<Item = PhysicalDriverKind>,
    ) -> Self {
        let required = required_drivers
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        Self {
            drivers: self
                .drivers
                .iter()
                .filter(|driver| required.contains(&driver.kind()))
                .cloned()
                .collect(),
        }
    }

    pub(crate) fn bind_required_schedule_yieldpoint(
        &self,
        name: &str,
        required_drivers: impl IntoIterator<Item = PhysicalDriverKind>,
    ) -> Option<YieldpointScheduleBinding> {
        let required = required_drivers
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        self.drivers
            .iter()
            .filter(|driver| required.contains(&driver.kind()))
            .flat_map(|driver| driver.yieldpoints())
            .find(|candidate| candidate.yieldpoint().name() == name)
            .map(|candidate| YieldpointScheduleBinding::bind(name, candidate.yieldpoint().clone()))
    }

    pub fn binds_yieldpoint(&self, name: &str) -> bool {
        self.drivers
            .iter()
            .flat_map(|driver| driver.yieldpoints())
            .any(|candidate| candidate.yieldpoint().name() == name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &PhysicalSimulationDriver> {
        self.drivers.iter()
    }
}

const fn baseline_storage_seam(seam: ProductionStorageBoundarySeam) -> bool {
    matches!(
        seam,
        ProductionStorageBoundarySeam::WalAppendBeforeFlush
            | ProductionStorageBoundarySeam::RootPublicationBeforeObserve
    )
}

fn sorted_unique_driver_contracts(
    drivers: impl IntoIterator<Item = PhysicalSimulationDriver>,
) -> Result<Vec<PhysicalSimulationDriver>, DriverAdmissionDenial> {
    let mut by_kind = std::collections::BTreeMap::new();
    for driver in drivers {
        let kind = driver.kind();
        if by_kind.insert(kind, driver).is_some() {
            return Err(DriverAdmissionDenial::DuplicateDriverKind(kind));
        }
    }
    Ok(by_kind.into_values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn developer_schedule_binds_an_additional_registered_storage_seam() {
        let contracts =
            AdmittedDriverContractSet::developer_smoke_with_production_storage_yieldpoints(
                BackendDurabilityProfileId::PosixFileFsyncDirFsync,
                [ProductionStorageBoundarySeam::DirectorySync],
            )
            .unwrap();
        assert!(contracts
            .bind_required_schedule_yieldpoint(
                ProductionStorageBoundarySeam::DirectorySync.token(),
                [PhysicalDriverKind::ProductionBoundaryYieldpoint],
            )
            .is_some());
    }

    #[test]
    fn baseline_seams_are_idempotent_when_explicitly_requested() {
        AdmittedDriverContractSet::developer_smoke_with_production_storage_yieldpoints(
            BackendDurabilityProfileId::PosixFileFsyncDirFsync,
            [ProductionStorageBoundarySeam::RootPublicationBeforeObserve],
        )
        .unwrap();
    }
}
