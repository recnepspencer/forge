use forge_store_physical_format::ChecksumCoverageMapDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgorithmMismatchDenial {
    UnknownAlgorithm,
    AlgorithmIdMismatch,
    MissingCoverageFields,
    DigestAsChecksumSubstitution,
    ChecksumAsAuthenticityClaim,
    ScopeFormatVersionMismatch,
    CoverageMapDenied(ChecksumCoverageMapDenial),
    CompatibilityReadmissionRequired,
    FoundationalEvidenceDenied,
}

impl From<ChecksumCoverageMapDenial> for ChecksumAlgorithmMismatchDenial {
    fn from(value: ChecksumCoverageMapDenial) -> Self {
        Self::CoverageMapDenied(value)
    }
}
