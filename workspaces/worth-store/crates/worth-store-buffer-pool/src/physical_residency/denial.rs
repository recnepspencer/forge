use super::PhysicalFrameLoadTerminal;
use super::PhysicalResidencyPressureDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalResidencyDenial {
    WrongStore,
    FrameLargerThanResidentBudget,
    MetadataBudgetExceeded,
    Pressure(PhysicalResidencyPressureDenial),
    WriteBackExceedsDirtyPosture,
    WriteBackFrameNotDirty,
    WriteBackFrameAlreadyClaimed,
    WriteBackReceiptMismatch,
    AllocationFailed,
    AllocationGrantMismatch,
    CandidatePublicationActive,
    PoolClosed,
    BoundedLoadLimitConflict {
        active_limit: u32,
        requested_limit: u32,
    },
    EmptyCandidateBatch,
    CandidateCardinalityMismatch {
        declared: usize,
        provided: usize,
    },
    DuplicateCandidateIdentity,
    CandidateCoverageConflict,
    CandidateSequenceConflict,
    CompleteArtifactRequiresOffsetZero,
    ArtifactIdentityOccupied,
    FrameIdentityOccupied,
    FrameLengthMismatch,
    FrameNotDirty,
    FrameAlreadyResident,
    FrameLoadTerminated(PhysicalFrameLoadTerminal),
    FrameNotResident,
    FramePinned,
    FrameDirty,
    IdentityAlreadyCurrent,
}
