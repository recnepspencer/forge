use crate::{
    PhysicalFaultEvidenceClass, PhysicalScenarioFaultKind, S6IoPressureFaultKind,
    S6PressureEvidenceMaturity,
};

pub const fn all_s6_fault_evidence_classes() -> [PhysicalFaultEvidenceClass; 6] {
    [
        PhysicalFaultEvidenceClass::Simulated,
        PhysicalFaultEvidenceClass::InjectedProductionBoundary,
        PhysicalFaultEvidenceClass::BackendEmulated,
        PhysicalFaultEvidenceClass::ObservedHost,
        PhysicalFaultEvidenceClass::CertifiedBackend,
        PhysicalFaultEvidenceClass::ExternallyGuaranteed,
    ]
}

pub const fn all_s6_io_pressure_fault_kinds() -> [S6IoPressureFaultKind; 6] {
    [
        S6IoPressureFaultKind::BackendLatencyInjection,
        S6IoPressureFaultKind::QueueDepthSaturation,
        S6IoPressureFaultKind::BandwidthThrottle,
        S6IoPressureFaultKind::DelayedSync,
        S6IoPressureFaultKind::PageCachePressure,
        S6IoPressureFaultKind::BackgroundPacingLateYield,
    ]
}

pub(crate) const fn fault_phase_for_pressure_fault(
    fault_kind: S6IoPressureFaultKind,
) -> PhysicalScenarioFaultKind {
    match fault_kind {
        S6IoPressureFaultKind::BackendLatencyInjection => {
            PhysicalScenarioFaultKind::S6BackendLatencyInjection
        }
        S6IoPressureFaultKind::QueueDepthSaturation => {
            PhysicalScenarioFaultKind::S6QueueDepthSaturation
        }
        S6IoPressureFaultKind::BandwidthThrottle => PhysicalScenarioFaultKind::S6BandwidthThrottle,
        S6IoPressureFaultKind::DelayedSync => PhysicalScenarioFaultKind::S6DelayedSync,
        S6IoPressureFaultKind::PageCachePressure => PhysicalScenarioFaultKind::S6PageCachePressure,
        S6IoPressureFaultKind::BackgroundPacingLateYield => {
            PhysicalScenarioFaultKind::S6BackgroundPacingLateYield
        }
    }
}

pub(crate) const fn maturity_for_fault_evidence_class(
    evidence_class: PhysicalFaultEvidenceClass,
) -> S6PressureEvidenceMaturity {
    match evidence_class {
        PhysicalFaultEvidenceClass::Simulated => S6PressureEvidenceMaturity::SimulatedOnly,
        PhysicalFaultEvidenceClass::InjectedProductionBoundary => {
            S6PressureEvidenceMaturity::ProductionBoundaryInjected
        }
        PhysicalFaultEvidenceClass::BackendEmulated => S6PressureEvidenceMaturity::BackendEmulated,
        PhysicalFaultEvidenceClass::ObservedHost => S6PressureEvidenceMaturity::HostObserved,
        PhysicalFaultEvidenceClass::CertifiedBackend => {
            S6PressureEvidenceMaturity::BackendCertified
        }
        PhysicalFaultEvidenceClass::ExternallyGuaranteed => {
            S6PressureEvidenceMaturity::ExternalGuarantee
        }
    }
}
