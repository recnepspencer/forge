use worth_store_io_scheduler::{
    foreground_reservation::ForegroundIoLaneKind, BackgroundIoPressureClass,
    LatencyEnvelopeAssessmentStatus,
};
use worth_store_physical_backend::BackendTargetProfile;
use worth_store_physical_certification::{
    IoPressureFaultKind, IoPressureHarnessSecureIoPosture, PhysicalFaultEvidenceClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoPressureTestProfile {
    profile_scope: &'static str,
    backend_profile: BackendTargetProfile,
    foreground_lane: ForegroundIoLaneKind,
    background_pressure: BackgroundIoPressureClass,
    secure_io_posture: IoPressureHarnessSecureIoPosture,
    fault_kind: IoPressureFaultKind,
    fault_evidence_class: PhysicalFaultEvidenceClass,
    expected_status: LatencyEnvelopeAssessmentStatus,
}

pub const fn deterministic_io_pressure_profile() -> IoPressureTestProfile {
    IoPressureTestProfile {
        profile_scope: "s6.deterministic-miniature.io-pressure",
        backend_profile: BackendTargetProfile::PosixFileFsyncDirSync,
        foreground_lane: ForegroundIoLaneKind::PointRead,
        background_pressure: BackgroundIoPressureClass::RepairScan,
        secure_io_posture: IoPressureHarnessSecureIoPosture::ScopePreserving,
        fault_kind: IoPressureFaultKind::BackendLatencyInjection,
        fault_evidence_class: PhysicalFaultEvidenceClass::InjectedProductionBoundary,
        expected_status: LatencyEnvelopeAssessmentStatus::Held,
    }
}

pub const fn large_io_pressure_profile() -> IoPressureTestProfile {
    IoPressureTestProfile {
        profile_scope: "s6.large.io-pressure",
        backend_profile: BackendTargetProfile::PosixFileFsyncDirSync,
        foreground_lane: ForegroundIoLaneKind::CommitCriticalWalWrite,
        background_pressure: BackgroundIoPressureClass::CheckpointFlush,
        secure_io_posture: IoPressureHarnessSecureIoPosture::ScopePreserving,
        fault_kind: IoPressureFaultKind::DelayedSync,
        fault_evidence_class: PhysicalFaultEvidenceClass::InjectedProductionBoundary,
        expected_status: LatencyEnvelopeAssessmentStatus::Held,
    }
}

impl IoPressureTestProfile {
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

    pub const fn secure_io_posture(self) -> IoPressureHarnessSecureIoPosture {
        self.secure_io_posture
    }

    pub const fn fault_kind(self) -> IoPressureFaultKind {
        self.fault_kind
    }

    pub const fn fault_evidence_class(self) -> PhysicalFaultEvidenceClass {
        self.fault_evidence_class
    }

    pub const fn expected_status(self) -> LatencyEnvelopeAssessmentStatus {
        self.expected_status
    }
}
