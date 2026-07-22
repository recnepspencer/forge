#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalResidencyDenial {
    WrongStore,
    FrameLargerThanResidentBudget,
    ResidentBudgetExhausted,
    FrameEntryBudgetExhausted,
    MetadataBudgetExceeded,
    PinnedFrameBudgetExceeded,
    PinLeaseBudgetExceeded,
    DirtyFrameBudgetExceeded,
    OperationBudgetExceeded,
    SpeculativeFrameBudgetExceeded,
    WriteBackExceedsDirtyPosture,
    WriteBackFrameNotDirty,
    WriteBackFrameAlreadyClaimed,
    WriteBackReceiptMismatch,
    AllocationFailed,
    CandidatePublicationActive,
    PoolClosed,
    FrameLengthMismatch,
    FrameNotDirty,
    FrameAlreadyResident,
    FrameNotResident,
    FramePinned,
    FrameDirty,
    IdentityAlreadyCurrent,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PhysicalFrameLoadError<E> {
    Residency(PhysicalResidencyDenial),
    Source(E),
}
