use forge_store_io_scheduler::{
    foreground_reservation::ForegroundIoLaneKind, BackgroundIoPressureClass,
    LatencyEnvelopeAssessmentStatus,
};
use forge_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};

use crate::{
    PhysicalDriverKind, PhysicalScenarioFaultKind, PhysicalSimulationProfile,
    S6IoPressureHarnessEvidenceDenial, SimulationReplayBundle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalFaultEvidenceClass {
    Simulated,
    InjectedProductionBoundary,
    BackendEmulated,
    ObservedHost,
    CertifiedBackend,
    ExternallyGuaranteed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S6IoPressureFaultKind {
    BackendLatencyInjection,
    QueueDepthSaturation,
    BandwidthThrottle,
    DelayedSync,
    PageCachePressure,
    BackgroundPacingLateYield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S6HarnessSecureIoPosture {
    ScopePreserving,
    UnsupportedTypedDenial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum S6PressureEvidenceMaturity {
    SimulatedOnly,
    ProductionBoundaryInjected,
    BackendEmulated,
    HostObserved,
    BackendCertified,
    ExternalGuarantee,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6BackendSafetyQualificationDenial {
    SimulatedBackendSuccess,
    InjectedBoundaryOnly,
    BackendEmulatedOnly,
    ObservedHostOnly,
    BackendEvidenceClassTooWeak {
        required: CapabilityEvidenceClass,
        actual: CapabilityEvidenceClass,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6IoPressureHarnessScenario {
    backend_profile: BackendTargetProfile,
    backend_evidence_class: CapabilityEvidenceClass,
    foreground_lane: ForegroundIoLaneKind,
    background_pressure: BackgroundIoPressureClass,
    secure_io_posture: S6HarnessSecureIoPosture,
    fault_kind: S6IoPressureFaultKind,
    fault_evidence_class: PhysicalFaultEvidenceClass,
    expected_status: LatencyEnvelopeAssessmentStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6IoPressureHarnessEvidence {
    scenario: S6IoPressureHarnessScenario,
    driver: PhysicalDriverKind,
    fault_phase: PhysicalScenarioFaultKind,
    maturity: S6PressureEvidenceMaturity,
    replay_profile: PhysicalSimulationProfile,
    replay_identity: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6IoPressureOracleObservation {
    foreground_lane: ForegroundIoLaneKind,
    background_pressure: BackgroundIoPressureClass,
    secure_io_posture: S6HarnessSecureIoPosture,
    fault_kind: S6IoPressureFaultKind,
    fault_evidence_class: PhysicalFaultEvidenceClass,
    envelope_status: LatencyEnvelopeAssessmentStatus,
    attribution_complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6RealBackendSafetyQualification {
    backend_profile: BackendTargetProfile,
    evidence_class: PhysicalFaultEvidenceClass,
}

impl S6IoPressureHarnessScenario {
    pub const fn deterministic_read_under_repair_pressure() -> Self {
        Self {
            backend_profile: BackendTargetProfile::PosixFileFsyncDirSync,
            backend_evidence_class: CapabilityEvidenceClass::CertifiedBackendProfile,
            foreground_lane: ForegroundIoLaneKind::PointRead,
            background_pressure: BackgroundIoPressureClass::RepairScan,
            secure_io_posture: S6HarnessSecureIoPosture::ScopePreserving,
            fault_kind: S6IoPressureFaultKind::BackendLatencyInjection,
            fault_evidence_class: PhysicalFaultEvidenceClass::InjectedProductionBoundary,
            expected_status: LatencyEnvelopeAssessmentStatus::Held,
        }
    }

    pub const fn with_fault_evidence_class(
        mut self,
        fault_evidence_class: PhysicalFaultEvidenceClass,
    ) -> Self {
        self.fault_evidence_class = fault_evidence_class;
        self
    }

    pub const fn with_foreground_lane(mut self, foreground_lane: ForegroundIoLaneKind) -> Self {
        self.foreground_lane = foreground_lane;
        self
    }

    pub const fn with_background_pressure(
        mut self,
        background_pressure: BackgroundIoPressureClass,
    ) -> Self {
        self.background_pressure = background_pressure;
        self
    }

    pub const fn with_backend_evidence_class(
        mut self,
        backend_evidence_class: CapabilityEvidenceClass,
    ) -> Self {
        self.backend_evidence_class = backend_evidence_class;
        self
    }

    pub const fn with_backend_profile(mut self, backend_profile: BackendTargetProfile) -> Self {
        self.backend_profile = backend_profile;
        self
    }

    pub const fn with_fault_kind(mut self, fault_kind: S6IoPressureFaultKind) -> Self {
        self.fault_kind = fault_kind;
        self
    }

    pub const fn with_expected_status(
        mut self,
        expected_status: LatencyEnvelopeAssessmentStatus,
    ) -> Self {
        self.expected_status = expected_status;
        self
    }

    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn backend_evidence_class(&self) -> CapabilityEvidenceClass {
        self.backend_evidence_class
    }

    pub const fn foreground_lane(&self) -> ForegroundIoLaneKind {
        self.foreground_lane
    }

    pub const fn background_pressure(&self) -> BackgroundIoPressureClass {
        self.background_pressure
    }

    pub const fn secure_io_posture(&self) -> S6HarnessSecureIoPosture {
        self.secure_io_posture
    }

    pub const fn fault_kind(&self) -> S6IoPressureFaultKind {
        self.fault_kind
    }

    pub const fn fault_evidence_class(&self) -> PhysicalFaultEvidenceClass {
        self.fault_evidence_class
    }

    pub const fn expected_status(&self) -> LatencyEnvelopeAssessmentStatus {
        self.expected_status
    }
}

impl S6IoPressureHarnessEvidence {
    pub fn from_replay_bundle(
        scenario: S6IoPressureHarnessScenario,
        replay: &SimulationReplayBundle,
    ) -> Result<Self, S6IoPressureHarnessEvidenceDenial> {
        crate::s6_io_pressure_replay::require_s6_replay_bundle(&scenario, replay)?;
        Ok(Self::from_executed_replay(
            scenario,
            replay.plan().profile(),
            *replay.replay_basis_identity().digest_bytes(),
        ))
    }

    fn from_executed_replay(
        scenario: S6IoPressureHarnessScenario,
        replay_profile: PhysicalSimulationProfile,
        replay_identity: [u8; 32],
    ) -> Self {
        let fault_phase =
            crate::s6_io_pressure_vocab::fault_phase_for_pressure_fault(scenario.fault_kind);
        Self {
            maturity: crate::s6_io_pressure_vocab::maturity_for_fault_evidence_class(
                scenario.fault_evidence_class,
            ),
            scenario,
            driver: PhysicalDriverKind::IoPressureBoundary,
            fault_phase,
            replay_profile,
            replay_identity,
        }
    }

    pub const fn scenario(&self) -> &S6IoPressureHarnessScenario {
        &self.scenario
    }

    pub const fn driver(&self) -> PhysicalDriverKind {
        self.driver
    }

    pub const fn fault_phase(&self) -> PhysicalScenarioFaultKind {
        self.fault_phase
    }

    pub const fn maturity(&self) -> S6PressureEvidenceMaturity {
        self.maturity
    }

    pub const fn replay_identity(&self) -> &[u8; 32] {
        &self.replay_identity
    }

    pub const fn replay_profile(&self) -> PhysicalSimulationProfile {
        self.replay_profile
    }

    pub fn require_real_backend_safety(
        &self,
    ) -> Result<S6RealBackendSafetyQualification, S6BackendSafetyQualificationDenial> {
        require_backend_evidence_class(self.scenario.backend_evidence_class)?;
        match self.scenario.fault_evidence_class {
            PhysicalFaultEvidenceClass::CertifiedBackend
            | PhysicalFaultEvidenceClass::ExternallyGuaranteed => {
                Ok(S6RealBackendSafetyQualification {
                    backend_profile: self.scenario.backend_profile,
                    evidence_class: self.scenario.fault_evidence_class,
                })
            }
            PhysicalFaultEvidenceClass::Simulated => {
                Err(S6BackendSafetyQualificationDenial::SimulatedBackendSuccess)
            }
            PhysicalFaultEvidenceClass::InjectedProductionBoundary => {
                Err(S6BackendSafetyQualificationDenial::InjectedBoundaryOnly)
            }
            PhysicalFaultEvidenceClass::BackendEmulated => {
                Err(S6BackendSafetyQualificationDenial::BackendEmulatedOnly)
            }
            PhysicalFaultEvidenceClass::ObservedHost => {
                Err(S6BackendSafetyQualificationDenial::ObservedHostOnly)
            }
        }
    }
}

impl S6RealBackendSafetyQualification {
    pub const fn backend_profile(&self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn evidence_class(&self) -> PhysicalFaultEvidenceClass {
        self.evidence_class
    }
}

impl S6IoPressureOracleObservation {
    pub(crate) fn from_executed_pressure(
        scenario: &S6IoPressureHarnessScenario,
        counters: crate::S6IoPressureExecutionCounters,
        envelope_status: LatencyEnvelopeAssessmentStatus,
    ) -> Self {
        Self {
            foreground_lane: scenario.foreground_lane,
            background_pressure: scenario.background_pressure,
            secure_io_posture: scenario.secure_io_posture,
            fault_kind: scenario.fault_kind,
            fault_evidence_class: scenario.fault_evidence_class,
            envelope_status,
            attribution_complete: counters.queue_depth() > 0
                && counters.interference_events() > 0
                && counters.allocation_bytes() > 0,
        }
    }

    pub const fn envelope_status(self) -> LatencyEnvelopeAssessmentStatus {
        self.envelope_status
    }

    pub const fn attribution_complete(self) -> bool {
        self.attribution_complete
    }

    pub fn matches_scenario(self, scenario: &S6IoPressureHarnessScenario) -> bool {
        self.foreground_lane == scenario.foreground_lane
            && self.background_pressure == scenario.background_pressure
            && self.secure_io_posture == scenario.secure_io_posture
            && self.fault_kind == scenario.fault_kind
            && self.fault_evidence_class == scenario.fault_evidence_class
            && self.envelope_status == scenario.expected_status
            && self.attribution_complete
    }
}

fn require_backend_evidence_class(
    actual: CapabilityEvidenceClass,
) -> Result<(), S6BackendSafetyQualificationDenial> {
    if actual == CapabilityEvidenceClass::CertifiedBackendProfile {
        Ok(())
    } else {
        Err(
            S6BackendSafetyQualificationDenial::BackendEvidenceClassTooWeak {
                required: CapabilityEvidenceClass::CertifiedBackendProfile,
                actual,
            },
        )
    }
}
