#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIntegrityEvidenceDenial {
    AuthorityDigestDenied,
    DerivedReportIsNotRebuildable,
    EvidenceBasisMismatch,
    MissingExecutedEvidence,
    SameMaterializationPath,
}
