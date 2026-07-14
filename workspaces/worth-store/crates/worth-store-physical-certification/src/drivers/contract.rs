use worth_store_physical_backend::BackendDurabilityProfileId;

use crate::PhysicalDriverKind;

use super::admission::DriverAdmissionDenial;
use super::boundary::DriverBoundaryKind;
use super::capability_profile::DriverCapabilityProfile;
use super::yieldpoint::{
    PhysicalBoundaryYieldpoint, YieldpointDeclaration, YieldpointScheduleBinding,
};
use super::yieldpoint_requirements::require_driver_yieldpoints;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationDriver {
    profile: DriverCapabilityProfile,
    yieldpoints: Vec<YieldpointDeclaration>,
    backend_profile: Option<BackendDurabilityProfileId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedDriverContractSet {
    drivers: Vec<PhysicalSimulationDriver>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionStorageBoundaryDriver {
    backend_profile: BackendDurabilityProfileId,
    yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdversarialStorageBoundaryDriver {
    yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashRuntimeIsolationDriver {
    yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryPressureDriver {
    yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoPressureDriver {
    yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineVerifierDriver {
    yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductionBoundaryDriverTrace {
    driver: PhysicalDriverKind,
    boundary: DriverBoundaryKind,
    backend_profile: BackendDurabilityProfileId,
    yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
}

impl PhysicalSimulationDriver {
    fn admit(
        profile: DriverCapabilityProfile,
        yieldpoints: Vec<PhysicalBoundaryYieldpoint>,
        backend_profile: Option<BackendDurabilityProfileId>,
    ) -> Result<Self, DriverAdmissionDenial> {
        let yieldpoints = require_driver_yieldpoints(profile.driver(), yieldpoints)?;
        Ok(Self {
            profile,
            yieldpoints,
            backend_profile,
        })
    }

    pub const fn kind(&self) -> PhysicalDriverKind {
        self.profile.driver()
    }

    pub const fn profile(&self) -> &DriverCapabilityProfile {
        &self.profile
    }

    pub const fn backend_profile(&self) -> Option<BackendDurabilityProfileId> {
        self.backend_profile
    }

    pub fn yieldpoints(&self) -> &[YieldpointDeclaration] {
        &self.yieldpoints
    }

    pub fn binds_yieldpoint(&self, name: &str) -> Option<&YieldpointDeclaration> {
        self.yieldpoints
            .iter()
            .find(|candidate| candidate.yieldpoint().name() == name)
    }

    pub fn production_boundary_trace(&self) -> Option<ProductionBoundaryDriverTrace> {
        if self.kind() != PhysicalDriverKind::ProductionBoundaryYieldpoint {
            return None;
        }
        let backend_profile = self.backend_profile?;
        Some(ProductionBoundaryDriverTrace {
            driver: self.kind(),
            boundary: self.profile.boundary(),
            backend_profile,
            yieldpoints: self
                .yieldpoints
                .iter()
                .map(|declaration| declaration.yieldpoint().clone())
                .collect(),
        })
    }
}

impl ProductionBoundaryDriverTrace {
    pub const fn driver(&self) -> PhysicalDriverKind {
        self.driver
    }

    pub const fn boundary(&self) -> DriverBoundaryKind {
        self.boundary
    }

    pub const fn backend_profile(&self) -> BackendDurabilityProfileId {
        self.backend_profile
    }

    pub fn yieldpoints(&self) -> &[PhysicalBoundaryYieldpoint] {
        &self.yieldpoints
    }
}

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
        Self::from_drivers([
            ProductionStorageBoundaryDriver::for_backend_profile(
                BackendDurabilityProfileId::PosixFileFsyncDirFsync,
            )
            .declare_yieldpoint(PhysicalBoundaryYieldpoint::wal_append_before_flush())
            .declare_yieldpoint(PhysicalBoundaryYieldpoint::root_publication_before_observe())
            .admit()?,
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
        let required_drivers = required_drivers
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        Self {
            drivers: self
                .drivers
                .iter()
                .filter(|driver| required_drivers.contains(&driver.kind()))
                .cloned()
                .collect(),
        }
    }

    pub(crate) fn bind_required_schedule_yieldpoint(
        &self,
        name: &str,
        required_drivers: impl IntoIterator<Item = PhysicalDriverKind>,
    ) -> Option<YieldpointScheduleBinding> {
        let required_drivers = required_drivers
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        self.drivers
            .iter()
            .filter(|driver| required_drivers.contains(&driver.kind()))
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

impl ProductionStorageBoundaryDriver {
    pub const fn for_backend_profile(backend_profile: BackendDurabilityProfileId) -> Self {
        Self {
            backend_profile,
            yieldpoints: Vec::new(),
        }
    }

    pub fn declare_yieldpoint(mut self, yieldpoint: PhysicalBoundaryYieldpoint) -> Self {
        self.yieldpoints.push(yieldpoint);
        self
    }

    pub fn admit(self) -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
        PhysicalSimulationDriver::admit(
            DriverCapabilityProfile::production_storage_boundary(),
            self.yieldpoints,
            Some(self.backend_profile),
        )
    }
}

impl AdversarialStorageBoundaryDriver {
    pub const fn shortcut_rejection() -> Self {
        Self {
            yieldpoints: Vec::new(),
        }
    }

    pub fn declare_yieldpoint(mut self, yieldpoint: PhysicalBoundaryYieldpoint) -> Self {
        self.yieldpoints.push(yieldpoint);
        self
    }

    pub fn admit(self) -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
        PhysicalSimulationDriver::admit(
            DriverCapabilityProfile::shortcut_rejection_boundary(),
            self.yieldpoints,
            None,
        )
    }
}

impl CrashRuntimeIsolationDriver {
    pub const fn fresh_runtime_recovery() -> Self {
        Self {
            yieldpoints: Vec::new(),
        }
    }

    pub fn declare_yieldpoint(mut self, yieldpoint: PhysicalBoundaryYieldpoint) -> Self {
        self.yieldpoints.push(yieldpoint);
        self
    }

    pub fn admit(self) -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
        PhysicalSimulationDriver::admit(
            DriverCapabilityProfile::fresh_runtime_recovery(),
            self.yieldpoints,
            None,
        )
    }
}

impl MemoryPressureDriver {
    pub const fn deterministic_pressure_boundary() -> Self {
        Self {
            yieldpoints: Vec::new(),
        }
    }

    pub fn declare_yieldpoint(mut self, yieldpoint: PhysicalBoundaryYieldpoint) -> Self {
        self.yieldpoints.push(yieldpoint);
        self
    }

    pub fn admit(self) -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
        PhysicalSimulationDriver::admit(
            DriverCapabilityProfile::memory_pressure_boundary(),
            self.yieldpoints,
            None,
        )
    }

    pub fn fake_in_memory_only() -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
        Err(DriverAdmissionDenial::FakeInMemoryOnlyDriverDenied)
    }
}

impl IoPressureDriver {
    pub const fn deterministic_queue_boundary() -> Self {
        Self {
            yieldpoints: Vec::new(),
        }
    }

    pub fn declare_yieldpoint(mut self, yieldpoint: PhysicalBoundaryYieldpoint) -> Self {
        self.yieldpoints.push(yieldpoint);
        self
    }

    pub fn admit(self) -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
        PhysicalSimulationDriver::admit(
            DriverCapabilityProfile::io_pressure_boundary(),
            self.yieldpoints,
            None,
        )
    }

    pub fn sleep_based_scheduling() -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
        Err(DriverAdmissionDenial::SleepBasedSchedulingDenied)
    }
}

impl OfflineVerifierDriver {
    pub const fn layout_walk_boundary() -> Self {
        Self {
            yieldpoints: Vec::new(),
        }
    }

    pub fn declare_yieldpoint(mut self, yieldpoint: PhysicalBoundaryYieldpoint) -> Self {
        self.yieldpoints.push(yieldpoint);
        self
    }

    pub fn admit(self) -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
        PhysicalSimulationDriver::admit(
            DriverCapabilityProfile::offline_verifier_boundary(),
            self.yieldpoints,
            None,
        )
    }
}

pub fn private_mutation_driver_attempt() -> Result<PhysicalSimulationDriver, DriverAdmissionDenial>
{
    Err(DriverAdmissionDenial::PrivateMutationDriverDenied)
}

pub fn test_support_verdict_driver_attempt(
) -> Result<PhysicalSimulationDriver, DriverAdmissionDenial> {
    Err(DriverAdmissionDenial::TestSupportVerdictDriverDenied)
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
