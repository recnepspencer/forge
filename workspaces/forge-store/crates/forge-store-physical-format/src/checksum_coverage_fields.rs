#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChecksumHeaderField {
    Magic,
    FormatVersion,
    HeaderLength,
    HeaderKind,
    Generation,
    PublicationState,
    PayloadLength,
    ReservedFields,
    ChecksumField,
    CompatibilityFields,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumFieldHandling {
    ExcludedDuringComputation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumReservedFieldPosture {
    CoveredAsZeroedAndPreserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumGenerationFieldPosture {
    CoveredAsPhysicalGeneration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumLengthFieldPosture {
    CoveredAsSerializedPayloadLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumPayloadRegion {
    SerializedPayloadBytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumPaddingPosture {
    ExcludedAndMustRemainZeroed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCompatibilityFieldPosture {
    CoveredAndDenyUnknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumUnknownFieldPosture {
    DenyUntilReadmitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCoverageEncoding {
    SerializedBytes,
    CanonicalizedFields,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCoverageDisposition {
    Covered,
    Excluded,
    Preserved,
    Skipped,
    Denied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCoverageRegion {
    HeaderField(ChecksumHeaderField),
    PayloadRegion,
    PaddingBytes,
    CompatibilityFields,
    LaterPhysicalFamily,
    UnknownFutureField,
}
