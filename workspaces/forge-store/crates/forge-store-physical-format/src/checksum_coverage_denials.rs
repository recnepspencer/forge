use crate::ChecksumHeaderField;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumCoverageMapDenial {
    MissingCoveredHeaderFields,
    MissingExcludedHeaderFields,
    MissingChecksumFieldHandling,
    MissingMutablePublicationFields,
    MissingReservedFields,
    MissingGenerationFields,
    MissingLengthFields,
    MissingPayloadRegion,
    MissingPaddingBytes,
    MissingCompatibilityFields,
    MissingUnknownFieldPosture,
    MissingCoverageEncoding,
    MissingRequiredHeaderField(ChecksumHeaderField),
    SerializerOrderRejected,
    RustLayoutRejected,
    UnsupportedFormatVersion,
}
