use worth_store_physical_backend::BackendDurabilityProfileId;

use crate::PhysicalDriverKind;

use super::admission::DriverAdmissionDenial;
use super::boundary::DriverBoundaryKind;
use super::capability_profile::DriverCapabilityProfile;
use super::yieldpoint::{PhysicalBoundaryYieldpoint, YieldpointDeclaration};
use super::yieldpoint_requirements::require_driver_yieldpoints;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSimulationDriver {
    profile: DriverCapabilityProfile,
    yieldpoints: Vec<YieldpointDeclaration>,
    backend_profile: Option<BackendDurabilityProfileId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedDriverContractSet {
    pub(super) drivers: Vec<PhysicalSimulationDriver>,
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
