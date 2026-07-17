use worth_store_offline_verifier::OfflineVerifierBoundarySeam;
use worth_store_physical_backend::ProductionStorageBoundarySeam;

use super::DriverAdmissionDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalBoundarySeam {
    ProductionStorage(ProductionStorageBoundarySeam),
    OfflineVerifier(OfflineVerifierBoundarySeam),
    FreshRuntimeRecovery,
    MemoryPressure,
    IoPressure,
    ShortcutRejection,
    OperationalRecovery(crate::OperationalRecoveryYieldpoint),
    FutureExtensionSlot,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalBoundaryYieldpoint {
    name: String,
    seam: PhysicalBoundarySeam,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YieldpointDeclaration {
    yieldpoint: PhysicalBoundaryYieldpoint,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldpointScheduleBinding {
    scheduled_yieldpoint: String,
    declared_yieldpoint: PhysicalBoundaryYieldpoint,
}

impl PhysicalBoundaryYieldpoint {
    pub fn wal_append_before_flush() -> Self {
        Self::production_storage(ProductionStorageBoundarySeam::WalAppendBeforeFlush)
    }

    pub fn root_publication_before_observe() -> Self {
        Self::production_storage(ProductionStorageBoundarySeam::RootPublicationBeforeObserve)
    }

    pub fn fresh_runtime_replay_open() -> Self {
        Self::new(
            "fresh-runtime-replay-open",
            PhysicalBoundarySeam::FreshRuntimeRecovery,
        )
    }

    pub fn memory_pressure_boundary() -> Self {
        Self::new(
            "memory-pressure-boundary",
            PhysicalBoundarySeam::MemoryPressure,
        )
    }

    pub fn io_pressure_boundary() -> Self {
        Self::new("io-pressure-boundary", PhysicalBoundarySeam::IoPressure)
    }

    pub fn shortcut_rejection_boundary() -> Self {
        Self::new(
            "shortcut-rejection-boundary",
            PhysicalBoundarySeam::ShortcutRejection,
        )
    }

    pub fn production_storage(seam: ProductionStorageBoundarySeam) -> Self {
        Self::new(seam.token(), PhysicalBoundarySeam::ProductionStorage(seam))
    }

    pub fn offline_verifier(seam: OfflineVerifierBoundarySeam) -> Self {
        Self::new(seam.token(), PhysicalBoundarySeam::OfflineVerifier(seam))
    }

    pub fn offline_verifier_layout_walk_before_runtime_recovery() -> Self {
        Self::offline_verifier(OfflineVerifierBoundarySeam::LayoutWalkBeforeRuntimeRecovery)
    }

    pub fn operational_recovery(seam: crate::OperationalRecoveryYieldpoint) -> Self {
        Self::new(
            seam.token(),
            PhysicalBoundarySeam::OperationalRecovery(seam),
        )
    }

    fn new(name: impl Into<String>, seam: PhysicalBoundarySeam) -> Self {
        Self {
            name: name.into(),
            seam,
        }
    }

    pub fn declare(self) -> Result<YieldpointDeclaration, DriverAdmissionDenial> {
        if self.name.trim().is_empty() {
            return Err(DriverAdmissionDenial::EmptyYieldpointName);
        }
        let expected = canonical_yieldpoint_name_for_seam(self.seam);
        if self.name != expected {
            return Err(DriverAdmissionDenial::YieldpointSeamNameMismatch {
                actual: self.name,
                expected,
            });
        }
        Ok(YieldpointDeclaration { yieldpoint: self })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn seam(&self) -> PhysicalBoundarySeam {
        self.seam
    }
}

pub(crate) fn canonical_yieldpoint_name_for_seam(seam: PhysicalBoundarySeam) -> &'static str {
    match seam {
        PhysicalBoundarySeam::ProductionStorage(seam) => seam.token(),
        PhysicalBoundarySeam::OfflineVerifier(seam) => seam.token(),
        PhysicalBoundarySeam::FreshRuntimeRecovery => "fresh-runtime-replay-open",
        PhysicalBoundarySeam::MemoryPressure => "memory-pressure-boundary",
        PhysicalBoundarySeam::IoPressure => "io-pressure-boundary",
        PhysicalBoundarySeam::ShortcutRejection => "shortcut-rejection-boundary",
        PhysicalBoundarySeam::OperationalRecovery(seam) => seam.token(),
        PhysicalBoundarySeam::FutureExtensionSlot => "future-extension-slot",
    }
}

impl YieldpointDeclaration {
    pub fn yieldpoint(&self) -> &PhysicalBoundaryYieldpoint {
        &self.yieldpoint
    }
}

impl YieldpointScheduleBinding {
    pub(crate) fn bind(
        scheduled_yieldpoint: impl Into<String>,
        declared_yieldpoint: PhysicalBoundaryYieldpoint,
    ) -> Self {
        Self {
            scheduled_yieldpoint: scheduled_yieldpoint.into(),
            declared_yieldpoint,
        }
    }

    pub fn scheduled_yieldpoint(&self) -> &str {
        &self.scheduled_yieldpoint
    }

    pub fn declared_yieldpoint(&self) -> &PhysicalBoundaryYieldpoint {
        &self.declared_yieldpoint
    }
}
