#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalCertifiedPerformanceSourceKind {
    CurrentBasisCounterBackedExecutionReceipt,
    MaterializedSupportExpansionReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalCertifiedPerformanceClass {
    HotPathOperational,
    SupportExpansionCompatibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalCertifiedPerformanceAttachmentDenial {
    HotPathCertificationRequiresCounterBackedExecution,
    HotPathCertificationRequiresExactCurrentVerifiedHotPath,
    HotPathCertificationRequiresExplicitOperationalExclusions,
    SupportCertificationRequiresSupportExpansionBoundary,
    SupportCertificationRequiresSupportRows,
}
