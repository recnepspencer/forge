use crate::PhysicalDriverKind;

use super::boundary::DriverBoundaryKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DriverFaultClass {
    NoFault,
    Crash,
    Corruption,
    MemoryPressure,
    IoPressure,
    ForbiddenShortcutAttempt,
    FutureExtensionSlot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DriverEvidenceSurface {
    ProductionBoundaryTrace,
    FreshRuntimeIsolation,
    MemoryPressureEnvelope,
    IoPressureEnvelope,
    OfflineVerifierTrace,
    ForbiddenShortcutRejection,
    FutureExtensionNonClaim,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverCapabilityProfile {
    driver: PhysicalDriverKind,
    boundary: DriverBoundaryKind,
    supported_faults: Vec<DriverFaultClass>,
    unsupported_faults: Vec<DriverFaultClass>,
    evidence_surfaces: Vec<DriverEvidenceSurface>,
}

impl DriverCapabilityProfile {
    pub fn production_storage_boundary() -> Self {
        Self::new(
            PhysicalDriverKind::ProductionBoundaryYieldpoint,
            DriverBoundaryKind::ProductionStorage,
            [DriverFaultClass::NoFault],
            [
                DriverFaultClass::Corruption,
                DriverFaultClass::MemoryPressure,
                DriverFaultClass::IoPressure,
                DriverFaultClass::FutureExtensionSlot,
            ],
            [DriverEvidenceSurface::ProductionBoundaryTrace],
        )
    }

    pub fn fresh_runtime_recovery() -> Self {
        Self::new(
            PhysicalDriverKind::FreshRuntimeRecovery,
            DriverBoundaryKind::CrashRuntimeIsolation,
            [DriverFaultClass::Crash],
            [DriverFaultClass::FutureExtensionSlot],
            [
                DriverEvidenceSurface::FreshRuntimeIsolation,
                DriverEvidenceSurface::ProductionBoundaryTrace,
            ],
        )
    }

    pub fn memory_pressure_boundary() -> Self {
        Self::new(
            PhysicalDriverKind::MemoryPressureBoundary,
            DriverBoundaryKind::MemoryPressure,
            [DriverFaultClass::MemoryPressure],
            [DriverFaultClass::FutureExtensionSlot],
            [DriverEvidenceSurface::MemoryPressureEnvelope],
        )
    }

    pub fn io_pressure_boundary() -> Self {
        Self::new(
            PhysicalDriverKind::IoPressureBoundary,
            DriverBoundaryKind::IoPressure,
            [DriverFaultClass::IoPressure],
            [DriverFaultClass::FutureExtensionSlot],
            [DriverEvidenceSurface::IoPressureEnvelope],
        )
    }

    pub fn offline_verifier_boundary() -> Self {
        Self::new(
            PhysicalDriverKind::OfflineVerifierBoundary,
            DriverBoundaryKind::OfflineVerifier,
            [DriverFaultClass::Corruption],
            [DriverFaultClass::FutureExtensionSlot],
            [DriverEvidenceSurface::OfflineVerifierTrace],
        )
    }

    pub fn shortcut_rejection_boundary() -> Self {
        Self::new(
            PhysicalDriverKind::ShortcutRejectionBoundary,
            DriverBoundaryKind::ShortcutRejection,
            [DriverFaultClass::ForbiddenShortcutAttempt],
            [DriverFaultClass::FutureExtensionSlot],
            [DriverEvidenceSurface::ForbiddenShortcutRejection],
        )
    }

    pub(crate) fn new(
        driver: PhysicalDriverKind,
        boundary: DriverBoundaryKind,
        supported_faults: impl IntoIterator<Item = DriverFaultClass>,
        unsupported_faults: impl IntoIterator<Item = DriverFaultClass>,
        evidence_surfaces: impl IntoIterator<Item = DriverEvidenceSurface>,
    ) -> Self {
        Self {
            driver,
            boundary,
            supported_faults: sorted_unique(supported_faults),
            unsupported_faults: sorted_unique(unsupported_faults),
            evidence_surfaces: sorted_unique(evidence_surfaces),
        }
    }

    pub const fn driver(&self) -> PhysicalDriverKind {
        self.driver
    }

    pub const fn boundary(&self) -> DriverBoundaryKind {
        self.boundary
    }

    pub fn supported_faults(&self) -> &[DriverFaultClass] {
        &self.supported_faults
    }

    pub fn unsupported_faults(&self) -> &[DriverFaultClass] {
        &self.unsupported_faults
    }

    pub fn evidence_surfaces(&self) -> &[DriverEvidenceSurface] {
        &self.evidence_surfaces
    }
}

fn sorted_unique<T: Ord>(values: impl IntoIterator<Item = T>) -> Vec<T> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}
