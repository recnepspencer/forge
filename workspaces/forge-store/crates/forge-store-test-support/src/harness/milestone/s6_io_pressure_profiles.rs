use forge_store_io_scheduler::{
    foreground_reservation::ForegroundIoLaneKind, BackgroundIoPressureClass,
    LatencyEnvelopeAssessmentStatus,
};
use forge_store_physical_backend::BackendTargetProfile;
use forge_store_physical_certification::{
    PhysicalFaultEvidenceClass, S6HarnessSecureIoPosture, S6IoPressureFaultKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6IoPressureTestProfile {
    profile_scope: &'static str,
    backend_profile: BackendTargetProfile,
    foreground_lane: ForegroundIoLaneKind,
    background_pressure: BackgroundIoPressureClass,
    secure_io_posture: S6HarnessSecureIoPosture,
    fault_kind: S6IoPressureFaultKind,
    fault_evidence_class: PhysicalFaultEvidenceClass,
    expected_status: LatencyEnvelopeAssessmentStatus,
}

pub const fn deterministic_s6_io_pressure_profile() -> S6IoPressureTestProfile {
    S6IoPressureTestProfile {
        profile_scope: "s6.deterministic-miniature.io-pressure",
        backend_profile: BackendTargetProfile::PosixFileFsyncDirSync,
        foreground_lane: ForegroundIoLaneKind::PointRead,
        background_pressure: BackgroundIoPressureClass::RepairScan,
        secure_io_posture: S6HarnessSecureIoPosture::ScopePreserving,
        fault_kind: S6IoPressureFaultKind::BackendLatencyInjection,
        fault_evidence_class: PhysicalFaultEvidenceClass::InjectedProductionBoundary,
        expected_status: LatencyEnvelopeAssessmentStatus::Held,
    }
}

pub const fn large_s6_io_pressure_profile() -> S6IoPressureTestProfile {
    S6IoPressureTestProfile {
        profile_scope: "s6.large.io-pressure",
        backend_profile: BackendTargetProfile::PosixFileFsyncDirSync,
        foreground_lane: ForegroundIoLaneKind::CommitCriticalWalWrite,
        background_pressure: BackgroundIoPressureClass::CheckpointFlush,
        secure_io_posture: S6HarnessSecureIoPosture::ScopePreserving,
        fault_kind: S6IoPressureFaultKind::DelayedSync,
        fault_evidence_class: PhysicalFaultEvidenceClass::InjectedProductionBoundary,
        expected_status: LatencyEnvelopeAssessmentStatus::Held,
    }
}

impl S6IoPressureTestProfile {
    pub const fn profile_scope(self) -> &'static str {
        self.profile_scope
    }

    pub const fn backend_profile(self) -> BackendTargetProfile {
        self.backend_profile
    }

    pub const fn foreground_lane(self) -> ForegroundIoLaneKind {
        self.foreground_lane
    }

    pub const fn background_pressure(self) -> BackgroundIoPressureClass {
        self.background_pressure
    }

    pub const fn secure_io_posture(self) -> S6HarnessSecureIoPosture {
        self.secure_io_posture
    }

    pub const fn fault_kind(self) -> S6IoPressureFaultKind {
        self.fault_kind
    }

    pub const fn fault_evidence_class(self) -> PhysicalFaultEvidenceClass {
        self.fault_evidence_class
    }

    pub const fn expected_status(self) -> LatencyEnvelopeAssessmentStatus {
        self.expected_status
    }
}
