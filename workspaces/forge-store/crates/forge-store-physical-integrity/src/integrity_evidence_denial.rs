#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalIntegrityEvidenceDenial {
    DerivedReportIsNotRebuildable,
    EvidenceBasisMismatch,
    MissingExecutedEvidence,
    SameMaterializationPath,
}
