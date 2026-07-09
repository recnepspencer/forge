use super::denial::RecoveryEvidenceDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonApplicableFoundationalSurface {
    Branch,
    Merge,
    Commit,
    ScopedMerge,
    SelectedNode,
    SelectedAspect,
    SkippedScope,
    CherryPick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAdmissionMechanism {
    WalReplay,
    SourcePrecedence,
    CheckpointCutover,
    RecoveredState,
}

pub const NON_APPLICABLE_FOUNDATIONAL_SURFACES: [NonApplicableFoundationalSurface; 8] = [
    NonApplicableFoundationalSurface::Branch,
    NonApplicableFoundationalSurface::Merge,
    NonApplicableFoundationalSurface::Commit,
    NonApplicableFoundationalSurface::ScopedMerge,
    NonApplicableFoundationalSurface::SelectedNode,
    NonApplicableFoundationalSurface::SelectedAspect,
    NonApplicableFoundationalSurface::SkippedScope,
    NonApplicableFoundationalSurface::CherryPick,
];

pub const RECOVERY_ADMISSION_MECHANISMS: [RecoveryAdmissionMechanism; 4] = [
    RecoveryAdmissionMechanism::WalReplay,
    RecoveryAdmissionMechanism::SourcePrecedence,
    RecoveryAdmissionMechanism::CheckpointCutover,
    RecoveryAdmissionMechanism::RecoveredState,
];

pub fn deny_non_applicable_surface(
    _surface: NonApplicableFoundationalSurface,
    _mechanism: RecoveryAdmissionMechanism,
) -> RecoveryEvidenceDenial {
    RecoveryEvidenceDenial::NonApplicableFoundationalSurface
}
