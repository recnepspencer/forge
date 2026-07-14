use super::denial::{FaultDeliveryDenial, FaultObservedBoundaryKind};
use crate::{FreshRuntimeCrashRecoveryEvidence, ProductionBoundaryDriverTrace};
use worth_foundational::FoundationalBoundaryEvidenceLocality;
use worth_store_physical_integrity::{
    PhysicalBoundaryLocalization, PhysicalContainerIntegrityDenial,
    PhysicalContainerIntegrityDenialKind, PreDecodePhysicalDenial, PreDecodePhysicalDenialKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoFaultProductionBoundaryParity {
    ordinary_trace: ProductionBoundaryDriverTrace,
    control_trace: ProductionBoundaryDriverTrace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservedFaultBoundary {
    PreDecodeIntegrityDenial {
        kind: PreDecodePhysicalDenialKind,
        locality: FoundationalBoundaryEvidenceLocality,
    },
    PhysicalIntegrityBoundary {
        kind: PhysicalContainerIntegrityDenialKind,
        localization: PhysicalBoundaryLocalization,
        locality: FoundationalBoundaryEvidenceLocality,
    },
    FreshRuntimeCrashRecovery {
        locality: FoundationalBoundaryEvidenceLocality,
    },
    NoFaultProductionBoundary {
        parity: NoFaultProductionBoundaryParity,
        locality: FoundationalBoundaryEvidenceLocality,
    },
    IoPressureBoundary {
        locality: FoundationalBoundaryEvidenceLocality,
    },
}

impl NoFaultProductionBoundaryParity {
    pub fn from_traces(
        ordinary_trace: ProductionBoundaryDriverTrace,
        control_trace: ProductionBoundaryDriverTrace,
    ) -> Result<Self, FaultDeliveryDenial> {
        if ordinary_trace != control_trace {
            return Err(FaultDeliveryDenial::NoFaultParityMismatch);
        }
        Ok(Self {
            ordinary_trace,
            control_trace,
        })
    }

    pub const fn ordinary_trace(&self) -> &ProductionBoundaryDriverTrace {
        &self.ordinary_trace
    }

    pub const fn control_trace(&self) -> &ProductionBoundaryDriverTrace {
        &self.control_trace
    }
}

impl ObservedFaultBoundary {
    pub fn pre_decode_integrity_denial(denial: &PreDecodePhysicalDenial) -> Self {
        Self::PreDecodeIntegrityDenial {
            kind: denial.kind(),
            locality: FoundationalBoundaryEvidenceLocality::Current,
        }
    }

    pub const fn pre_decode_integrity_denial_kind(kind: PreDecodePhysicalDenialKind) -> Self {
        Self::PreDecodeIntegrityDenial {
            kind,
            locality: FoundationalBoundaryEvidenceLocality::Current,
        }
    }

    pub fn physical_integrity_denial(denial: &PhysicalContainerIntegrityDenial) -> Self {
        Self::PhysicalIntegrityBoundary {
            kind: denial.kind(),
            localization: denial.localization(),
            locality: FoundationalBoundaryEvidenceLocality::Current,
        }
    }

    pub const fn physical_integrity_boundary(
        kind: PhysicalContainerIntegrityDenialKind,
        localization: PhysicalBoundaryLocalization,
    ) -> Self {
        Self::PhysicalIntegrityBoundary {
            kind,
            localization,
            locality: FoundationalBoundaryEvidenceLocality::Current,
        }
    }

    pub const fn fresh_runtime_crash_recovery(
        _evidence: &FreshRuntimeCrashRecoveryEvidence,
    ) -> Self {
        Self::FreshRuntimeCrashRecovery {
            locality: FoundationalBoundaryEvidenceLocality::RestoredReadmitted,
        }
    }

    pub const fn no_fault_production_boundary(parity: NoFaultProductionBoundaryParity) -> Self {
        Self::NoFaultProductionBoundary {
            parity,
            locality: FoundationalBoundaryEvidenceLocality::Current,
        }
    }

    pub const fn io_pressure_boundary() -> Self {
        Self::IoPressureBoundary {
            locality: FoundationalBoundaryEvidenceLocality::Current,
        }
    }

    pub const fn locality(&self) -> FoundationalBoundaryEvidenceLocality {
        match self {
            Self::PreDecodeIntegrityDenial { locality, .. }
            | Self::PhysicalIntegrityBoundary { locality, .. }
            | Self::FreshRuntimeCrashRecovery { locality }
            | Self::NoFaultProductionBoundary { locality, .. }
            | Self::IoPressureBoundary { locality } => *locality,
        }
    }

    pub const fn boundary_kind(&self) -> FaultObservedBoundaryKind {
        match self {
            Self::PreDecodeIntegrityDenial { .. } => {
                FaultObservedBoundaryKind::PreDecodeIntegrityDenial
            }
            Self::PhysicalIntegrityBoundary { .. } => {
                FaultObservedBoundaryKind::PhysicalIntegrityBoundary
            }
            Self::FreshRuntimeCrashRecovery { .. } => {
                FaultObservedBoundaryKind::FreshRuntimeCrashRecovery
            }
            Self::NoFaultProductionBoundary { .. } => {
                FaultObservedBoundaryKind::NoFaultProductionBoundary
            }
            Self::IoPressureBoundary { .. } => FaultObservedBoundaryKind::IoPressureBoundary,
        }
    }

    pub const fn no_fault_parity(&self) -> Option<&NoFaultProductionBoundaryParity> {
        match self {
            Self::NoFaultProductionBoundary { parity, .. } => Some(parity),
            _ => None,
        }
    }
}
