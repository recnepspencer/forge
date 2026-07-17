#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WriteCompletionAssumption {
    SubmissionOnly,
    BackendAccepted,
    DurabilityFenceCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PublicationAtomicityAssumption {
    RenameNotDurableWithoutDirectoryFence,
    AtomicReplacementAfterDirectoryFence,
    NoAtomicReplacementClaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TornWriteAssumption {
    TornSectorPossible,
    TornPagePossible,
    AtomicSectorOnly,
    AtomicPageGuaranteed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IoBufferingAssumption {
    BufferedWriteback,
    DirectIo,
    MemoryMappedWriteback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChecksumCoverageAssumption {
    FramePayload,
    FrameHeaderAndPayload,
    PageAndPublicationEnvelope,
}
