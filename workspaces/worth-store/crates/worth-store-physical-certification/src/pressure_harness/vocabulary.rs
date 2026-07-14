use crate::{
    IoPressureEvidenceMaturity, IoPressureFaultKind, PhysicalFaultEvidenceClass,
    PhysicalScenarioFaultKind,
};

pub const fn all_io_pressure_fault_evidence_classes() -> [PhysicalFaultEvidenceClass; 6] {
    [
        PhysicalFaultEvidenceClass::Simulated,
        PhysicalFaultEvidenceClass::InjectedProductionBoundary,
        PhysicalFaultEvidenceClass::BackendEmulated,
        PhysicalFaultEvidenceClass::ObservedHost,
        PhysicalFaultEvidenceClass::CertifiedBackend,
        PhysicalFaultEvidenceClass::ExternallyGuaranteed,
    ]
}

pub const fn all_io_pressure_fault_kinds() -> [IoPressureFaultKind; 6] {
    [
        IoPressureFaultKind::BackendLatencyInjection,
        IoPressureFaultKind::QueueDepthSaturation,
        IoPressureFaultKind::BandwidthThrottle,
        IoPressureFaultKind::DelayedSync,
        IoPressureFaultKind::PageCachePressure,
        IoPressureFaultKind::BackgroundPacingLateYield,
    ]
}

pub(crate) const fn fault_phase_for_pressure_fault(
    fault_kind: IoPressureFaultKind,
) -> PhysicalScenarioFaultKind {
    match fault_kind {
        IoPressureFaultKind::BackendLatencyInjection => {
            PhysicalScenarioFaultKind::IoPressureBackendLatencyInjection
        }
        IoPressureFaultKind::QueueDepthSaturation => {
            PhysicalScenarioFaultKind::IoPressureQueueDepthSaturation
        }
        IoPressureFaultKind::BandwidthThrottle => {
            PhysicalScenarioFaultKind::IoPressureBandwidthThrottle
        }
        IoPressureFaultKind::DelayedSync => PhysicalScenarioFaultKind::IoPressureDelayedSync,
        IoPressureFaultKind::PageCachePressure => {
            PhysicalScenarioFaultKind::IoPressurePageCachePressure
        }
        IoPressureFaultKind::BackgroundPacingLateYield => {
            PhysicalScenarioFaultKind::IoPressureBackgroundPacingLateYield
        }
    }
}

pub(crate) const fn maturity_for_fault_evidence_class(
    evidence_class: PhysicalFaultEvidenceClass,
) -> IoPressureEvidenceMaturity {
    match evidence_class {
        PhysicalFaultEvidenceClass::Simulated => IoPressureEvidenceMaturity::SimulatedOnly,
        PhysicalFaultEvidenceClass::InjectedProductionBoundary => {
            IoPressureEvidenceMaturity::ProductionBoundaryInjected
        }
        PhysicalFaultEvidenceClass::BackendEmulated => IoPressureEvidenceMaturity::BackendEmulated,
        PhysicalFaultEvidenceClass::ObservedHost => IoPressureEvidenceMaturity::HostObserved,
        PhysicalFaultEvidenceClass::CertifiedBackend => {
            IoPressureEvidenceMaturity::BackendCertified
        }
        PhysicalFaultEvidenceClass::ExternallyGuaranteed => {
            IoPressureEvidenceMaturity::ExternalGuarantee
        }
    }
}
